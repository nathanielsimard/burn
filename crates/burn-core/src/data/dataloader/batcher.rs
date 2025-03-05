use alloc::{vec, vec::Vec};
use burn_tensor::backend::Backend;

#[cfg(test)]
use crate::TestBackend;

/// A trait for batching items of type `I` into items of type `O`.
pub trait Batcher<B: Backend, I, O>: Send {
    /// Batches the given items on the first device.
    ///
    /// # Arguments
    ///
    /// * `items` - The items to batch.
    ///
    /// # Returns
    ///
    /// The batched items.
    fn batch(&self, items: Vec<I>) -> O {
        self.batch_with_device(items, &self.devices()[0])
    }

    /// Batches the given items on the specified device.
    ///
    /// # Arguments
    ///
    /// * `items` - The items to batch.
    /// * `device` - The backend device to use.
    ///
    /// # Returns
    ///
    /// The batched items.
    fn batch_with_device(&self, items: Vec<I>, device: &B::Device) -> O;

    /// Returns the devices to use for the batcher.
    ///
    /// By default, the batcher will use the backend's default device.
    ///
    /// # Notes
    ///
    fn devices(&self) -> Vec<B::Device> {
        vec![Default::default()]
    }
}

/// A super trait for [batcher](Batcher) that allows it to be cloned dynamically.
///
/// Any batcher that implements [Clone] should also implement this automatically.
pub trait DynBatcher<B: Backend, I, O>: Send + Batcher<B, I, O> {
    /// Clone the batcher and returns a new one.
    fn clone_dyn(&self) -> Box<dyn DynBatcher<B, I, O>>;
}

impl<Ba, B, I, O> DynBatcher<B, I, O> for Ba
where
    Ba: Batcher<B, I, O> + Clone + 'static,
    B: Backend,
{
    fn clone_dyn(&self) -> Box<dyn DynBatcher<B, I, O>> {
        Box::new(self.clone())
    }
}

/// Test batcher
#[cfg(test)]
#[derive(new, Clone)]
pub struct TestBatcher;

#[cfg(test)]
impl<I> Batcher<TestBackend, I, Vec<I>> for TestBatcher {
    fn batch(&self, items: Vec<I>) -> Vec<I> {
        items
    }

    fn batch_with_device(
        &self,
        items: Vec<I>,
        _device: &<TestBackend as Backend>::Device,
    ) -> Vec<I> {
        items
    }
}
