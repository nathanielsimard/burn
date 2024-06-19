#[burn_tensor_testgen::testgen(tanh_activation)]
mod tests {
    use super::*;
    use burn_tensor::{activation, backend::Backend, Tensor, TensorData};

    #[test]
    fn test_tanh() {
        let tensor = TestTensor::from([[1., 2.], [3., 4.]]);

        let output = activation::tanh(tensor);
        let expected = TensorData::from([[0.7616, 0.9640], [0.9951, 0.9993]])
            .convert::<<TestBackend as Backend>::FloatElem>();

        output.into_data().assert_approx_eq(&expected, 4);
    }
}
