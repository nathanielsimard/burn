use super::ADTensor;
use crate::graph::grad::Gradients;
use crate::tensor::{
    backend::{ADBackend, Backend},
    Element,
};
use crate::tensor::{Data, Distribution, Shape};
use rand::distributions::Standard;

macro_rules! define_impl {
    (
        name: $name:ident,
        backend: $backend:ty
    ) => {
        #[derive(Clone, Copy, Debug, Default)]
        pub struct $name<E> {
            _b: $backend,
        }

        impl<E: Element> Backend for $name<E>
        where
            Standard: rand::distributions::Distribution<E>,
        {
            type Device = <$backend as Backend>::Device;
            type Elem = E;
            type Tensor<const D: usize> = ADTensor<D, $backend>;

            fn from_data<const D: usize>(
                data: Data<Self::Elem, D>,
                device: Self::Device,
            ) -> Self::Tensor<D> {
                let tensor = <$backend as Backend>::from_data(data, device);
                ADTensor::from_tensor(tensor)
            }

            fn random<const D: usize>(
                shape: Shape<D>,
                distribution: Distribution<Self::Elem>,
                device: Self::Device,
            ) -> Self::Tensor<D> {
                Self::from_data(Data::random(shape, distribution), device)
            }

            fn ad_enabled() -> bool {
                true
            }

            fn zeros<const D: usize>(shape: Shape<D>, device: Self::Device) -> Self::Tensor<D> {
                Self::from_data(Data::zeros(shape), device)
            }

            fn ones<const D: usize>(shape: Shape<D>, device: Self::Device) -> Self::Tensor<D> {
                Self::from_data(Data::ones(shape), device)
            }

            fn name() -> String {
                format!("autodiff<{}>", <$backend as Backend>::name())
            }
        }

        impl<E: Element> ADBackend for $name<E>
        where
            Standard: rand::distributions::Distribution<E>,
        {
            type InnerBackend = $backend;

            fn backward<const D: usize>(tensor: &Self::Tensor<D>) -> Gradients {
                tensor.backward()
            }
            fn grad<const D: usize>(
                tensor: &Self::Tensor<D>,
                grads: &Gradients,
            ) -> Option<<$backend as Backend>::Tensor<D>> {
                grads.wrt(tensor).map(|grad| grad.clone())
            }
        }
    };
}

#[cfg(feature = "ndarray")]
define_impl!(
    name: ADBackendNdArray,
    backend: crate::tensor::backend::ndarray::NdArrayBackend<E>
);
#[cfg(feature = "tch")]
define_impl!(
    name: ADBackendTch,
    backend: crate::tensor::backend::tch::TchBackend<E>
);
