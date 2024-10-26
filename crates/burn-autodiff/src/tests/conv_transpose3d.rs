#[burn_tensor_testgen::testgen(ad_conv_transpose3d)]
mod tests {
    use super::*;
    use burn_tensor::{module::conv_transpose3d, ops::ConvTransposeOptions, Shape};

    #[test]
    fn test_conv_transpose3d_basic() {
        let test = ConvTranspose3dTestCase {
            batch_size: 2,
            channels: [2, 2],
            kernel_size: [3, 3, 3],
            padding: [0, 0, 0],
            padding_out: [0, 0, 0],
            stride: [1, 1, 1],
            dilation: [1, 1, 1],
            groups: 1,
            size: [4, 4, 4],
        };
        let device = Default::default();
        let grads = Grads {
            x: TestTensor::from_floats(
                [
                    [
                        [
                            [
                                [1431., 1431., 1431., 1431.],
                                [1431., 1431., 1431., 1431.],
                                [1431., 1431., 1431., 1431.],
                                [1431., 1431., 1431., 1431.],
                            ],
                            [
                                [1431., 1431., 1431., 1431.],
                                [1431., 1431., 1431., 1431.],
                                [1431., 1431., 1431., 1431.],
                                [1431., 1431., 1431., 1431.],
                            ],
                            [
                                [1431., 1431., 1431., 1431.],
                                [1431., 1431., 1431., 1431.],
                                [1431., 1431., 1431., 1431.],
                                [1431., 1431., 1431., 1431.],
                            ],
                            [
                                [1431., 1431., 1431., 1431.],
                                [1431., 1431., 1431., 1431.],
                                [1431., 1431., 1431., 1431.],
                                [1431., 1431., 1431., 1431.],
                            ],
                        ],
                        [
                            [
                                [4347., 4347., 4347., 4347.],
                                [4347., 4347., 4347., 4347.],
                                [4347., 4347., 4347., 4347.],
                                [4347., 4347., 4347., 4347.],
                            ],
                            [
                                [4347., 4347., 4347., 4347.],
                                [4347., 4347., 4347., 4347.],
                                [4347., 4347., 4347., 4347.],
                                [4347., 4347., 4347., 4347.],
                            ],
                            [
                                [4347., 4347., 4347., 4347.],
                                [4347., 4347., 4347., 4347.],
                                [4347., 4347., 4347., 4347.],
                                [4347., 4347., 4347., 4347.],
                            ],
                            [
                                [4347., 4347., 4347., 4347.],
                                [4347., 4347., 4347., 4347.],
                                [4347., 4347., 4347., 4347.],
                                [4347., 4347., 4347., 4347.],
                            ],
                        ],
                    ],
                    [
                        [
                            [
                                [1431., 1431., 1431., 1431.],
                                [1431., 1431., 1431., 1431.],
                                [1431., 1431., 1431., 1431.],
                                [1431., 1431., 1431., 1431.],
                            ],
                            [
                                [1431., 1431., 1431., 1431.],
                                [1431., 1431., 1431., 1431.],
                                [1431., 1431., 1431., 1431.],
                                [1431., 1431., 1431., 1431.],
                            ],
                            [
                                [1431., 1431., 1431., 1431.],
                                [1431., 1431., 1431., 1431.],
                                [1431., 1431., 1431., 1431.],
                                [1431., 1431., 1431., 1431.],
                            ],
                            [
                                [1431., 1431., 1431., 1431.],
                                [1431., 1431., 1431., 1431.],
                                [1431., 1431., 1431., 1431.],
                                [1431., 1431., 1431., 1431.],
                            ],
                        ],
                        [
                            [
                                [4347., 4347., 4347., 4347.],
                                [4347., 4347., 4347., 4347.],
                                [4347., 4347., 4347., 4347.],
                                [4347., 4347., 4347., 4347.],
                            ],
                            [
                                [4347., 4347., 4347., 4347.],
                                [4347., 4347., 4347., 4347.],
                                [4347., 4347., 4347., 4347.],
                                [4347., 4347., 4347., 4347.],
                            ],
                            [
                                [4347., 4347., 4347., 4347.],
                                [4347., 4347., 4347., 4347.],
                                [4347., 4347., 4347., 4347.],
                                [4347., 4347., 4347., 4347.],
                            ],
                            [
                                [4347., 4347., 4347., 4347.],
                                [4347., 4347., 4347., 4347.],
                                [4347., 4347., 4347., 4347.],
                                [4347., 4347., 4347., 4347.],
                            ],
                        ],
                    ],
                ],
                &device,
            ),
            weight: TestTensor::from_floats(
                [
                    [
                        [
                            [
                                [12224., 12224., 12224.],
                                [12224., 12224., 12224.],
                                [12224., 12224., 12224.],
                            ],
                            [
                                [12224., 12224., 12224.],
                                [12224., 12224., 12224.],
                                [12224., 12224., 12224.],
                            ],
                            [
                                [12224., 12224., 12224.],
                                [12224., 12224., 12224.],
                                [12224., 12224., 12224.],
                            ],
                        ],
                        [
                            [
                                [12224., 12224., 12224.],
                                [12224., 12224., 12224.],
                                [12224., 12224., 12224.],
                            ],
                            [
                                [12224., 12224., 12224.],
                                [12224., 12224., 12224.],
                                [12224., 12224., 12224.],
                            ],
                            [
                                [12224., 12224., 12224.],
                                [12224., 12224., 12224.],
                                [12224., 12224., 12224.],
                            ],
                        ],
                    ],
                    [
                        [
                            [
                                [20416., 20416., 20416.],
                                [20416., 20416., 20416.],
                                [20416., 20416., 20416.],
                            ],
                            [
                                [20416., 20416., 20416.],
                                [20416., 20416., 20416.],
                                [20416., 20416., 20416.],
                            ],
                            [
                                [20416., 20416., 20416.],
                                [20416., 20416., 20416.],
                                [20416., 20416., 20416.],
                            ],
                        ],
                        [
                            [
                                [20416., 20416., 20416.],
                                [20416., 20416., 20416.],
                                [20416., 20416., 20416.],
                            ],
                            [
                                [20416., 20416., 20416.],
                                [20416., 20416., 20416.],
                                [20416., 20416., 20416.],
                            ],
                            [
                                [20416., 20416., 20416.],
                                [20416., 20416., 20416.],
                                [20416., 20416., 20416.],
                            ],
                        ],
                    ],
                ],
                &device,
            ),
            bias: TestTensor::from_floats([432., 432.], &device),
        };
        test.assert_grads(grads);
    }

    #[test]
    fn test_conv_transpose3d_complex_groups() {
        let test = ConvTranspose3dTestCase {
            batch_size: 1,
            channels: [4, 2],
            kernel_size: [2, 3, 4],
            padding: [1, 2, 3],
            padding_out: [1, 2, 3],
            stride: [2, 3, 4],
            dilation: [1, 2, 3],
            groups: 2,
            size: [6, 6, 6],
        };
        let device = Default::default();
        let grads = Grads {
            x: TestTensor::from_floats(
                [[
                    [
                        [
                            [120., 156., 156., 156., 156., 156.],
                            [162., 210., 210., 210., 210., 210.],
                            [162., 210., 210., 210., 210., 210.],
                            [162., 210., 210., 210., 210., 210.],
                            [162., 210., 210., 210., 210., 210.],
                            [162., 210., 210., 210., 210., 210.],
                        ],
                        [
                            [168., 216., 216., 216., 216., 216.],
                            [216., 276., 276., 276., 276., 276.],
                            [216., 276., 276., 276., 276., 276.],
                            [216., 276., 276., 276., 276., 276.],
                            [216., 276., 276., 276., 276., 276.],
                            [216., 276., 276., 276., 276., 276.],
                        ],
                        [
                            [168., 216., 216., 216., 216., 216.],
                            [216., 276., 276., 276., 276., 276.],
                            [216., 276., 276., 276., 276., 276.],
                            [216., 276., 276., 276., 276., 276.],
                            [216., 276., 276., 276., 276., 276.],
                            [216., 276., 276., 276., 276., 276.],
                        ],
                        [
                            [168., 216., 216., 216., 216., 216.],
                            [216., 276., 276., 276., 276., 276.],
                            [216., 276., 276., 276., 276., 276.],
                            [216., 276., 276., 276., 276., 276.],
                            [216., 276., 276., 276., 276., 276.],
                            [216., 276., 276., 276., 276., 276.],
                        ],
                        [
                            [168., 216., 216., 216., 216., 216.],
                            [216., 276., 276., 276., 276., 276.],
                            [216., 276., 276., 276., 276., 276.],
                            [216., 276., 276., 276., 276., 276.],
                            [216., 276., 276., 276., 276., 276.],
                            [216., 276., 276., 276., 276., 276.],
                        ],
                        [
                            [168., 216., 216., 216., 216., 216.],
                            [216., 276., 276., 276., 276., 276.],
                            [216., 276., 276., 276., 276., 276.],
                            [216., 276., 276., 276., 276., 276.],
                            [216., 276., 276., 276., 276., 276.],
                            [216., 276., 276., 276., 276., 276.],
                        ],
                    ],
                    [
                        [
                            [264., 348., 348., 348., 348., 348.],
                            [378., 498., 498., 498., 498., 498.],
                            [378., 498., 498., 498., 498., 498.],
                            [378., 498., 498., 498., 498., 498.],
                            [378., 498., 498., 498., 498., 498.],
                            [378., 498., 498., 498., 498., 498.],
                        ],
                        [
                            [456., 600., 600., 600., 600., 600.],
                            [648., 852., 852., 852., 852., 852.],
                            [648., 852., 852., 852., 852., 852.],
                            [648., 852., 852., 852., 852., 852.],
                            [648., 852., 852., 852., 852., 852.],
                            [648., 852., 852., 852., 852., 852.],
                        ],
                        [
                            [456., 600., 600., 600., 600., 600.],
                            [648., 852., 852., 852., 852., 852.],
                            [648., 852., 852., 852., 852., 852.],
                            [648., 852., 852., 852., 852., 852.],
                            [648., 852., 852., 852., 852., 852.],
                            [648., 852., 852., 852., 852., 852.],
                        ],
                        [
                            [456., 600., 600., 600., 600., 600.],
                            [648., 852., 852., 852., 852., 852.],
                            [648., 852., 852., 852., 852., 852.],
                            [648., 852., 852., 852., 852., 852.],
                            [648., 852., 852., 852., 852., 852.],
                            [648., 852., 852., 852., 852., 852.],
                        ],
                        [
                            [456., 600., 600., 600., 600., 600.],
                            [648., 852., 852., 852., 852., 852.],
                            [648., 852., 852., 852., 852., 852.],
                            [648., 852., 852., 852., 852., 852.],
                            [648., 852., 852., 852., 852., 852.],
                            [648., 852., 852., 852., 852., 852.],
                        ],
                        [
                            [456., 600., 600., 600., 600., 600.],
                            [648., 852., 852., 852., 852., 852.],
                            [648., 852., 852., 852., 852., 852.],
                            [648., 852., 852., 852., 852., 852.],
                            [648., 852., 852., 852., 852., 852.],
                            [648., 852., 852., 852., 852., 852.],
                        ],
                    ],
                    [
                        [
                            [408., 540., 540., 540., 540., 540.],
                            [594., 786., 786., 786., 786., 786.],
                            [594., 786., 786., 786., 786., 786.],
                            [594., 786., 786., 786., 786., 786.],
                            [594., 786., 786., 786., 786., 786.],
                            [594., 786., 786., 786., 786., 786.],
                        ],
                        [
                            [744., 984., 984., 984., 984., 984.],
                            [1080., 1428., 1428., 1428., 1428., 1428.],
                            [1080., 1428., 1428., 1428., 1428., 1428.],
                            [1080., 1428., 1428., 1428., 1428., 1428.],
                            [1080., 1428., 1428., 1428., 1428., 1428.],
                            [1080., 1428., 1428., 1428., 1428., 1428.],
                        ],
                        [
                            [744., 984., 984., 984., 984., 984.],
                            [1080., 1428., 1428., 1428., 1428., 1428.],
                            [1080., 1428., 1428., 1428., 1428., 1428.],
                            [1080., 1428., 1428., 1428., 1428., 1428.],
                            [1080., 1428., 1428., 1428., 1428., 1428.],
                            [1080., 1428., 1428., 1428., 1428., 1428.],
                        ],
                        [
                            [744., 984., 984., 984., 984., 984.],
                            [1080., 1428., 1428., 1428., 1428., 1428.],
                            [1080., 1428., 1428., 1428., 1428., 1428.],
                            [1080., 1428., 1428., 1428., 1428., 1428.],
                            [1080., 1428., 1428., 1428., 1428., 1428.],
                            [1080., 1428., 1428., 1428., 1428., 1428.],
                        ],
                        [
                            [744., 984., 984., 984., 984., 984.],
                            [1080., 1428., 1428., 1428., 1428., 1428.],
                            [1080., 1428., 1428., 1428., 1428., 1428.],
                            [1080., 1428., 1428., 1428., 1428., 1428.],
                            [1080., 1428., 1428., 1428., 1428., 1428.],
                            [1080., 1428., 1428., 1428., 1428., 1428.],
                        ],
                        [
                            [744., 984., 984., 984., 984., 984.],
                            [1080., 1428., 1428., 1428., 1428., 1428.],
                            [1080., 1428., 1428., 1428., 1428., 1428.],
                            [1080., 1428., 1428., 1428., 1428., 1428.],
                            [1080., 1428., 1428., 1428., 1428., 1428.],
                            [1080., 1428., 1428., 1428., 1428., 1428.],
                        ],
                    ],
                    [
                        [
                            [552., 732., 732., 732., 732., 732.],
                            [810., 1074., 1074., 1074., 1074., 1074.],
                            [810., 1074., 1074., 1074., 1074., 1074.],
                            [810., 1074., 1074., 1074., 1074., 1074.],
                            [810., 1074., 1074., 1074., 1074., 1074.],
                            [810., 1074., 1074., 1074., 1074., 1074.],
                        ],
                        [
                            [1032., 1368., 1368., 1368., 1368., 1368.],
                            [1512., 2004., 2004., 2004., 2004., 2004.],
                            [1512., 2004., 2004., 2004., 2004., 2004.],
                            [1512., 2004., 2004., 2004., 2004., 2004.],
                            [1512., 2004., 2004., 2004., 2004., 2004.],
                            [1512., 2004., 2004., 2004., 2004., 2004.],
                        ],
                        [
                            [1032., 1368., 1368., 1368., 1368., 1368.],
                            [1512., 2004., 2004., 2004., 2004., 2004.],
                            [1512., 2004., 2004., 2004., 2004., 2004.],
                            [1512., 2004., 2004., 2004., 2004., 2004.],
                            [1512., 2004., 2004., 2004., 2004., 2004.],
                            [1512., 2004., 2004., 2004., 2004., 2004.],
                        ],
                        [
                            [1032., 1368., 1368., 1368., 1368., 1368.],
                            [1512., 2004., 2004., 2004., 2004., 2004.],
                            [1512., 2004., 2004., 2004., 2004., 2004.],
                            [1512., 2004., 2004., 2004., 2004., 2004.],
                            [1512., 2004., 2004., 2004., 2004., 2004.],
                            [1512., 2004., 2004., 2004., 2004., 2004.],
                        ],
                        [
                            [1032., 1368., 1368., 1368., 1368., 1368.],
                            [1512., 2004., 2004., 2004., 2004., 2004.],
                            [1512., 2004., 2004., 2004., 2004., 2004.],
                            [1512., 2004., 2004., 2004., 2004., 2004.],
                            [1512., 2004., 2004., 2004., 2004., 2004.],
                            [1512., 2004., 2004., 2004., 2004., 2004.],
                        ],
                        [
                            [1032., 1368., 1368., 1368., 1368., 1368.],
                            [1512., 2004., 2004., 2004., 2004., 2004.],
                            [1512., 2004., 2004., 2004., 2004., 2004.],
                            [1512., 2004., 2004., 2004., 2004., 2004.],
                            [1512., 2004., 2004., 2004., 2004., 2004.],
                            [1512., 2004., 2004., 2004., 2004., 2004.],
                        ],
                    ],
                ]],
                &device,
            ),
            weight: TestTensor::from_floats(
                [
                    [[
                        [
                            [16125., 19275., 19275., 19275.],
                            [18900., 22590., 22590., 22590.],
                            [18900., 22590., 22590., 22590.],
                        ],
                        [
                            [16650., 19890., 19890., 19890.],
                            [19440., 23220., 23220., 23220.],
                            [19440., 23220., 23220., 23220.],
                        ],
                    ]],
                    [[
                        [
                            [43125., 51675., 51675., 51675.],
                            [51300., 61470., 61470., 61470.],
                            [51300., 61470., 61470., 61470.],
                        ],
                        [
                            [49050., 58770., 58770., 58770.],
                            [58320., 69876., 69876., 69876.],
                            [58320., 69876., 69876., 69876.],
                        ],
                    ]],
                    [[
                        [
                            [70125., 84075., 84075., 84075.],
                            [83700., 100350., 100350., 100350.],
                            [83700., 100350., 100350., 100350.],
                        ],
                        [
                            [81450., 97650., 97650., 97650.],
                            [97200., 116532., 116532., 116532.],
                            [97200., 116532., 116532., 116532.],
                        ],
                    ]],
                    [[
                        [
                            [97125., 116475., 116475., 116475.],
                            [116100., 139230., 139230., 139230.],
                            [116100., 139230., 139230., 139230.],
                        ],
                        [
                            [113850., 136530., 136530., 136530.],
                            [136080., 163188., 163188., 163188.],
                            [136080., 163188., 163188., 163188.],
                        ],
                    ]],
                ],
                &device,
            ),
            bias: TestTensor::from_floats([5346., 5346.], &device),
        };
        test.assert_grads(grads);
    }

    struct ConvTranspose3dTestCase {
        batch_size: usize,
        channels: [usize; 2],
        kernel_size: [usize; 3],
        padding: [usize; 3],
        padding_out: [usize; 3],
        stride: [usize; 3],
        dilation: [usize; 3],
        groups: usize,
        size: [usize; 3],
    }

    struct Grads {
        x: TestTensor<5>,
        weight: TestTensor<5>,
        bias: TestTensor<1>,
    }

    impl ConvTranspose3dTestCase {
        fn assert_grads(self, expected_grads: Grads) {
            let shape_x = Shape::new([
                self.batch_size,
                self.channels[0],
                self.size[0],
                self.size[1],
                self.size[2],
            ]);
            let shape_weight = Shape::new([
                self.channels[0],
                self.channels[1] / self.groups,
                self.kernel_size[0],
                self.kernel_size[1],
                self.kernel_size[2],
            ]);
            let device = Default::default();
            let weight = TestAutodiffTensor::from_data(
                TestTensorInt::arange(0..shape_weight.num_elements() as i64, &device)
                    .reshape::<5, _>(shape_weight)
                    .into_data(),
                &device,
            )
            .require_grad();
            let bias = TestAutodiffTensor::from_data(
                TestTensorInt::arange(0..self.channels[1] as i64, &device).into_data(),
                &device,
            )
            .require_grad();
            let x = TestAutodiffTensor::from_data(
                TestTensorInt::arange(0..shape_x.num_elements() as i64, &device)
                    .reshape::<5, _>(shape_x)
                    .into_data(),
                &device,
            )
            .require_grad();
            let output = conv_transpose3d(
                x.clone(),
                weight.clone(),
                Some(bias.clone()),
                ConvTransposeOptions::new(
                    self.stride,
                    self.padding,
                    self.padding_out,
                    self.dilation,
                    self.groups,
                ),
            );
            let grads = output.backward();

            // Assert
            let x_grad_actual = x.grad(&grads).unwrap();
            let weight_grad_actual = weight.grad(&grads).unwrap();
            let bias_grad_actual = bias.grad(&grads).unwrap();

            expected_grads
                .bias
                .to_data()
                .assert_approx_eq(&bias_grad_actual.to_data(), 3);
            expected_grads
                .x
                .to_data()
                .assert_approx_eq(&x_grad_actual.to_data(), 3);
            expected_grads
                .weight
                .to_data()
                .assert_approx_eq(&weight_grad_actual.to_data(), 3);
        }
    }
}
