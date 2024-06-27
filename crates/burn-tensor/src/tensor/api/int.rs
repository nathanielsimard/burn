use crate::{backend::Backend, Float, Int, Shape, Tensor, TensorData};

use core::ops::Range;

#[cfg(all(not(feature = "wasm-sync"), target_family = "wasm"))]
use crate::{argsort, check, check::TensorCheck, sort, sort_with_indices};

impl<B> Tensor<B, 1, Int>
where
    B: Backend,
{
    /// Returns a new integer tensor on the specified device.
    ///
    /// # Arguments
    ///
    /// * `range` - The range of values to generate.
    /// * `device` - The device to create the tensor on.
    pub fn arange(range: Range<i64>, device: &B::Device) -> Self {
        Tensor::new(B::int_arange(range, device))
    }

    /// Returns a new integer tensor on the specified device.
    ///
    /// # Arguments
    ///
    /// * `range` - The range of values to generate.
    /// * `step` - The step between each value.
    pub fn arange_step(range: Range<i64>, step: usize, device: &B::Device) -> Self {
        Tensor::new(B::int_arange_step(range, step, device))
    }
}

impl<const D: usize, B> Tensor<B, D, Int>
where
    B: Backend,
{
    /// Create a tensor from integers (i32), placing it on a given device.
    ///
    /// # Example
    ///
    /// ```rust
    /// use burn_tensor::backend::Backend;
    /// use burn_tensor::{Tensor, Int};
    ///
    /// fn example<B: Backend>() {
    ///     let device = B::Device::default();
    ///     let _x: Tensor<B, 1, Int> = Tensor::from_ints([1, 2], &device);
    ///     let _y: Tensor<B, 2, Int> = Tensor::from_ints([[1, 2], [3, 4]], &device);
    /// }
    /// ```
    pub fn from_ints<A: Into<TensorData>>(ints: A, device: &B::Device) -> Self {
        Self::from_data(ints.into().convert::<i32>(), device)
    }

    /// Returns a new tensor with the same shape and device as the current tensor and the data
    /// casted to Float.
    ///
    /// # Example
    ///
    /// ```rust
    /// use burn_tensor::backend::Backend;
    /// use burn_tensor::{Int, Tensor};
    ///
    /// fn example<B: Backend>() {
    ///     let device = Default::default();
    ///     let int_tensor = Tensor::<B, 1, Int>::arange(0..5, &device);
    ///     let float_tensor = int_tensor.float();
    /// }
    /// ```
    pub fn float(self) -> Tensor<B, D, Float> {
        Tensor::new(B::int_into_float(self.primitive))
    }

    /// Generates a cartesian grid for the given tensor shape on the specified device.
    /// The generated tensor is of dimension `D2 = D + 1`, where each element at dimension D contains the cartesian grid coordinates for that element.
    ///
    /// # Arguments
    ///
    /// * `shape` - The shape specifying the dimensions of the tensor.
    /// * `device` - The device to create the tensor on.
    ///
    /// # Panics
    ///
    /// Panics if `D2` is not equal to `D+1`.
    ///
    /// # Examples
    ///
    /// ```rust
    ///    use burn_tensor::Int;
    ///    use burn_tensor::{backend::Backend, Shape, Tensor};
    ///    fn example<B: Backend>() {
    ///        let device = Default::default();
    ///        let result: Tensor<B, 3, _> = Tensor::<B, 2, Int>::cartesian_grid([2, 3], &device);
    ///        println!("{}", result);
    ///    }
    /// ```
    pub fn cartesian_grid<S: Into<Shape<D>>, const D2: usize>(
        shape: S,
        device: &B::Device,
    ) -> Tensor<B, D2, Int> {
        Tensor::new(B::int_cartesian_grid::<S, D, D2>(shape, device))
    }

    /// Sort the elements by value in ascending order along a given dimension.
    ///
    /// This sort is unstable (i.e., may reorder equal elements).
    #[cfg(all(not(feature = "wasm-sync"), target_family = "wasm"))]
    pub async fn sort(self, dim: usize) -> Tensor<B, D, Int> {
        Tensor::new(sort::<B, D, Int>(self.primitive, dim, /* descending */ false).await)
    }

    /// Sort the elements by value in descending order along a given dimension.
    ///
    /// This sort is unstable (i.e., may reorder equal elements).
    #[cfg(all(not(feature = "wasm-sync"), target_family = "wasm"))]
    pub async fn sort_descending(self, dim: usize) -> Tensor<B, D, Int> {
        Tensor::new(sort::<B, D, Int>(self.primitive, dim, /* descending */ true).await)
    }

    /// Sort the elements by value in ascending order along a given dimension.
    /// Also returns the indices.
    ///
    /// This sort is unstable (i.e., may reorder equal elements).
    #[cfg(all(not(feature = "wasm-sync"), target_family = "wasm"))]
    pub async fn sort_with_indices(self, dim: usize) -> (Tensor<B, D, Int>, Tensor<B, D, Int>) {
        check!(TensorCheck::sort_dim::<D>("Sort_with_indices", dim));
        let (values, indices) =
            sort_with_indices::<B, D, Int>(self.primitive, dim, /*descending*/ false).await;
        (Tensor::new(values), Tensor::new(indices))
    }

    /// Sort the elements by value in descending order along a given dimension.
    /// Also returns the indices.
    ///
    /// This sort is unstable (i.e., may reorder equal elements).
    #[cfg(all(not(feature = "wasm-sync"), target_family = "wasm"))]
    pub async fn sort_descending_with_indices(
        self,
        dim: usize,
    ) -> (Tensor<B, D, Int>, Tensor<B, D, Int>) {
        check!(TensorCheck::sort_dim::<D>("Sort_with_indices", dim));
        let (values, indices) =
            sort_with_indices::<B, D, Int>(self.primitive, dim, /*descending*/ true).await;
        (Tensor::new(values), Tensor::new(indices))
    }

    /// Returns the indices that sort the elements by value in ascending order along a given dimension.
    ///
    /// This sort is unstable (i.e., may reorder equal elements).
    #[cfg(all(not(feature = "wasm-sync"), target_family = "wasm"))]
    pub async fn argsort(self, dim: usize) -> Tensor<B, D, Int> {
        check!(TensorCheck::sort_dim::<D>("Argsort", dim));
        Tensor::new(argsort::<B, D, Int>(self.primitive, dim, /*descending*/ false).await)
    }

    /// Returns the indices that sort the elements by value in descending order along a given dimension.
    ///
    /// This sort is unstable (i.e., may reorder equal elements).
    #[cfg(all(not(feature = "wasm-sync"), target_family = "wasm"))]
    pub async fn argsort_descending(self, dim: usize) -> Tensor<B, D, Int> {
        check!(TensorCheck::sort_dim::<D>("Argsort", dim));
        Tensor::new(argsort::<B, D, Int>(self.primitive, dim, /*descending*/ true).await)
    }

    /// Returns the `k` largest elements of the given input tensor along a given dimension.
    #[cfg(all(not(feature = "wasm-sync"), target_family = "wasm"))]
    pub async fn topk(self, k: usize, dim: usize) -> Tensor<B, D, Int> {
        let k_indices = Tensor::arange(0..k as i64, &self.device());
        self.sort_descending(dim).await.select(dim, k_indices)
    }

    /// Returns the `k` largest elements of the given input tensor along a given dimension.
    /// Also returns the indices.
    #[cfg(all(not(feature = "wasm-sync"), target_family = "wasm"))]
    pub async fn topk_with_indices(
        self,
        k: usize,
        dim: usize,
    ) -> (Tensor<B, D, Int>, Tensor<B, D, Int>) {
        let k_indices = Tensor::arange(0..k as i64, &self.device());
        let (values, indices) = self.sort_descending_with_indices(dim).await;
        (
            values.select(dim, k_indices.clone()),
            indices.select(dim, k_indices),
        )
    }
}
