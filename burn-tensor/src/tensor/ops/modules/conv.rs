use super::Conv2dBackward;
use crate::{backend::Backend, ElementConversion, Shape};

/// Calculate the amount of padding necessary to keep the same output size when applying a
/// convolution with the specified kernel size and stride.
pub fn calculate_same_padding(kernel_size: usize, stride: usize, size: usize) -> usize {
    let kernel_size = kernel_size as f32;
    let stride = stride as f32;
    let size = size as f32;

    let padding = (stride - 1.) * size - stride + kernel_size;
    let padding = f32::floor(padding / 2.);

    padding as usize
}

/// Calculate the expected output size when applying a convolution with the specified kernel size,
/// stride and padding.
pub fn calculate_output_size(
    kernel_size: usize,
    stride: usize,
    padding: usize,
    size_in: usize,
) -> usize {
    let kernel_size = kernel_size as f32;
    let stride = stride as f32;
    let padding = padding as f32;
    let size_in = size_in as f32;

    let size_out = (size_in + (2. * padding) - (kernel_size - 1.) - 1.) / stride;
    let size_out = f32::floor(size_out + 1.);

    size_out as usize
}

/// Calculate a [2D convolution](crate::ops::ModuleOps::conv2d) with same padding strategy.
pub fn conv2d_same_padding<B: Backend>(
    x: &B::TensorPrimitive<4>,
    weight: &B::TensorPrimitive<4>,
    bias: Option<&B::TensorPrimitive<1>>,
    stride: [usize; 2],
) -> B::TensorPrimitive<4> {
    let [_, _, width, height] = B::shape(x).dims;
    let [_, _, kernel_size_1, kernel_size_2] = B::shape(weight).dims;

    let padding_1 = calculate_same_padding(kernel_size_1, stride[0], width);
    let padding_2 = calculate_same_padding(kernel_size_2, stride[1], height);

    B::conv2d(x, weight, bias, stride, [padding_1, padding_2])
}

/// Calculate the [2D convolution](crate::ops::ModuleOps::conv2d) backward pass using convolutions.
pub(crate) fn conv2d_backward<B: Backend>(
    x: &B::TensorPrimitive<4>,
    weight: &B::TensorPrimitive<4>,
    bias: Option<&B::TensorPrimitive<1>>,
    stride: [usize; 2],
    output_grad: &B::TensorPrimitive<4>,
) -> Conv2dBackward<B> {
    let [_batch_size, _channels_in, width, height] = B::shape(x).dims;

    let weight_tmp = B::swap_dims(weight, 0, 1);
    let x_grad = conv2d_same_padding::<B>(&weight_tmp, output_grad, None, stride);
    let x_grad = B::swap_dims(&x_grad, 0, 1);

    let x_tmp = B::swap_dims(x, 0, 1);
    let output_grad_tmp = B::swap_dims(output_grad, 0, 1);

    let weight_grad = conv2d_same_padding::<B>(&x_tmp, &output_grad_tmp, None, stride);
    let weight_grad = B::swap_dims(&weight_grad, 0, 1);

    Conv2dBackward::new(
        x_grad,
        weight_grad,
        bias.map(|b| {
            let elem = width * height;
            let elem = (elem as i32).to_elem();

            let b = B::zeros(*B::shape(b), B::device(b));
            

            B::add_scalar(&b, &elem)
        }),
    )
}

/// Execute a 1D convolution using a 2d convolution.
pub(crate) fn conv1d_from_conv2d<B: Backend>(
    x: &B::TensorPrimitive<3>,
    weight: &B::TensorPrimitive<3>,
    bias: Option<&B::TensorPrimitive<1>>,
    stride: usize,
    padding: usize,
) -> B::TensorPrimitive<3> {
    let [channels_out, _channels_in, kernel_size] = B::shape(weight).dims;
    let [batch_size, channels_in, length_in] = B::shape(x).dims;

    let weight = B::reshape(
        weight,
        Shape::new([channels_out, channels_in, kernel_size, 1]),
    );
    let x = B::reshape(x, Shape::new([batch_size, channels_in, length_in, 1]));

    let tensor = B::conv2d(&x, &weight, bias, [stride, 1], [padding, 0]);
    let [batch_size, channels_out, height_out, _weight_out] = B::shape(&tensor).dims;
    B::reshape(&tensor, Shape::from([batch_size, channels_out, height_out]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_output_size_1() {
        let kernel_size = 3;
        let stride = 1;
        let padding = 1;
        let size_in = 3;

        let size_out = calculate_output_size(kernel_size, stride, padding, size_in);

        assert_eq!(size_out, 3);
    }

    #[test]
    fn test_calculate_output_size_2() {
        let kernel_size = 5;
        let stride = 2;
        let padding = 3;
        let size_in = 27;

        let size_out = calculate_output_size(kernel_size, stride, padding, size_in);

        assert_eq!(size_out, 15);
    }

    #[test]
    fn test_calculate_same_padding_1() {
        let kernel_size = 3;
        let stride = 1;
        let size_in = 3;

        let padding_actual = calculate_same_padding(kernel_size, stride, size_in);
        let size_out = calculate_output_size(kernel_size, stride, padding_actual, size_in);

        let padding_expected = 1;
        assert_eq!(padding_actual, padding_expected, "Expected padding");
        assert_eq!(size_in, size_out, "Expected size");
    }

    #[test]
    fn test_calculate_same_padding_2() {
        let kernel_size = 3;
        let stride = 2;
        let size_in = 7;

        let padding_actual = calculate_same_padding(kernel_size, stride, size_in);
        let size_out = calculate_output_size(kernel_size, stride, padding_actual, size_in);

        let padding_expected = 4;
        assert_eq!(padding_actual, padding_expected, "Expected padding");
        assert_eq!(size_in, size_out, "Expected size");
    }
}
