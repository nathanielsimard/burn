use crate::{
    client::FusionClient,
    graph::{
        AdaptiveAvgPool2dBackwardDescription, AdaptiveAvgPool2dDescription,
        AvgPool2dBackwardDescription, AvgPool2dDescription, Conv1dDescription, Conv2dDescription,
        ConvTranspose1dDescription, ConvTranspose2dDescription, MaxPool2dDescription,
        MaxPool2dWithIndicesBackwardDescription, MaxPool2dWithIndicesDescription, Ops,
        TensorOpsDescription,
    },
    Fusion, FusionBackend,
};
use burn_tensor::ops::{
    conv::{
        calculate_conv_output_size, calculate_conv_transpose_output_size,
        calculate_pool_output_size,
    },
    ConvOptions, ConvTransposeOptions, FloatTensor, IntTensor, MaxPool2dBackward,
    MaxPool2dWithIndices, ModuleOps,
};

impl<B: FusionBackend> ModuleOps<Fusion<B>> for Fusion<B> {
    fn conv1d(
        x: FloatTensor<Self, 3>,
        weight: FloatTensor<Self, 3>,
        bias: Option<FloatTensor<Self, 1>>,
        options: ConvOptions<1>,
    ) -> FloatTensor<Self, 3> {
        struct Conv1dOps;

        impl<B: FusionBackend> Ops<B> for Conv1dOps {
            type Args = Conv1dDescription;

            fn execute(&self, args: &Self::Args, handles: &mut crate::HandleContainer<B>) {
                let x = handles.get_float_tensor(&args.x);
                let weight = handles.get_float_tensor(&args.weight);
                let bias = args
                    .bias
                    .as_ref()
                    .map(|bias| handles.get_float_tensor(bias));

                let output = B::conv1d(x, weight, bias, args.options.clone());

                handles.register_float_tensor(&args.out.id, output);
            }
        }

        let size = calculate_conv_output_size(
            weight.shape[2],
            options.stride[0],
            options.padding[0],
            options.dilation[0],
            x.shape[2],
        );

        let mut shape = Vec::with_capacity(3);
        shape.push(x.shape[0]);
        shape.push(weight.shape[0]);
        shape.push(size);

        let out = x.client.create_tensor_empty(shape);

        x.client.clone().register(TensorOpsDescription::ModuleOps(
            crate::graph::ModuleOpsDescription::Conv1d(
                Conv1dDescription {
                    x: x.into_description(),
                    weight: weight.into_description(),
                    bias: bias.map(|bias| bias.into_description()),
                    options,
                    out: out.to_description_out(),
                },
                Box::new(Conv1dOps),
            ),
        ));

        out
    }

    fn conv2d(
        x: FloatTensor<Self, 4>,
        weight: FloatTensor<Self, 4>,
        bias: Option<FloatTensor<Self, 1>>,
        options: ConvOptions<2>,
    ) -> FloatTensor<Self, 4> {
        struct Conv2dOps;

        impl<B: FusionBackend> Ops<B> for Conv2dOps {
            type Args = Conv2dDescription;

            fn execute(&self, args: &Self::Args, handles: &mut crate::HandleContainer<B>) {
                let x = handles.get_float_tensor(&args.x);
                let weight = handles.get_float_tensor(&args.weight);
                let bias = args
                    .bias
                    .as_ref()
                    .map(|bias| handles.get_float_tensor(bias));

                let output = B::conv2d(x, weight, bias, args.options.clone());

                handles.register_float_tensor(&args.out.id, output);
            }
        }

        let size_0 = calculate_conv_output_size(
            weight.shape[2],
            options.stride[0],
            options.padding[0],
            options.dilation[0],
            x.shape[2],
        );
        let size_1 = calculate_conv_output_size(
            weight.shape[3],
            options.stride[1],
            options.padding[1],
            options.dilation[1],
            x.shape[3],
        );

        let mut shape = Vec::with_capacity(4);
        shape.push(x.shape[0]);
        shape.push(weight.shape[0]);
        shape.push(size_0);
        shape.push(size_1);

        let out = x.client.create_tensor_empty(shape);

        x.client.clone().register(TensorOpsDescription::ModuleOps(
            crate::graph::ModuleOpsDescription::Conv2d(
                Conv2dDescription {
                    x: x.into_description(),
                    weight: weight.into_description(),
                    bias: bias.map(|bias| bias.into_description()),
                    options,
                    out: out.to_description_out(),
                },
                Box::new(Conv2dOps),
            ),
        ));

        out
    }

    fn conv_transpose1d(
        x: FloatTensor<Self, 3>,
        weight: FloatTensor<Self, 3>,
        bias: Option<FloatTensor<Self, 1>>,
        options: ConvTransposeOptions<1>,
    ) -> FloatTensor<Self, 3> {
        struct ConvTranspose1dOps;

        impl<B: FusionBackend> Ops<B> for ConvTranspose1dOps {
            type Args = ConvTranspose1dDescription;

            fn execute(&self, args: &Self::Args, handles: &mut crate::HandleContainer<B>) {
                let x = handles.get_float_tensor(&args.x);
                let weight = handles.get_float_tensor(&args.weight);
                let bias = args
                    .bias
                    .as_ref()
                    .map(|bias| handles.get_float_tensor(bias));

                let output = B::conv_transpose1d(x, weight, bias, args.options.clone());

                handles.register_float_tensor(&args.out.id, output);
            }
        }

        let size = calculate_conv_transpose_output_size(
            weight.shape[2],
            options.stride[0],
            options.padding[0],
            options.padding_out[0],
            options.dilation[0],
            x.shape[2],
        );

        let mut shape = Vec::with_capacity(3);
        shape.push(x.shape[0]);
        shape.push(weight.shape[1] * options.groups);
        shape.push(size);

        let out = x.client.create_tensor_empty(shape);

        x.client.clone().register(TensorOpsDescription::ModuleOps(
            crate::graph::ModuleOpsDescription::ConvTranspose1d(
                ConvTranspose1dDescription {
                    x: x.into_description(),
                    weight: weight.into_description(),
                    bias: bias.map(|bias| bias.into_description()),
                    options,
                    out: out.to_description_out(),
                },
                Box::new(ConvTranspose1dOps),
            ),
        ));

        out
    }

    fn conv_transpose2d(
        x: FloatTensor<Self, 4>,
        weight: FloatTensor<Self, 4>,
        bias: Option<FloatTensor<Self, 1>>,
        options: ConvTransposeOptions<2>,
    ) -> FloatTensor<Self, 4> {
        struct ConvTranspose2dOps;

        impl<B: FusionBackend> Ops<B> for ConvTranspose2dOps {
            type Args = ConvTranspose2dDescription;

            fn execute(&self, args: &Self::Args, handles: &mut crate::HandleContainer<B>) {
                let x = handles.get_float_tensor(&args.x);
                let weight = handles.get_float_tensor(&args.weight);
                let bias = args
                    .bias
                    .as_ref()
                    .map(|bias| handles.get_float_tensor(bias));

                let output = B::conv_transpose2d(x, weight, bias, args.options.clone());

                handles.register_float_tensor(&args.out.id, output);
            }
        }

        let size_0 = calculate_conv_transpose_output_size(
            weight.shape[2],
            options.stride[0],
            options.padding[0],
            options.padding_out[0],
            options.dilation[0],
            x.shape[2],
        );
        let size_1 = calculate_conv_transpose_output_size(
            weight.shape[3],
            options.stride[1],
            options.padding[1],
            options.padding_out[1],
            options.dilation[1],
            x.shape[3],
        );

        let mut shape = Vec::with_capacity(4);
        shape.push(x.shape[0]);
        shape.push(weight.shape[1] * options.groups);
        shape.push(size_0);
        shape.push(size_1);

        let out = x.client.create_tensor_empty(shape);

        x.client.clone().register(TensorOpsDescription::ModuleOps(
            crate::graph::ModuleOpsDescription::ConvTranspose2d(
                ConvTranspose2dDescription {
                    x: x.into_description(),
                    weight: weight.into_description(),
                    bias: bias.map(|bias| bias.into_description()),
                    options,
                    out: out.to_description_out(),
                },
                Box::new(ConvTranspose2dOps),
            ),
        ));

        out
    }

    fn avg_pool2d(
        x: FloatTensor<Self, 4>,
        kernel_size: [usize; 2],
        stride: [usize; 2],
        padding: [usize; 2],
        count_include_pad: bool,
    ) -> FloatTensor<Self, 4> {
        struct AvgPool2dOps;

        impl<B: FusionBackend> Ops<B> for AvgPool2dOps {
            type Args = AvgPool2dDescription;

            fn execute(&self, args: &Self::Args, handles: &mut crate::HandleContainer<B>) {
                let x = handles.get_float_tensor(&args.x);
                let output = B::avg_pool2d(
                    x,
                    args.kernel_size,
                    args.stride,
                    args.padding,
                    args.count_include_pad,
                );

                handles.register_float_tensor(&args.out.id, output);
            }
        }

        let size_0 =
            calculate_pool_output_size(kernel_size[0], stride[0], padding[0], 1, x.shape[2]);
        let size_1 =
            calculate_pool_output_size(kernel_size[1], stride[1], padding[1], 1, x.shape[3]);

        let mut shape = Vec::with_capacity(4);
        shape.push(x.shape[0]);
        shape.push(x.shape[1]);
        shape.push(size_0);
        shape.push(size_1);

        let out = x.client.create_tensor_empty(shape);

        x.client.clone().register(TensorOpsDescription::ModuleOps(
            crate::graph::ModuleOpsDescription::AvgPool2d(
                AvgPool2dDescription {
                    x: x.into_description(),
                    kernel_size,
                    stride,
                    padding,
                    count_include_pad,
                    out: out.to_description_out(),
                },
                Box::new(AvgPool2dOps),
            ),
        ));

        out
    }

    fn avg_pool2d_backward(
        x: FloatTensor<Self, 4>,
        grad: FloatTensor<Self, 4>,
        kernel_size: [usize; 2],
        stride: [usize; 2],
        padding: [usize; 2],
        count_include_pad: bool,
    ) -> FloatTensor<Self, 4> {
        struct AvgPool2dBackwardOps;

        impl<B: FusionBackend> Ops<B> for AvgPool2dBackwardOps {
            type Args = AvgPool2dBackwardDescription;

            fn execute(&self, args: &Self::Args, handles: &mut crate::HandleContainer<B>) {
                let x = handles.get_float_tensor(&args.x);
                let grad = handles.get_float_tensor(&args.grad);
                let output = B::avg_pool2d_backward(
                    x,
                    grad,
                    args.kernel_size,
                    args.stride,
                    args.padding,
                    args.count_include_pad,
                );

                handles.register_float_tensor(&args.out.id, output);
            }
        }

        let out = x.client.create_tensor_empty(x.shape.clone());

        x.client.clone().register(TensorOpsDescription::ModuleOps(
            crate::graph::ModuleOpsDescription::AvgPool2dBackward(
                AvgPool2dBackwardDescription {
                    x: x.into_description(),
                    grad: grad.into_description(),
                    kernel_size,
                    stride,
                    padding,
                    count_include_pad,
                    out: out.to_description_out(),
                },
                Box::new(AvgPool2dBackwardOps),
            ),
        ));

        out
    }

    fn max_pool2d(
        x: FloatTensor<Self, 4>,
        kernel_size: [usize; 2],
        stride: [usize; 2],
        padding: [usize; 2],
        dilation: [usize; 2],
    ) -> FloatTensor<Self, 4> {
        struct MaxPool2dOps;

        impl<B: FusionBackend> Ops<B> for MaxPool2dOps {
            type Args = MaxPool2dDescription;

            fn execute(&self, args: &Self::Args, handles: &mut crate::HandleContainer<B>) {
                let x = handles.get_float_tensor(&args.x);
                let output = B::max_pool2d(
                    x,
                    args.kernel_size,
                    args.stride,
                    args.padding,
                    args.dilation,
                );

                handles.register_float_tensor(&args.out.id, output);
            }
        }

        let size_0 = calculate_pool_output_size(
            kernel_size[0],
            stride[0],
            padding[0],
            dilation[0],
            x.shape[2],
        );
        let size_1 = calculate_pool_output_size(
            kernel_size[1],
            stride[1],
            padding[1],
            dilation[1],
            x.shape[3],
        );

        let mut shape = Vec::with_capacity(4);
        shape.push(x.shape[0]);
        shape.push(x.shape[1]);
        shape.push(size_0);
        shape.push(size_1);

        let out = x.client.create_tensor_empty(shape);

        x.client.clone().register(TensorOpsDescription::ModuleOps(
            crate::graph::ModuleOpsDescription::MaxPool2d(
                MaxPool2dDescription {
                    x: x.into_description(),
                    kernel_size,
                    stride,
                    padding,
                    dilation,
                    out: out.to_description_out(),
                },
                Box::new(MaxPool2dOps),
            ),
        ));

        out
    }

    fn max_pool2d_with_indices(
        x: FloatTensor<Self, 4>,
        kernel_size: [usize; 2],
        stride: [usize; 2],
        padding: [usize; 2],
        dilation: [usize; 2],
    ) -> MaxPool2dWithIndices<Self> {
        struct MaxPool2dWithIndicesOps;

        impl<B: FusionBackend> Ops<B> for MaxPool2dWithIndicesOps {
            type Args = MaxPool2dWithIndicesDescription;

            fn execute(&self, args: &Self::Args, handles: &mut crate::HandleContainer<B>) {
                let x = handles.get_float_tensor(&args.x);
                let output = B::max_pool2d_with_indices(
                    x,
                    args.kernel_size,
                    args.stride,
                    args.padding,
                    args.dilation,
                );

                handles.register_float_tensor(&args.out.id, output.output);
                handles.register_int_tensor(&args.out_indices.id, output.indices);
            }
        }

        let size_0 = calculate_pool_output_size(
            kernel_size[0],
            stride[0],
            padding[0],
            dilation[0],
            x.shape[2],
        );
        let size_1 = calculate_pool_output_size(
            kernel_size[1],
            stride[1],
            padding[1],
            dilation[1],
            x.shape[3],
        );

        let mut shape = Vec::with_capacity(4);
        shape.push(x.shape[0]);
        shape.push(x.shape[1]);
        shape.push(size_0);
        shape.push(size_1);

        let out = x.client.create_tensor_empty(shape.clone());
        let out_indices = x.client.create_tensor_empty(shape);

        x.client.clone().register(TensorOpsDescription::ModuleOps(
            crate::graph::ModuleOpsDescription::MaxPool2dWithIndices(
                MaxPool2dWithIndicesDescription {
                    x: x.into_description(),
                    kernel_size,
                    stride,
                    padding,
                    dilation,
                    out: out.to_description_out(),
                    out_indices: out_indices.to_description_out(),
                },
                Box::new(MaxPool2dWithIndicesOps),
            ),
        ));

        MaxPool2dWithIndices::new(out, out_indices)
    }

    fn max_pool2d_with_indices_backward(
        x: FloatTensor<Self, 4>,
        kernel_size: [usize; 2],
        stride: [usize; 2],
        padding: [usize; 2],
        dilation: [usize; 2],
        output_grad: FloatTensor<Self, 4>,
        indices: IntTensor<Self, 4>,
    ) -> MaxPool2dBackward<Self> {
        struct MaxPool2dWithIndicesBackwardOps;

        impl<B: FusionBackend> Ops<B> for MaxPool2dWithIndicesBackwardOps {
            type Args = MaxPool2dWithIndicesBackwardDescription;

            fn execute(&self, args: &Self::Args, handles: &mut crate::HandleContainer<B>) {
                let x = handles.get_float_tensor(&args.x);
                let grad = handles.get_float_tensor(&args.grad);
                let indices = handles.get_int_tensor(&args.indices);
                let output = B::max_pool2d_with_indices_backward(
                    x,
                    args.kernel_size,
                    args.stride,
                    args.padding,
                    args.dilation,
                    grad,
                    indices,
                );

                handles.register_float_tensor(&args.out.id, output.x_grad);
            }
        }

        let out = x.client.create_tensor_empty(x.shape.clone());

        x.client.clone().register(TensorOpsDescription::ModuleOps(
            crate::graph::ModuleOpsDescription::MaxPool2dWithIndicesBackward(
                MaxPool2dWithIndicesBackwardDescription {
                    x: x.into_description(),
                    grad: output_grad.into_description(),
                    indices: indices.into_description(),
                    kernel_size,
                    stride,
                    padding,
                    dilation,
                    out: out.to_description_out(),
                },
                Box::new(MaxPool2dWithIndicesBackwardOps),
            ),
        ));

        MaxPool2dBackward::new(out)
    }

    fn adaptive_avg_pool2d(
        x: FloatTensor<Self, 4>,
        output_size: [usize; 2],
    ) -> FloatTensor<Self, 4> {
        struct AdaptiveAvgPool2dOps;

        impl<B: FusionBackend> Ops<B> for AdaptiveAvgPool2dOps {
            type Args = AdaptiveAvgPool2dDescription;

            fn execute(&self, args: &Self::Args, handles: &mut crate::HandleContainer<B>) {
                let x = handles.get_float_tensor(&args.x);
                let output = B::adaptive_avg_pool2d(x, args.output_size);

                handles.register_float_tensor(&args.out.id, output);
            }
        }

        let mut shape = Vec::with_capacity(4);
        shape.push(x.shape[0]);
        shape.push(x.shape[1]);
        shape.push(output_size[0]);
        shape.push(output_size[1]);
        let out = x.client.create_tensor_empty(shape);

        x.client.clone().register(TensorOpsDescription::ModuleOps(
            crate::graph::ModuleOpsDescription::AdaptiveAvgPool2d(
                AdaptiveAvgPool2dDescription {
                    x: x.into_description(),
                    output_size,
                    out: out.to_description_out(),
                },
                Box::new(AdaptiveAvgPool2dOps),
            ),
        ));

        out
    }

    fn adaptive_avg_pool2d_backward(
        x: FloatTensor<Self, 4>,
        grad: FloatTensor<Self, 4>,
    ) -> FloatTensor<Self, 4> {
        struct AdaptiveAvgPool2dBackwardOps;

        impl<B: FusionBackend> Ops<B> for AdaptiveAvgPool2dBackwardOps {
            type Args = AdaptiveAvgPool2dBackwardDescription;

            fn execute(&self, args: &Self::Args, handles: &mut crate::HandleContainer<B>) {
                let x = handles.get_float_tensor(&args.x);
                let grad = handles.get_float_tensor(&args.grad);
                let output = B::adaptive_avg_pool2d_backward(x, grad);

                handles.register_float_tensor(&args.out.id, output);
            }
        }

        let out = x.client.create_tensor_empty(x.shape.clone());

        x.client.clone().register(TensorOpsDescription::ModuleOps(
            crate::graph::ModuleOpsDescription::AdaptiveAvgPool2dBackward(
                AdaptiveAvgPool2dBackwardDescription {
                    x: x.into_description(),
                    grad: grad.into_description(),
                    out: out.to_description_out(),
                },
                Box::new(AdaptiveAvgPool2dBackwardOps),
            ),
        ));

        out
    }
}
