use super::{ElementWise, ElementWiseState};
use crate::{
    element::JitElement, fusion::ElementWiseBuilder, tensor::JitTensor, JitBackend, Runtime,
};
use burn_compute::client::ComputeClient;
use burn_fusion::{client::MutexFusionClient, FusionBackend};
use burn_tensor::Shape;
use core::marker::PhantomData;
use serde::{Deserialize, Serialize};

/// Fusion optimization type for WGPU.
///
/// More optimization variants should be added here.
pub enum JitOptimization<R: Runtime> {
    /// Element wise optimization.
    ElementWise(ElementWise<R>),
}

/// Fusion optimization state type for WGPU.
///
/// More optimization variants should be added here.
#[derive(Serialize, Deserialize)]
pub enum JitOptimizationState {
    /// Element wise state.
    ElementWise(ElementWiseState),
}

impl<R: Runtime> burn_fusion::Optimization<JitBackend<R>> for JitOptimization<R> {
    fn execute(&mut self, context: &mut burn_fusion::stream::Context<'_, JitBackend<R>>) {
        match self {
            Self::ElementWise(op) => op.execute(context),
        }
    }

    fn len(&self) -> usize {
        match self {
            Self::ElementWise(op) => op.len(),
        }
    }

    fn to_state(&self) -> JitOptimizationState {
        match self {
            Self::ElementWise(value) => JitOptimizationState::ElementWise(value.to_state()),
        }
    }

    fn from_state(device: &R::Device, state: JitOptimizationState) -> Self {
        match state {
            JitOptimizationState::ElementWise(state) => {
                Self::ElementWise(ElementWise::from_state(device, state))
            }
        }
    }
}

impl<R: Runtime> FusionBackend for JitBackend<R> {
    type OptimizationState = JitOptimizationState;
    type Optimization = JitOptimization<R>;
    type FusionDevice = R::Device;
    type Handle = JitFusionHandle<R>;
    type FusionClient = MutexFusionClient<Self>;

    fn optimizations(
        device: R::Device,
    ) -> Vec<Box<dyn burn_fusion::OptimizationBuilder<Self::Optimization>>> {
        vec![Box::new(ElementWiseBuilder::new(device))]
    }

    fn float_tensor<const D: usize>(
        handle: Self::Handle,
        shape: Shape<D>,
    ) -> Self::FloatTensorPrimitive<D> {
        handle.into_tensor(shape)
    }

    fn int_tensor<const D: usize>(
        handle: Self::Handle,
        shape: Shape<D>,
    ) -> Self::IntTensorPrimitive<D> {
        handle.into_tensor(shape)
    }

    fn bool_tensor<const D: usize>(
        handle: Self::Handle,
        shape: Shape<D>,
    ) -> Self::BoolTensorPrimitive<D> {
        handle.into_tensor(shape)
    }

    fn float_tensor_handle<const D: usize>(tensor: Self::FloatTensorPrimitive<D>) -> Self::Handle {
        tensor.into()
    }

    fn int_tensor_handle<const D: usize>(tensor: Self::IntTensorPrimitive<D>) -> Self::Handle {
        tensor.into()
    }

    fn bool_tensor_handle<const D: usize>(tensor: Self::BoolTensorPrimitive<D>) -> Self::Handle {
        tensor.into()
    }
}

pub fn strides_dyn_rank(shape: &[usize]) -> Vec<usize> {
    let mut strides = vec![0; shape.len()];

    let mut current = 1;
    shape.iter().enumerate().rev().for_each(|(index, val)| {
        strides[index] = current;
        current *= val;
    });

    strides
}

/// Handle to be used when fusing operations.
pub struct JitFusionHandle<R: Runtime> {
    /// Compute client for wgpu.
    pub client: ComputeClient<R::Server, R::Channel>,
    /// The buffer where the data are stored.
    pub handle: burn_compute::server::Handle<R::Server>,
    /// The device of the current tensor.
    pub device: R::Device,
    pub(crate) strides: Vec<usize>,
}

impl<R: Runtime> core::fmt::Debug for JitFusionHandle<R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "JitFusionHandle {{ device: {:?}, runtime: {}}}",
            self.device,
            R::name(),
        ))
    }
}

impl<R: Runtime> Clone for JitFusionHandle<R> {
    fn clone(&self) -> Self {
        Self {
            client: self.client.clone(),
            handle: self.handle.clone(),
            device: self.device.clone(),
            strides: self.strides.clone(),
        }
    }
}

unsafe impl<R: Runtime> Send for JitFusionHandle<R> {}
unsafe impl<R: Runtime> Sync for JitFusionHandle<R> {}

impl<R: Runtime> JitFusionHandle<R> {
    pub(crate) fn into_tensor<const D: usize, E: JitElement>(
        self,
        shape: Shape<D>,
    ) -> JitTensor<R, E, D> {
        JitTensor {
            client: self.client,
            handle: self.handle,
            device: self.device,
            shape,
            strides: self.strides.try_into().expect("Wrong dimension"),
            elem: PhantomData,
        }
    }
}

impl<R: Runtime, E: JitElement, const D: usize> From<JitTensor<R, E, D>> for JitFusionHandle<R> {
    fn from(value: JitTensor<R, E, D>) -> Self {
        Self {
            client: value.client,
            handle: value.handle,
            device: value.device,
            strides: value.strides.into(),
        }
    }
}

#[cfg(feature = "export_tests")]
#[allow(missing_docs)]
#[macro_export]
macro_rules! testgen_jit_fusion {
    () => {
        use super::*;

        pub type TestBackend = burn_fusion::Fusion<JitBackend<TestRuntime>>;
        pub type ReferenceBackend = burn_ndarray::NdArray<f32>;

        pub type TestTensor<const D: usize> = burn_tensor::Tensor<TestBackend, D>;
        pub type TestTensorInt<const D: usize> =
            burn_tensor::Tensor<TestBackend, D, burn_tensor::Int>;
        pub type TestTensorBool<const D: usize> =
            burn_tensor::Tensor<TestBackend, D, burn_tensor::Bool>;

        pub type ReferenceTensor<const D: usize> = burn_tensor::Tensor<ReferenceBackend, D>;

        burn_tensor::testgen_all!();
        burn_autodiff::testgen_all!();
    };
}
