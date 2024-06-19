#[burn_tensor_testgen::testgen(softmax)]
mod tests {
    use super::*;
    use burn_tensor::{activation, backend::Backend, Tensor, TensorData};

    #[test]
    fn test_softmax_d2() {
        let tensor = TestTensor::from([[1.0, 7.0], [13.0, -3.0]]);

        let output = activation::softmax(tensor, 1);
        let expected = TensorData::from([[2.47e-03, 9.975e-01], [1.0, 1.1254e-07]])
            .convert::<<TestBackend as Backend>::FloatElem>();

        output.into_data().assert_approx_eq(&expected, 4);
    }
}
