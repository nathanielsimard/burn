#[burn_tensor_testgen::testgen(cartesian_grid)]
mod tests {
    use super::*;
    use burn_tensor::backend::Backend;
    use burn_tensor::{Int, Shape, Tensor, TensorData};

    #[test]
    fn test_cartesian_grid() {
        let device = <TestBackend as Backend>::Device::default();

        // Test a single element tensor
        let tensor: Tensor<TestBackend, 2, Int> =
            Tensor::<TestBackend, 1, Int>::cartesian_grid([1], &device);
        let expected = TensorData::from([[0]]).convert::<<TestBackend as Backend>::IntElem>();

        tensor.into_data().assert_eq(&expected, true);

        // Test for a 2x2 tensor
        let tensor: Tensor<TestBackend, 3, Int> =
            Tensor::<TestBackend, 2, Int>::cartesian_grid([2, 2], &device);
        let expected = TensorData::from([[[0, 0], [0, 1]], [[1, 0], [1, 1]]])
            .convert::<<TestBackend as Backend>::IntElem>();

        tensor.into_data().assert_eq(&expected, true);
    }
}
