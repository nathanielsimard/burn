#[burn_tensor_testgen::testgen(ad_sin)]
mod tests {
    use super::*;
    use burn_tensor::{backend::Backend, TensorData};

    #[test]
    fn should_diff_sin() {
        let data_1 = TensorData::from([[0.0, 1.0], [3.0, 4.0]]);
        let data_2 = TensorData::from([[6.0, 7.0], [9.0, 10.0]]);

        let device = Default::default();
        let tensor_1 = TestAutodiffTensor::from_data(data_1, &device).require_grad();
        let tensor_2 = TestAutodiffTensor::from_data(data_2, &device).require_grad();

        let tensor_3 = tensor_1.clone().matmul(tensor_2.clone().sin());
        let tensor_4 = tensor_3.matmul(tensor_2.clone());
        let grads = tensor_4.backward();

        let grad_1 = tensor_1.grad(&grads).unwrap();
        let grad_2 = tensor_2.grad(&grads).unwrap();

        let expected = TensorData::from([[8.8500, -4.9790], [8.8500, -4.9790]])
            .convert::<<TestAutodiffBackend as Backend>::FloatElem>();
        grad_1.to_data().assert_approx_eq_diff(&expected, 2.6e-3);

        let expected = TensorData::from([[38.668987, 44.194775], [-59.97261, -80.46094]])
            .convert::<<TestAutodiffBackend as Backend>::FloatElem>();
        grad_2.to_data().assert_approx_eq_diff(&expected, 2.6e-3);
    }
}
