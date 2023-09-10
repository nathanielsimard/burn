#[burn_tensor_testgen::testgen(cat)]
mod tests {
    use super::*;
    use burn_tensor::{Bool, Data, Int, Tensor};

    #[test]
    fn should_support_cat_ops_2d_dim0() {
        let tensor_1 = TestTensor::from_data([[1.0, 2.0, 3.0]]);
        let tensor_2 = TestTensor::from_data([[4.0, 5.0, 6.0]]);

        let data_actual = TestTensor::cat(vec![tensor_1, tensor_2], 0).into_data();

        let data_expected = Data::from([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]]);
        data_expected.assert_approx_eq(&data_actual, 3);
    }

    #[test]
    fn should_support_cat_ops_int() {
        let tensor_1 = Tensor::<TestBackend, 2, Int>::from_data([[1, 2, 3]]);
        let tensor_2 = Tensor::<TestBackend, 2, Int>::from_data([[4, 5, 6]]);

        let data_actual = Tensor::cat(vec![tensor_1, tensor_2], 0).into_data();

        let data_expected = Data::from([[1, 2, 3], [4, 5, 6]]);
        assert_eq!(&data_actual, &data_expected);
    }

    #[test]
    fn should_support_cat_ops_bool() {
        let tensor_1 = Tensor::<TestBackend, 2, Bool>::from_data([[false, true, true]]);
        let tensor_2 = Tensor::<TestBackend, 2, Bool>::from_data([[true, true, false]]);

        let data_actual = Tensor::cat(vec![tensor_1, tensor_2], 0).into_data();

        let data_expected = Data::from([[false, true, true], [true, true, false]]);
        assert_eq!(&data_actual, &data_expected);
    }

    #[test]
    fn should_support_cat_ops_2d_dim1() {
        let tensor_1 = TestTensor::from_data([[1.0, 2.0, 3.0]]);
        let tensor_2 = TestTensor::from_data([[4.0, 5.0, 6.0]]);

        let data_actual = TestTensor::cat(vec![tensor_1, tensor_2], 1).into_data();

        let data_expected = Data::from([[1.0, 2.0, 3.0, 4.0, 5.0, 6.0]]);
        data_expected.assert_approx_eq(&data_actual, 3);
    }

    #[test]
    fn should_support_cat_ops_3d() {
        let tensor_1 = TestTensor::from_data([[[1.0, 2.0, 3.0]], [[1.1, 2.1, 3.1]]]);
        let tensor_2 = TestTensor::from_data([[[4.0, 5.0, 6.0]]]);

        let data_actual = TestTensor::cat(vec![tensor_1, tensor_2], 0).into_data();

        let data_expected = Data::from([[[1.0, 2.0, 3.0]], [[1.1, 2.1, 3.1]], [[4.0, 5.0, 6.0]]]);
        data_expected.assert_approx_eq(&data_actual, 3);
    }

    #[test]
    #[should_panic(
        expected = "=== Tensor Operation Error ===\n  Operation: 'Cat'\n  Reason:\n    1. Can't concatenate tensors with different shapes, except for the provided dimension Provided dimension (0), tensors shapes: [Shape { dims: [2, 3] }, Shape { dims: [1, 2] }] \n"
    )]
    fn should_panic_when_dimensions_are_not_the_same() {
        let tensor_1 = TestTensor::from_data([[1.0, 2.0, 3.0], [1.0, 2.0, 3.0]]);
        let tensor_2 = TestTensor::from_data([[4.0, 5.0]]);

        TestTensor::cat(vec![tensor_1, tensor_2], 0).into_data();
    }

    #[test]
    #[should_panic(
        expected = "=== Tensor Operation Error ===\n  Operation: 'Cat'\n  Reason:\n    1. Can't concatenate an empty list of tensors. \n"
    )]
    fn should_panic_when_list_of_vectors_is_empty() {
        let tensor_2: Vec<burn_tensor::Tensor<TestBackend, 2, burn_tensor::Float>> = vec![];

        TestTensor::cat(tensor_2, 0).into_data();
    }

    #[test]
    #[should_panic(expected = "index out of bounds: the len is 3 but the index is 3")]
    fn should_panic_when_cat_exceeds_dimension() {
        let tensor_1 = TestTensor::from_data([[[1.0, 2.0, 3.0]], [[1.1, 2.1, 3.1]]]);
        let tensor_2 = TestTensor::from_data([[[4.0, 5.0, 6.0]]]);

        TestTensor::cat(vec![tensor_1, tensor_2], 3).into_data();
    }
}
