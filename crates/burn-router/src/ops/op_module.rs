use alloc::{boxed::Box, vec};

use burn_tensor::ops::conv::{
    calculate_conv_output_size, calculate_conv_transpose_output_size, calculate_pool_output_size,
};
use burn_tensor::ops::{
    ConvOptions, ConvTransposeOptions, DeformConv2dBackward, DeformConvOptions, FloatTensor,
    IntElem, ModuleOps,
};
use burn_tensor::ops::{
    IntTensor, InterpolateOptions, MaxPool1dBackward, MaxPool1dWithIndices, MaxPool2dBackward,
    MaxPool2dWithIndices,
};
use burn_tensor::repr::{
    AdaptiveAvgPool1dBackwardDescription, AdaptiveAvgPool1dDescription,
    AdaptiveAvgPool2dBackwardDescription, AdaptiveAvgPool2dDescription,
    AvgPool1dBackwardDescription, AvgPool1dDescription, AvgPool2dBackwardDescription,
    AvgPool2dDescription, Conv1dDescription, Conv2dDescription, Conv3dDescription,
    ConvTranspose1dDescription, ConvTranspose2dDescription, ConvTranspose3dDescription,
    DeformConv2dBackwardDescription, DeformConv2dDescription, InterpolateBackwardDescription,
    InterpolateDescription, MaxPool1dDescription, MaxPool1dWithIndicesBackwardDescription,
    MaxPool1dWithIndicesDescription, MaxPool2dDescription, MaxPool2dWithIndicesBackwardDescription,
    MaxPool2dWithIndicesDescription, ModuleOperationDescription, OperationDescription,
};
use burn_tensor::Element;

use crate::{BackendRouter, RunnerChannel, RunnerClient};

impl<R: RunnerChannel> ModuleOps<Self> for BackendRouter<R> {
    fn conv1d(
        x: FloatTensor<Self>,
        weight: FloatTensor<Self>,
        bias: Option<FloatTensor<Self>>,
        options: ConvOptions<1>,
    ) -> FloatTensor<Self> {
        let size = calculate_conv_output_size(
            weight.shape[2],
            options.stride[0],
            options.padding[0],
            options.dilation[0],
            x.shape[2],
        );

        let shape = vec![x.shape[0], weight.shape[0], size];
        let client = x.client.clone();
        let out = client.register_empty_tensor(shape, x.dtype);

        let desc = Conv1dDescription {
            x: x.into_description(),
            weight: weight.into_description(),
            bias: bias.map(|bias| bias.into_description()),
            options: options.into(),
            out: out.to_description_out(),
        };

        client.register(OperationDescription::Module(
            ModuleOperationDescription::Conv1d(desc),
        ));

        out
    }

    fn conv2d(
        x: FloatTensor<Self>,
        weight: FloatTensor<Self>,
        bias: Option<FloatTensor<Self>>,
        options: ConvOptions<2>,
    ) -> FloatTensor<Self> {
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

        let shape = vec![x.shape[0], weight.shape[0], size_0, size_1];
        let client = x.client.clone();
        let out = client.register_empty_tensor(shape, x.dtype);

        let desc = Conv2dDescription {
            x: x.into_description(),
            weight: weight.into_description(),
            bias: bias.map(|bias| bias.into_description()),
            options: options.into(),
            out: out.to_description_out(),
        };

        client.register(OperationDescription::Module(
            ModuleOperationDescription::Conv2d(desc),
        ));

        out
    }

    fn conv3d(
        x: FloatTensor<Self>,
        weight: FloatTensor<Self>,
        bias: Option<FloatTensor<Self>>,
        options: ConvOptions<3>,
    ) -> FloatTensor<Self> {
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
        let size_2 = calculate_conv_output_size(
            weight.shape[4],
            options.stride[2],
            options.padding[2],
            options.dilation[2],
            x.shape[4],
        );

        let shape = vec![x.shape[0], weight.shape[0], size_0, size_1, size_2];
        let client = x.client.clone();
        let out = client.register_empty_tensor(shape, x.dtype);

        let desc = Conv3dDescription {
            x: x.into_description(),
            weight: weight.into_description(),
            bias: bias.map(|bias| bias.into_description()),
            options: options.into(),
            out: out.to_description_out(),
        };

        client.register(OperationDescription::Module(
            ModuleOperationDescription::Conv3d(desc),
        ));

        out
    }

    fn conv_transpose1d(
        x: FloatTensor<Self>,
        weight: FloatTensor<Self>,
        bias: Option<FloatTensor<Self>>,
        options: ConvTransposeOptions<1>,
    ) -> FloatTensor<Self> {
        let size = calculate_conv_transpose_output_size(
            weight.shape[2],
            options.stride[0],
            options.padding[0],
            options.padding_out[0],
            options.dilation[0],
            x.shape[2],
        );

        let shape = vec![x.shape[0], weight.shape[1] * options.groups, size];
        let client = x.client.clone();
        let out = client.register_empty_tensor(shape, x.dtype);

        let desc = ConvTranspose1dDescription {
            x: x.into_description(),
            weight: weight.into_description(),
            bias: bias.map(|bias| bias.into_description()),
            options: options.into(),
            out: out.to_description_out(),
        };

        client.register(OperationDescription::Module(
            ModuleOperationDescription::ConvTranspose1d(desc),
        ));

        out
    }

    fn conv_transpose2d(
        x: FloatTensor<Self>,
        weight: FloatTensor<Self>,
        bias: Option<FloatTensor<Self>>,
        options: ConvTransposeOptions<2>,
    ) -> FloatTensor<Self> {
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

        let shape = vec![x.shape[0], weight.shape[1] * options.groups, size_0, size_1];
        let client = x.client.clone();
        let out = client.register_empty_tensor(shape, x.dtype);

        let desc = ConvTranspose2dDescription {
            x: x.into_description(),
            weight: weight.into_description(),
            bias: bias.map(|bias| bias.into_description()),
            options: options.into(),
            out: out.to_description_out(),
        };

        client.register(OperationDescription::Module(
            ModuleOperationDescription::ConvTranspose2d(desc),
        ));

        out
    }

    fn conv_transpose3d(
        x: FloatTensor<Self>,
        weight: FloatTensor<Self>,
        bias: Option<FloatTensor<Self>>,
        options: ConvTransposeOptions<3>,
    ) -> FloatTensor<Self> {
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
        let size_2 = calculate_conv_transpose_output_size(
            weight.shape[4],
            options.stride[2],
            options.padding[2],
            options.padding_out[2],
            options.dilation[2],
            x.shape[4],
        );

        let shape = vec![
            x.shape[0],
            weight.shape[1] * options.groups,
            size_0,
            size_1,
            size_2,
        ];
        let client = x.client.clone();
        let out = client.register_empty_tensor(shape, x.dtype);

        let desc = ConvTranspose3dDescription {
            x: x.into_description(),
            weight: weight.into_description(),
            bias: bias.map(|bias| bias.into_description()),
            options: options.into(),
            out: out.to_description_out(),
        };

        client.register(OperationDescription::Module(
            ModuleOperationDescription::ConvTranspose3d(desc),
        ));

        out
    }

    fn avg_pool1d(
        x: FloatTensor<Self>,
        kernel_size: usize,
        stride: usize,
        padding: usize,
        count_include_pad: bool,
    ) -> FloatTensor<Self> {
        let size = calculate_pool_output_size(kernel_size, stride, padding, 1, x.shape[2]);

        let shape = vec![x.shape[0], x.shape[1], size];
        let client = x.client.clone();
        let out = client.register_empty_tensor(shape, x.dtype);

        let desc = AvgPool1dDescription {
            x: x.into_description(),
            kernel_size,
            stride,
            padding,
            count_include_pad,
            out: out.to_description_out(),
        };

        client.register(OperationDescription::Module(
            ModuleOperationDescription::AvgPool1d(desc),
        ));

        out
    }

    fn avg_pool2d(
        x: FloatTensor<Self>,
        kernel_size: [usize; 2],
        stride: [usize; 2],
        padding: [usize; 2],
        count_include_pad: bool,
    ) -> FloatTensor<Self> {
        let size_0 =
            calculate_pool_output_size(kernel_size[0], stride[0], padding[0], 1, x.shape[2]);
        let size_1 =
            calculate_pool_output_size(kernel_size[1], stride[1], padding[1], 1, x.shape[3]);

        let shape = vec![x.shape[0], x.shape[1], size_0, size_1];
        let client = x.client.clone();
        let out = client.register_empty_tensor(shape, x.dtype);

        let desc = AvgPool2dDescription {
            x: x.into_description(),
            kernel_size,
            stride,
            padding,
            count_include_pad,
            out: out.to_description_out(),
        };

        client.register(OperationDescription::Module(
            ModuleOperationDescription::AvgPool2d(desc),
        ));

        out
    }

    fn avg_pool1d_backward(
        x: FloatTensor<Self>,
        grad: FloatTensor<Self>,
        kernel_size: usize,
        stride: usize,
        padding: usize,
        count_include_pad: bool,
    ) -> FloatTensor<Self> {
        let client = x.client.clone();
        let out = client.register_empty_tensor(x.shape.clone(), x.dtype);

        let desc = AvgPool1dBackwardDescription {
            x: x.into_description(),
            grad: grad.into_description(),
            kernel_size,
            stride,
            padding,
            count_include_pad,
            out: out.to_description_out(),
        };

        client.register(OperationDescription::Module(
            ModuleOperationDescription::AvgPool1dBackward(desc),
        ));

        out
    }

    fn avg_pool2d_backward(
        x: FloatTensor<Self>,
        grad: FloatTensor<Self>,
        kernel_size: [usize; 2],
        stride: [usize; 2],
        padding: [usize; 2],
        count_include_pad: bool,
    ) -> FloatTensor<Self> {
        let client = x.client.clone();
        let out = client.register_empty_tensor(x.shape.clone(), x.dtype);

        let desc = AvgPool2dBackwardDescription {
            x: x.into_description(),
            grad: grad.into_description(),
            kernel_size,
            stride,
            padding,
            count_include_pad,
            out: out.to_description_out(),
        };

        client.register(OperationDescription::Module(
            ModuleOperationDescription::AvgPool2dBackward(desc),
        ));

        out
    }

    fn max_pool1d(
        x: FloatTensor<Self>,
        kernel_size: usize,
        stride: usize,
        padding: usize,
        dilation: usize,
    ) -> FloatTensor<Self> {
        let size = calculate_pool_output_size(kernel_size, stride, padding, dilation, x.shape[2]);

        let shape = vec![x.shape[0], x.shape[1], size];
        let client = x.client.clone();
        let out = client.register_empty_tensor(shape, x.dtype);

        let desc = MaxPool1dDescription {
            x: x.into_description(),
            kernel_size,
            stride,
            padding,
            dilation,
            out: out.to_description_out(),
        };

        client.register(OperationDescription::Module(
            ModuleOperationDescription::MaxPool1d(desc),
        ));

        out
    }

    fn max_pool2d(
        x: FloatTensor<Self>,
        kernel_size: [usize; 2],
        stride: [usize; 2],
        padding: [usize; 2],
        dilation: [usize; 2],
    ) -> FloatTensor<Self> {
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

        let shape = vec![x.shape[0], x.shape[1], size_0, size_1];
        let client = x.client.clone();
        let out = client.register_empty_tensor(shape, x.dtype);

        let desc = MaxPool2dDescription {
            x: x.into_description(),
            kernel_size,
            stride,
            padding,
            dilation,
            out: out.to_description_out(),
        };

        client.register(OperationDescription::Module(
            ModuleOperationDescription::MaxPool2d(desc),
        ));

        out
    }

    fn max_pool1d_with_indices(
        x: FloatTensor<Self>,
        kernel_size: usize,
        stride: usize,
        padding: usize,
        dilation: usize,
    ) -> MaxPool1dWithIndices<Self> {
        let size = calculate_pool_output_size(kernel_size, stride, padding, dilation, x.shape[2]);

        let shape = vec![x.shape[0], x.shape[1], size];
        let client = x.client.clone();
        let out = client.register_empty_tensor(shape.clone(), x.dtype);
        let out_indices = client.register_empty_tensor(shape, IntElem::<Self>::dtype());

        let desc = MaxPool1dWithIndicesDescription {
            x: x.into_description(),
            kernel_size,
            stride,
            padding,
            dilation,
            out: out.to_description_out(),
            out_indices: out_indices.to_description_out(),
        };

        client.register(OperationDescription::Module(
            ModuleOperationDescription::MaxPool1dWithIndices(desc),
        ));

        MaxPool1dWithIndices::new(out, out_indices)
    }

    fn max_pool2d_with_indices(
        x: FloatTensor<Self>,
        kernel_size: [usize; 2],
        stride: [usize; 2],
        padding: [usize; 2],
        dilation: [usize; 2],
    ) -> MaxPool2dWithIndices<Self> {
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

        let shape = vec![x.shape[0], x.shape[1], size_0, size_1];
        let client = x.client.clone();
        let out = client.register_empty_tensor(shape.clone(), x.dtype);
        let out_indices = client.register_empty_tensor(shape, IntElem::<Self>::dtype());

        let desc = MaxPool2dWithIndicesDescription {
            x: x.into_description(),
            kernel_size,
            stride,
            padding,
            dilation,
            out: out.to_description_out(),
            out_indices: out_indices.to_description_out(),
        };

        client.register(OperationDescription::Module(
            ModuleOperationDescription::MaxPool2dWithIndices(desc),
        ));

        MaxPool2dWithIndices::new(out, out_indices)
    }

    fn max_pool1d_with_indices_backward(
        x: FloatTensor<Self>,
        kernel_size: usize,
        stride: usize,
        padding: usize,
        dilation: usize,
        output_grad: FloatTensor<Self>,
        indices: IntTensor<Self>,
    ) -> MaxPool1dBackward<Self> {
        let client = x.client.clone();
        let out = client.register_empty_tensor(x.shape.clone(), x.dtype);

        let desc = MaxPool1dWithIndicesBackwardDescription {
            x: x.into_description(),
            grad: output_grad.into_description(),
            indices: indices.into_description(),
            kernel_size,
            stride,
            padding,
            dilation,
            out: out.to_description_out(),
        };

        client.register(OperationDescription::Module(
            ModuleOperationDescription::MaxPool1dWithIndicesBackward(desc),
        ));

        MaxPool1dBackward::new(out)
    }

    fn max_pool2d_with_indices_backward(
        x: FloatTensor<Self>,
        kernel_size: [usize; 2],
        stride: [usize; 2],
        padding: [usize; 2],
        dilation: [usize; 2],
        output_grad: FloatTensor<Self>,
        indices: IntTensor<Self>,
    ) -> MaxPool2dBackward<Self> {
        let client = x.client.clone();
        let out = client.register_empty_tensor(x.shape.clone(), x.dtype);

        let desc = MaxPool2dWithIndicesBackwardDescription {
            x: x.into_description(),
            grad: output_grad.into_description(),
            indices: indices.into_description(),
            kernel_size,
            stride,
            padding,
            dilation,
            out: out.to_description_out(),
        };

        client.register(OperationDescription::Module(
            ModuleOperationDescription::MaxPool2dWithIndicesBackward(desc),
        ));

        MaxPool2dBackward::new(out)
    }

    fn adaptive_avg_pool1d(x: FloatTensor<Self>, output_size: usize) -> FloatTensor<Self> {
        let shape = vec![x.shape[0], x.shape[1], output_size];

        let client = x.client.clone();
        let out = client.register_empty_tensor(shape.clone(), x.dtype);

        let desc = AdaptiveAvgPool1dDescription {
            x: x.into_description(),
            output_size,
            out: out.to_description_out(),
        };

        client.register(OperationDescription::Module(
            ModuleOperationDescription::AdaptiveAvgPool1d(desc),
        ));

        out
    }

    fn adaptive_avg_pool2d(x: FloatTensor<Self>, output_size: [usize; 2]) -> FloatTensor<Self> {
        let shape = vec![x.shape[0], x.shape[1], output_size[0], output_size[1]];

        let client = x.client.clone();
        let out = client.register_empty_tensor(shape.clone(), x.dtype);

        let desc = AdaptiveAvgPool2dDescription {
            x: x.into_description(),
            output_size,
            out: out.to_description_out(),
        };

        client.register(OperationDescription::Module(
            ModuleOperationDescription::AdaptiveAvgPool2d(desc),
        ));

        out
    }

    fn adaptive_avg_pool1d_backward(
        x: FloatTensor<Self>,
        grad: FloatTensor<Self>,
    ) -> FloatTensor<Self> {
        let client = x.client.clone();
        let out = client.register_empty_tensor(x.shape.clone(), x.dtype);

        let desc = AdaptiveAvgPool1dBackwardDescription {
            x: x.into_description(),
            grad: grad.into_description(),
            out: out.to_description_out(),
        };

        client.register(OperationDescription::Module(
            ModuleOperationDescription::AdaptiveAvgPool1dBackward(desc),
        ));

        out
    }

    fn adaptive_avg_pool2d_backward(
        x: FloatTensor<Self>,
        grad: FloatTensor<Self>,
    ) -> FloatTensor<Self> {
        let client = x.client.clone();
        let out = client.register_empty_tensor(x.shape.clone(), x.dtype);

        let desc = AdaptiveAvgPool2dBackwardDescription {
            x: x.into_description(),
            grad: grad.into_description(),
            out: out.to_description_out(),
        };

        client.register(OperationDescription::Module(
            ModuleOperationDescription::AdaptiveAvgPool2dBackward(desc),
        ));

        out
    }

    fn interpolate(
        x: FloatTensor<Self>,
        output_size: [usize; 2],
        options: InterpolateOptions,
    ) -> FloatTensor<Self> {
        let shape = vec![x.shape[0], x.shape[1], output_size[0], output_size[1]];

        let client = x.client.clone();
        let out = client.register_empty_tensor(shape.clone(), x.dtype);

        let desc = InterpolateDescription {
            x: x.into_description(),
            output_size,
            options: options.into(),
            out: out.to_description_out(),
        };

        client.register(OperationDescription::Module(
            ModuleOperationDescription::Interpolate(desc),
        ));

        out
    }

    fn interpolate_backward(
        x: FloatTensor<Self>,
        grad: FloatTensor<Self>,
        output_size: [usize; 2],
        options: InterpolateOptions,
    ) -> FloatTensor<Self> {
        let client = x.client.clone();
        let out = client.register_empty_tensor(x.shape.clone(), x.dtype);

        let desc = InterpolateBackwardDescription {
            x: x.into_description(),
            grad: grad.into_description(),
            output_size,
            options: options.into(),
            out: out.to_description_out(),
        };

        client.register(OperationDescription::Module(
            ModuleOperationDescription::InterpolateBackward(desc),
        ));

        out
    }

    fn deform_conv2d(
        x: FloatTensor<Self>,
        offset: FloatTensor<Self>,
        weight: FloatTensor<Self>,
        mask: Option<FloatTensor<Self>>,
        bias: Option<FloatTensor<Self>>,
        options: DeformConvOptions<2>,
    ) -> FloatTensor<Self> {
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

        let shape = vec![x.shape[0], weight.shape[0], size_0, size_1];
        let client = x.client.clone();
        let out = client.register_empty_tensor(shape, x.dtype);

        let desc = DeformConv2dDescription {
            x: x.into_description(),
            offset: offset.into_description(),
            weight: weight.into_description(),
            mask: mask.map(|mask| mask.into_description()),
            bias: bias.map(|bias| bias.into_description()),
            options: options.into(),
            out: out.to_description_out(),
        };

        client.register(OperationDescription::Module(
            ModuleOperationDescription::DeformableConv2d(Box::new(desc)),
        ));

        out
    }

    fn deform_conv2d_backward(
        x: FloatTensor<Self>,
        offset: FloatTensor<Self>,
        weight: FloatTensor<Self>,
        mask: Option<FloatTensor<Self>>,
        bias: Option<FloatTensor<Self>>,
        output_grad: FloatTensor<Self>,
        options: DeformConvOptions<2>,
    ) -> DeformConv2dBackward<Self> {
        let client = x.client.clone();

        let input_grad = client.register_empty_tensor(x.shape.clone(), x.dtype);
        let offset_grad = client.register_empty_tensor(offset.shape.clone(), offset.dtype);
        let weight_grad = client.register_empty_tensor(weight.shape.clone(), weight.dtype);
        let mask_grad = mask
            .as_ref()
            .map(|mask| client.register_empty_tensor(mask.shape.clone(), mask.dtype));
        let bias_grad = bias
            .as_ref()
            .map(|bias| client.register_empty_tensor(bias.shape.clone(), bias.dtype));

        let desc = DeformConv2dBackwardDescription {
            x: x.into_description(),
            offset: offset.into_description(),
            weight: weight.into_description(),
            mask: mask.map(|mask| mask.into_description()),
            bias: bias.map(|bias| bias.into_description()),
            options: options.into(),
            out_grad: output_grad.into_description(),
            input_grad: input_grad.to_description_out(),
            offset_grad: offset_grad.to_description_out(),
            weight_grad: weight_grad.to_description_out(),
            mask_grad: mask_grad
                .as_ref()
                .map(|mask_grad| mask_grad.to_description_out()),
            bias_grad: bias_grad
                .as_ref()
                .map(|bias_grad| bias_grad.to_description_out()),
        };

        client.register(OperationDescription::Module(
            ModuleOperationDescription::DeformableConv2dBackward(Box::new(desc)),
        ));

        DeformConv2dBackward::new(input_grad, offset_grad, weight_grad, mask_grad, bias_grad)
    }
}
