use super::memory_management::{GraphId, GraphsMemoryManagement};
use crate::{
    checkpoint::{base::Checkpointer, builder::CheckpointerBuilder},
    grads::Gradients,
    graph::{traversal::BreadthFirstSearch, StepBoxed},
    tensor::{AutodiffTensor, NodeRefCount},
    NodeID,
};
use burn_tensor::backend::Backend;
use std::collections::HashMap;

#[derive(Default)]
pub struct AutodiffServer {
    steps: HashMap<NodeID, StepBoxed>,
    actions_builder: HashMap<NodeID, CheckpointerBuilder>,
    memory_management: GraphsMemoryManagement,
}

impl AutodiffServer {
    pub fn register(&mut self, rc: NodeRefCount, step: StepBoxed, actions: CheckpointerBuilder) {
        let parents = step.parents();
        let node_id = *rc.as_ref();

        self.memory_management.register(rc, parents);

        self.steps.insert(node_id, step);
        self.actions_builder.insert(node_id, actions);
    }

    pub fn backward<B: Backend, const D: usize>(
        &mut self,
        root: AutodiffTensor<B, D>,
    ) -> Gradients {
        let step = self.steps.remove(&root.node.id).expect(
            "Root node should have a step registered, did you forget to call \
             `Tensor::register_grad` on the tensor where you need gradients?",
        );
        let builder = self.actions_builder.remove(&root.node.id).unwrap();

        let grads = Gradients::new::<B, D>(root.node.clone(), root.primitive);
        let (tape, builder) = self.build_tape(root.node.id.clone(), step, builder);
        let checkpointer = builder.build(&self.steps);

        let gradients = Self::execute_steps(tape, grads, checkpointer);

        // Cleanup
        let mut on_free_graph = |node_id: &NodeID| {
            self.steps.remove(node_id);
            self.actions_builder.remove(node_id);
        };

        for graph_id in self.memory_management.find_orphan_graphs() {
            self.memory_management
                .free_graph(graph_id, &mut on_free_graph);
        }

        gradients
    }

    fn build_tape(
        &mut self,
        root: NodeID,
        root_step: StepBoxed,
        mut builder: CheckpointerBuilder,
    ) -> (Vec<Vec<StepBoxed>>, CheckpointerBuilder) {
        let mut tape = (0..root_step.order())
            .map(|_| Vec::with_capacity(1))
            .collect::<Vec<_>>();

        BreadthFirstSearch.traverse(root, root_step, &mut self.steps, |id, step| {
            let order = step.order();
            if order == 0 {
                return;
            }

            if let Some(steps) = tape.get_mut(order - 1) {
                steps.push(step);
            }

            if let Some(node_builder) = self.actions_builder.remove(&id) {
                builder.extend(node_builder);
            }
        });

        (tape, builder)
    }

    fn execute_steps(
        tape: Vec<Vec<StepBoxed>>,
        mut grads: Gradients,
        mut checkpointer: Checkpointer,
    ) -> Gradients {
        tape.into_iter().rev().for_each(|steps| {
            steps
                .into_iter()
                .for_each(|step| step.step(&mut grads, &mut checkpointer))
        });

        #[cfg(feature = "export_tests")]
        // For checkpointing tests
        assert!(checkpointer.is_empty());
        grads
    }
}
