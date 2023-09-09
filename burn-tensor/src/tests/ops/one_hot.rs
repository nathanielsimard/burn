#[burn_tensor_testgen::testgen(one_hot)]
mod tests {
    use super::*;
    use burn_tensor::{Data, Int, Tensor};

    #[test]
    fn should_support_one_hot() {
        let tensor = Tensor::<TestBackend, 1>::one_hot(0, 5);
        assert_eq!(tensor.to_data(), Data::from([1., 0., 0., 0., 0.]));

        let tensor = Tensor::<TestBackend, 1>::one_hot(1, 5);
        assert_eq!(tensor.to_data(), Data::from([0., 1., 0., 0., 0.]));

        let tensor = Tensor::<TestBackend, 1>::one_hot(4, 5);
        assert_eq!(tensor.to_data(), Data::from([0., 0., 0., 0., 1.]));

        let tensor = Tensor::<TestBackend, 1>::one_hot(1, 2);
        assert_eq!(tensor.to_data(), Data::from([0., 1.]));
    }

    #[test]
    #[should_panic]
    fn index_exceeds_number_of_classes() {
        let tensor = Tensor::<TestBackend, 1>::one_hot(1, 1);
    }

    #[test]
    #[should_panic]
    fn number_of_classes_is_zero() {
        let tensor = Tensor::<TestBackend, 1>::one_hot(0, 0);
    }
}
