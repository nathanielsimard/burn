use super::unary_ops_wrapper;
use crate::graph::ops::{UnaryOps, UnaryOpsNodeState};
use crate::tensor::ADTensor;
use crate::ADBackendDecorator;
use burn_tensor::backend::Backend;
use burn_tensor::ops::*;

#[derive(new, Debug)]
struct EmbeddingBackward<B: Backend> {
    indexes: <B::IntegerBackend as Backend>::TensorPrimitive<2>,
}

impl<B: Backend> UnaryOps<B::TensorPrimitive<2>, B::TensorPrimitive<3>> for EmbeddingBackward<B> {
    fn partial(
        &self,
        state: &UnaryOpsNodeState<B::TensorPrimitive<2>, B::TensorPrimitive<3>>,
    ) -> B::TensorPrimitive<2> {
        B::embedding_backward(&state.input.value, &state.output.grad(), &self.indexes)
    }
}

impl<B: Backend> ModuleOps<ADBackendDecorator<B>> for ADBackendDecorator<B> {
    fn embedding(
        weights: &<ADBackendDecorator<B> as Backend>::TensorPrimitive<2>,
        indexes: &<<ADBackendDecorator<B> as Backend>::IntegerBackend as Backend>::TensorPrimitive<
            2,
        >,
    ) -> <ADBackendDecorator<B> as Backend>::TensorPrimitive<3> {
        let input = weights.node.clone();
        let output = B::embedding(weights.tensor_ref(), indexes);
        let ops = EmbeddingBackward::<B>::new(indexes.clone());

        unary_ops_wrapper(input, output, ops)
    }

    fn embedding_backward(
        weights: &<ADBackendDecorator<B> as Backend>::TensorPrimitive<2>,
        output: &<ADBackendDecorator<B> as Backend>::TensorPrimitive<3>,
        indexes: &<<ADBackendDecorator<B> as Backend>::IntegerBackend as Backend>::TensorPrimitive<
            2,
        >,
    ) -> <ADBackendDecorator<B> as Backend>::TensorPrimitive<2> {
        let tensor = B::embedding_backward(weights.tensor_ref(), output.tensor_ref(), indexes);
        ADTensor::from_tensor(tensor)
    }

    fn conv1d(
        x: &<ADBackendDecorator<B> as Backend>::TensorPrimitive<3>,
        weight: &<ADBackendDecorator<B> as Backend>::TensorPrimitive<3>,
        bias: &Option<<ADBackendDecorator<B> as Backend>::TensorPrimitive<1>>,
        stride: usize,
        padding: usize,
    ) -> <ADBackendDecorator<B> as Backend>::TensorPrimitive<3> {
        todo!()
    }

    fn conv2d(
        x: &<ADBackendDecorator<B> as Backend>::TensorPrimitive<4>,
        weight: &<ADBackendDecorator<B> as Backend>::TensorPrimitive<4>,
        bias: &Option<<ADBackendDecorator<B> as Backend>::TensorPrimitive<1>>,
        stride: [usize; 2],
        padding: [usize; 2],
    ) -> <ADBackendDecorator<B> as Backend>::TensorPrimitive<4> {
        todo!()
    }
}
