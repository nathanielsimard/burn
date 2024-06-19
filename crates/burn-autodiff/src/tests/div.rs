#[burn_tensor_testgen::testgen(ad_div)]
mod tests {
    use super::*;
    use burn_tensor::{backend::Backend, TensorData};

    #[test]
    fn should_diff_div() {
        let data_1 = TensorData::from([1.0, 7.0]);
        let data_2 = TensorData::from([4.0, 7.0]);

        let device = Default::default();
        let tensor_1 = TestAutodiffTensor::from_data(data_1, &device).require_grad();
        let tensor_2 = TestAutodiffTensor::from_data(data_2, &device).require_grad();

        let tensor_3 = tensor_1.clone().div(tensor_2.clone());
        let grads = tensor_3.backward();

        let grad_1 = tensor_1.grad(&grads).unwrap();
        let grad_2 = tensor_2.grad(&grads).unwrap();

        let expected = TensorData::from([0.25, 0.1429])
            .convert::<<TestAutodiffBackend as Backend>::FloatElem>();
        grad_1.to_data().assert_approx_eq(&expected, 3);

        let expected = TensorData::from([-0.0625, -0.1429])
            .convert::<<TestAutodiffBackend as Backend>::FloatElem>();
        grad_2.to_data().assert_approx_eq(&expected, 3);
    }

    #[test]
    fn should_diff_div_scalar() {
        let data = TensorData::from([1.0, 7.0]);

        let tensor = TestAutodiffTensor::from_data(data, &Default::default()).require_grad();
        let tensor_out = tensor.clone().div_scalar(4.0);

        let grads = tensor_out.backward();
        let grad = tensor.grad(&grads).unwrap();

        let expected =
            TensorData::from([0.25, 0.25]).convert::<<TestAutodiffBackend as Backend>::FloatElem>();
        grad.to_data().assert_eq(&expected, true);
    }

    #[test]
    fn test_div_complex_1() {
        let data_1 = TensorData::from([[1.0, 7.0], [13.0, -3.0]]);
        let data_2 = TensorData::from([[4.0, 7.0], [2.0, 3.0]]);
        let data_3 = TensorData::from([[2.0, 2.0], [2.0, 2.0]]);

        let device = Default::default();
        let tensor_1 = TestAutodiffTensor::from_data(data_1, &device).require_grad();
        let tensor_2 = TestAutodiffTensor::from_data(data_2, &device).require_grad();
        let tensor_3 = TestAutodiffTensor::from_data(data_3, &device).require_grad();

        let tensor_4 = tensor_1.clone().div(tensor_2.clone());
        let tensor_5 = tensor_4.div(tensor_3);

        let grads = tensor_5.backward();

        let grad_1 = tensor_1.grad(&grads).unwrap();
        let grad_2 = tensor_2.grad(&grads).unwrap();

        let expected = TensorData::from([[0.1250, 0.0714], [0.25, 0.1667]])
            .convert::<<TestAutodiffBackend as Backend>::FloatElem>();
        grad_1.to_data().assert_approx_eq(&expected, 3);

        let expected = TensorData::from([[-0.0312, -0.0714], [-1.6250, 0.1667]])
            .convert::<<TestAutodiffBackend as Backend>::FloatElem>();
        grad_2.to_data().assert_approx_eq(&expected, 3);
    }

    #[test]
    fn test_div_complex_2() {
        let data_1 = TensorData::from([[0.0, 1.0], [3.0, 4.0]]);
        let data_2 = TensorData::from([[6.0, 7.0], [9.0, 10.0]]);

        let device = Default::default();
        let tensor_1 = TestAutodiffTensor::from_data(data_1, &device).require_grad();
        let tensor_2 = TestAutodiffTensor::from_data(data_2, &device).require_grad();

        let tensor_3 = tensor_1.clone().matmul(tensor_2.clone());
        let tensor_4 = tensor_3.div(tensor_2.clone());

        let grads = tensor_4.backward();
        let grad_1 = tensor_1.grad(&grads).unwrap();
        let grad_2 = tensor_2.grad(&grads).unwrap();

        let expected = TensorData::from([[2.00, 2.9286], [1.3667, 2.0]])
            .convert::<<TestAutodiffBackend as Backend>::FloatElem>();
        grad_1.to_data().assert_approx_eq(&expected, 3);

        let expected = TensorData::from([[0.0833, 0.0959], [-0.0556, -0.0671]])
            .convert::<<TestAutodiffBackend as Backend>::FloatElem>();
        grad_2.to_data().assert_approx_eq(&expected, 3);
    }
}
