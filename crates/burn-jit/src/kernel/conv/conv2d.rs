use burn_cube::{calculate_cube_count_elemwise, prelude::*, SUBCUBE_DIM_APPROX};

use burn_tensor::{
    ops::{conv::calculate_conv_output_size, ConvOptions},
    Shape,
};

use crate::{
    kernel::into_contiguous,
    ops::{
        numeric::{empty_device, zeros_device},
        reshape,
    },
    tensor::JitTensor,
    FloatElement, JitRuntime,
};

struct Conv2dArgs {
    conv_stride_0: UInt,
    conv_stride_1: UInt,
    dilation_0: UInt,
    dilation_1: UInt,
    padding_0: UInt,
    padding_1: UInt,
    groups: UInt,
}

#[derive(Clone)]
struct Conv2dArgsExpand {
    conv_stride_0: <UInt as CubeType>::ExpandType,
    conv_stride_1: <UInt as CubeType>::ExpandType,
    dilation_0: <UInt as CubeType>::ExpandType,
    dilation_1: <UInt as CubeType>::ExpandType,
    padding_0: <UInt as CubeType>::ExpandType,
    padding_1: <UInt as CubeType>::ExpandType,
    groups: <UInt as CubeType>::ExpandType,
}

#[derive(Clone)]
struct Conv2dArgsLaunch<'a, R: Runtime> {
    conv_stride_0: <UInt as LaunchArg>::RuntimeArg<'a, R>,
    conv_stride_1: <UInt as LaunchArg>::RuntimeArg<'a, R>,
    dilation_0: <UInt as LaunchArg>::RuntimeArg<'a, R>,
    dilation_1: <UInt as LaunchArg>::RuntimeArg<'a, R>,
    padding_0: <UInt as LaunchArg>::RuntimeArg<'a, R>,
    padding_1: <UInt as LaunchArg>::RuntimeArg<'a, R>,
    groups: <UInt as LaunchArg>::RuntimeArg<'a, R>,
}

impl<'a, R: Runtime> ArgSettings<R> for Conv2dArgsLaunch<'a, R> {
    fn register(&self, launcher: &mut KernelLauncher<R>) {
        self.conv_stride_0.register(launcher);
        self.conv_stride_1.register(launcher);
        self.dilation_0.register(launcher);
        self.dilation_1.register(launcher);
        self.padding_0.register(launcher);
        self.padding_1.register(launcher);
        self.groups.register(launcher);
    }
}

impl CubeType for Conv2dArgs {
    type ExpandType = Conv2dArgsExpand;
}

impl LaunchArg for Conv2dArgs {
    type RuntimeArg<'a, R: Runtime> = Conv2dArgsLaunch<'a, R>;

    fn compile_input(
        builder: &mut KernelBuilder,
        vectorization: burn_cube::ir::Vectorization,
    ) -> <Self as CubeType>::ExpandType {
        Conv2dArgsExpand {
            conv_stride_0: UInt::compile_input(builder, vectorization),
            conv_stride_1: UInt::compile_input(builder, vectorization),
            dilation_0: UInt::compile_input(builder, vectorization),
            dilation_1: UInt::compile_input(builder, vectorization),
            padding_0: UInt::compile_input(builder, vectorization),
            padding_1: UInt::compile_input(builder, vectorization),
            groups: UInt::compile_input(builder, vectorization),
        }
    }

    fn compile_output(
        builder: &mut KernelBuilder,
        vectorization: burn_cube::ir::Vectorization,
    ) -> <Self as CubeType>::ExpandType {
        Conv2dArgsExpand {
            conv_stride_0: UInt::compile_output(builder, vectorization),
            conv_stride_1: UInt::compile_output(builder, vectorization),
            dilation_0: UInt::compile_output(builder, vectorization),
            dilation_1: UInt::compile_output(builder, vectorization),
            padding_0: UInt::compile_output(builder, vectorization),
            padding_1: UInt::compile_output(builder, vectorization),
            groups: UInt::compile_output(builder, vectorization),
        }
    }
}

#[cube(launch)]
fn conv2d_kernel<F: Float>(
    input: Tensor<F>,
    weight: Tensor<F>,
    bias: Tensor<F>,
    mut output: Tensor<F>,
    args: Conv2dArgs,
    kernel_size_0_unroll: Comptime<Option<UInt>>,
    kernel_size_1_unroll: Comptime<Option<UInt>>,
) {
    if ABSOLUTE_POS >= output.len() {
        return;
    }

    let in_channels = weight.shape(1);

    let kernel_size_0 = Comptime::unwrap_or_else(kernel_size_0_unroll, || weight.shape(2));
    let unroll_0 = Comptime::is_some(kernel_size_0_unroll);
    let kernel_size_1 = Comptime::unwrap_or_else(kernel_size_1_unroll, || weight.shape(3));
    let unroll_1 = Comptime::is_some(kernel_size_1_unroll);

    let b = ABSOLUTE_POS / output.stride(0) % output.shape(0);
    let oc = ABSOLUTE_POS / output.stride(1) % output.shape(1);
    let oh = ABSOLUTE_POS / output.stride(2) % output.shape(2);
    let ow = ABSOLUTE_POS / output.stride(3) % output.shape(3);

    let g = (weight.shape(0) + oc) % args.groups;
    let ic_start = in_channels * g;
    let ic_end = ic_start + in_channels;
    let mut sum = bias[oc];

    let ih_base = oh * args.conv_stride_0;
    let iw_base = ow * args.conv_stride_1;

    let weight_stride_1 = weight.stride(1);
    let weight_stride_2 = weight.stride(2);
    let weight_stride_3 = weight.stride(3);

    let input_stride_1 = input.stride(1);
    let input_stride_2 = input.stride(2);
    let input_stride_3 = input.stride(3);
    let input_shape_2 = input.shape(2);
    let input_shape_3 = input.shape(3);

    let border_top = args.padding_0;
    let border_left = args.padding_1;
    let border_bottom = input_shape_2 + args.padding_0;
    let border_right = input_shape_3 + args.padding_1;

    let index_input_0 = b * input.stride(0);
    let index_weight_0 = oc * weight.stride(0);

    for ic in range(ic_start, ic_end, Comptime::new(false)) {
        let index_input_1 = ic * input_stride_1;
        let index_weight_1 = (ic - ic_start) * weight_stride_1;

        for kh in range(0, kernel_size_0, unroll_0) {
            for kw in range(0, kernel_size_1, unroll_1) {
                let ih = kh * args.dilation_0 + ih_base;
                let iw = kw * args.dilation_1 + iw_base;

                let within_padding = ih >= border_top
                    && ih < border_bottom
                    && iw >= border_left
                    && iw < border_right;

                if within_padding {
                    let ih_pad = ih - args.padding_0;
                    let iw_pad = iw - args.padding_1;

                    let index_input = index_input_0
                        + index_input_1
                        + ih_pad * input_stride_2
                        + iw_pad * input_stride_3;

                    let index_weight = index_weight_0
                        + index_weight_1
                        + kh * weight_stride_2
                        + kw * weight_stride_3;

                    sum += input[index_input] * weight[index_weight];
                }
            }
        }
    }

    output[ABSOLUTE_POS] = sum;
}

pub(crate) fn conv2d<R: JitRuntime, E: FloatElement>(
    input: JitTensor<R, E, 4>,
    weight: JitTensor<R, E, 4>,
    bias: Option<JitTensor<R, E, 1>>,
    options: ConvOptions<2>,
) -> JitTensor<R, E, 4> {
    let input = into_contiguous(input);
    let weight = into_contiguous(weight);
    let [batch_size, _, in_height, in_width] = input.shape.dims;
    let [out_channels, _, kernel_0, kernel_1] = weight.shape.dims;

    let out_0 = calculate_conv_output_size(
        kernel_0,
        options.stride[0],
        options.padding[0],
        options.dilation[0],
        in_height,
    );
    let out_1 = calculate_conv_output_size(
        kernel_1,
        options.stride[1],
        options.padding[1],
        options.dilation[1],
        in_width,
    );

    let shape_out = Shape::new([batch_size, out_channels, out_0, out_1]);

    let output = empty_device(
        input.client.clone(),
        input.device.clone(),
        shape_out.clone(),
    );

    let bias = match bias {
        Some(bias) => {
            let shape = Shape::from([bias.shape.dims[0], 1, 1, 1]);
            reshape(bias, shape)
        }
        None => {
            let shape = Shape::from([output.shape.dims[0], 1, 1, 1]);
            zeros_device(input.client.clone(), input.device.clone(), shape)
        }
    };

    let num_elems_output = output.shape.num_elements();
    let workgroup = calculate_cube_count_elemwise(num_elems_output, SUBCUBE_DIM_APPROX);
    let settings = KernelSettings::default()
        .vectorize_input(0, 1)
        .vectorize_output(0, 1);

    conv2d_kernel_launch::<E::CubeElement, R>(
        input.client,
        workgroup,
        settings,
        TensorHandle::new(&input.handle, &input.strides, &input.shape.dims),
        TensorHandle::new(&weight.handle, &weight.strides, &weight.shape.dims),
        TensorHandle::new(&bias.handle, &bias.strides, &bias.shape.dims),
        TensorHandle::new(&output.handle, &output.strides, &output.shape.dims),
        Conv2dArgsLaunch {
            conv_stride_0: options.stride[0] as u32,
            conv_stride_1: options.stride[1] as u32,
            dilation_0: options.dilation[0] as u32,
            dilation_1: options.dilation[1] as u32,
            padding_0: options.padding[0] as u32,
            padding_1: options.padding[1] as u32,
            groups: options.groups as u32,
        },
        Some(kernel_0.into()),
        Some(kernel_1.into()),
    );

    output
}
