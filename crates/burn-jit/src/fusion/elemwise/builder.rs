use burn_fusion::OptimizationBuilder;

use crate::{
    fusion::{
        on_write::{builder::FuseOnWriteBuilder, ir::ElemwisePrecision},
        JitOptimization,
    },
    JitRuntime,
};

use super::optimization::ElemwiseOptimization;

/// Fused element wise operations that are normally memory bound.
pub(crate) struct ElementWiseBuilder<R: JitRuntime> {
    builder: FuseOnWriteBuilder,
    device: R::Device,
}

impl<R: JitRuntime> ElementWiseBuilder<R> {
    pub fn new(device: R::Device, bool_precision: ElemwisePrecision) -> Self {
        let client = R::client(&device);
        let props = client.properties();
        let max_bindings = props.hardware_properties().max_bindings;

        Self {
            builder: FuseOnWriteBuilder::new(max_bindings, bool_precision),
            device,
        }
    }
}

impl<R: JitRuntime> OptimizationBuilder<JitOptimization<R>> for ElementWiseBuilder<R> {
    fn register(&mut self, operation: &burn_tensor::repr::OperationDescription) {
        println!("op {operation:?}");
        self.builder.register(operation)
    }

    fn build(&self) -> JitOptimization<R> {
        println!("Build");
        let client = R::client(&self.device);
        let trace = self.builder.build();
        let elementwise =
            ElemwiseOptimization::<R>::new(trace, client, self.device.clone(), self.len());

        JitOptimization::ElementWise(elementwise)
    }

    fn reset(&mut self) {
        self.builder.reset()
    }

    fn status(&self) -> burn_fusion::OptimizationStatus {
        let state = self.builder.status();
        println!("{state:?}");
        state
    }

    fn properties(&self) -> burn_fusion::OptimizationProperties {
        let mut props = self.builder.properties();
        props.ready = props.ready && self.builder.num_ops > self.builder.num_reshapes;
        props
    }

    fn len(&self) -> usize {
        self.builder.len()
    }
}
