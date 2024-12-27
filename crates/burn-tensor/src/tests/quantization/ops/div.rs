#[burn_tensor_testgen::testgen(q_div)]
mod tests {
    use super::*;
    use burn_tensor::TensorData;

    #[test]
    fn should_support_div_ops() {
        let tensor_1 = QTensor::<TestBackend, 2>::int8([[0.0, 1.0, 2.0], [3.0, 4.0, 5.0]]);
        let tensor_2 = QTensor::<TestBackend, 2>::int8([[1.0, 1.0, 2.0], [3.0, 4.0, 5.0]]);

        let output = tensor_1 / tensor_2;
        let expected = TensorData::from([[0.0, 1.0, 1.0], [1.0, 1.0, 1.0]]);

        // Precision 1 to approximate de/quantization errors
        output
            .dequantize()
            .into_data()
            .assert_approx_eq(&expected, 1);
    }

    #[test]
    fn test_div_broadcast() {
        let tensor_1 = QTensor::<TestBackend, 2>::int8([[0.0, 1.0, 2.0]]);
        let tensor_2 = QTensor::<TestBackend, 2>::int8([[1.0, 1.0, 2.0], [3.0, 4.0, 5.0]]);

        let output = tensor_1 / tensor_2;

        // Precision 1 to approximate de/quantization errors
        output
            .dequantize()
            .into_data()
            .assert_approx_eq(&TensorData::from([[0.0, 1.0, 1.0], [0.0, 0.25, 0.4]]), 1);
    }

    #[test]
    fn should_support_div_scalar_ops() {
        let scalar = 2.0;
        let tensor = QTensor::<TestBackend, 2>::int8([[0.0, 1.0, 2.0], [3.0, 4.0, 5.0]]);

        let output = tensor / scalar;

        // Precision 1 to approximate de/quantization errors
        output
            .dequantize()
            .into_data()
            .assert_approx_eq(&TensorData::from([[0.0, 0.5, 1.0], [1.5, 2.0, 2.5]]), 1);
    }
}
