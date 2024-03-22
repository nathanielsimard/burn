use crate::{ops::FloatTensor, Device};

use super::Backend;

/// Allows tensors to be transfered between backends efficiently.
pub trait BackendBridge<Origin: Backend> {
    /// The target backend
    type Target: Backend;

    /// Transfer the tensor to the target backend.
    fn into_target<const D: usize>(
        tensor: FloatTensor<Origin, D>,
        device: Device<Self::Target>,
    ) -> FloatTensor<Self::Target, D>;

    /// Transfer the tensor from the target backend.
    fn from_target<const D: usize>(
        tensor: FloatTensor<Self::Target, D>,
        device: Device<Origin>,
    ) -> FloatTensor<Origin, D>;
}
