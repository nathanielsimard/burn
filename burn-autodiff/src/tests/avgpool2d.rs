#[burn_tensor_testgen::testgen(ad_avg_pool2d)]
mod tests {
    use super::*;
    use burn_tensor::module::avg_pool2d;
    use burn_tensor::{Data, Shape, Tensor};

    #[test]
    fn test_avg_pool2d_simple() {
        let test = AvgPool2dTestCase {
            batch_size: 1,
            channels: 1,
            kernel_size_1: 3,
            kernel_size_2: 3,
            padding_1: 0,
            padding_2: 0,
            stride_1: 1,
            stride_2: 1,
            height: 6,
            width: 6,
        };

        test.assert_output(TestTensor::from_floats([[[
            [0.1111, 0.2222, 0.3333, 0.3333, 0.2222, 0.1111],
            [0.2222, 0.4444, 0.6667, 0.6667, 0.4444, 0.2222],
            [0.3333, 0.6667, 1.0000, 1.0000, 0.6667, 0.3333],
            [0.3333, 0.6667, 1.0000, 1.0000, 0.6667, 0.3333],
            [0.2222, 0.4444, 0.6667, 0.6667, 0.4444, 0.2222],
            [0.1111, 0.2222, 0.3333, 0.3333, 0.2222, 0.1111],
        ]]]));
    }

    #[test]
    fn test_avg_pool2d_complex() {
        let test = AvgPool2dTestCase {
            batch_size: 1,
            channels: 1,
            kernel_size_1: 3,
            kernel_size_2: 4,
            padding_1: 1,
            padding_2: 2,
            stride_1: 1,
            stride_2: 2,
            height: 4,
            width: 6,
        };

        test.assert_output(TestTensor::from_floats([[[
            [0.3333, 0.3333, 0.3333, 0.3333, 0.3333, 0.3333],
            [0.5000, 0.5000, 0.5000, 0.5000, 0.5000, 0.5000],
            [0.5000, 0.5000, 0.5000, 0.5000, 0.5000, 0.5000],
            [0.3333, 0.3333, 0.3333, 0.3333, 0.3333, 0.3333],
        ]]]));
    }

    struct AvgPool2dTestCase {
        batch_size: usize,
        channels: usize,
        kernel_size_1: usize,
        kernel_size_2: usize,
        padding_1: usize,
        padding_2: usize,
        stride_1: usize,
        stride_2: usize,
        height: usize,
        width: usize,
    }

    impl AvgPool2dTestCase {
        fn assert_output(self, x_grad: TestTensor<4>) {
            let shape_x = Shape::new([self.batch_size, self.channels, self.height, self.width]);
            let x = TestADTensor::from_data(
                TestTensorInt::arange(0..shape_x.num_elements())
                    .reshape(shape_x)
                    .into_data()
                    .convert(),
            )
            .require_grad();
            let output = avg_pool2d(
                x.clone(),
                [self.kernel_size_1, self.kernel_size_2],
                [self.stride_1, self.stride_2],
                [self.padding_1, self.padding_2],
            );
            let grads = output.backward();
            println!("{output}");
            let x_grad_actual = x.grad(&grads).unwrap();

            x_grad
                .to_data()
                .assert_approx_eq(&x_grad_actual.into_data(), 3);
        }
    }
}
