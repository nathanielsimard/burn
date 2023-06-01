use burn_tensor::ops::ActivationOps;

use crate::{
    element::{FloatElement, IntElement},
    kernel::{unary, unary_inplace},
    unary, unary_inplace, GraphicsAPI, WGPUBackend,
};

use super::FloatTensor;

impl<G, F, I> ActivationOps<WGPUBackend<G, F, I>> for WGPUBackend<G, F, I>
where
    G: GraphicsAPI + 'static,
    F: FloatElement,
    I: IntElement,
{
    fn relu<const D: usize>(tensor: FloatTensor<Self, D>) -> FloatTensor<Self, D> {
        unary!(Relu, body "output[global_id.x] = max(lhs[global_id.x], 0.0);");
        unary_inplace!(ReluInplace, body "lhs[global_id.x] = max(lhs[global_id.x], 0.0);");

        if tensor.can_mut() {
            return unary_inplace::<ReluInplace, F, D>(tensor);
        }

        unary::<Relu, F, D>(tensor)
    }
}
