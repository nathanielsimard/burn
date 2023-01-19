use crate::{element::TchElement, TchBackend, TchKind, TchTensor};
use burn_tensor::{
    ops::{MaxPooling2dBackward, MaxPooling2dWithIndices, ModuleOps},
    Shape,
};

impl<E: TchElement> ModuleOps<TchBackend<E>> for TchBackend<E> {
    fn embedding(weights: &TchTensor<E, 2>, indexes: &TchTensor<i64, 2>) -> TchTensor<E, 3> {
        let tensor = tch::Tensor::embedding(&weights.tensor, &indexes.tensor, -1, false, false);
        let shape = Shape::from(tensor.size());

        TchTensor {
            kind: weights.kind,
            tensor,
            shape,
        }
    }

    fn embedding_backward(
        weights: &TchTensor<E, 2>,
        output: &TchTensor<E, 3>,
        indexes: &TchTensor<i64, 2>,
    ) -> TchTensor<E, 2> {
        let [n_embedding, _d_model] = weights.shape.dims;
        let tensor = tch::Tensor::embedding_backward(
            &output.tensor,
            &indexes.tensor,
            n_embedding as i64,
            -1,
            false,
            false,
        );
        let shape = Shape::from(tensor.size());

        TchTensor {
            kind: weights.kind,
            tensor,
            shape,
        }
    }

    fn conv1d(
        x: &TchTensor<E, 3>,
        weight: &TchTensor<E, 3>,
        bias: Option<&TchTensor<E, 1>>,
        stride: usize,
        padding: usize,
    ) -> TchTensor<E, 3> {
        let tensor = tch::Tensor::conv1d(
            &x.tensor,
            &weight.tensor,
            bias.map(|t| &t.tensor),
            &[stride as i64],
            &[padding as i64],
            &[1],
            1,
        );
        let shape = Shape::from(tensor.size());

        TchTensor {
            kind: weight.kind,
            tensor,
            shape,
        }
    }

    fn conv2d(
        x: &TchTensor<E, 4>,
        weight: &TchTensor<E, 4>,
        bias: Option<&TchTensor<E, 1>>,
        stride: [usize; 2],
        padding: [usize; 2],
    ) -> TchTensor<E, 4> {
        let tensor = tch::Tensor::conv2d(
            &x.tensor,
            &weight.tensor,
            bias.map(|t| &t.tensor),
            &[stride[0] as i64, stride[1] as i64],
            &[padding[0] as i64, padding[1] as i64],
            &[1, 1],
            1,
        );
        let shape = Shape::from(tensor.size());

        TchTensor {
            kind: weight.kind,
            tensor,
            shape,
        }
    }

    fn max_pooling2d(
        x: &TchTensor<E, 4>,
        kernel_size: [usize; 2],
        stride: [usize; 2],
        padding: [usize; 2],
    ) -> TchTensor<E, 4> {
        let tensor = tch::Tensor::max_pool2d(
            &x.tensor,
            &[kernel_size[0] as i64, kernel_size[1] as i64],
            &[stride[0] as i64, stride[1] as i64],
            &[padding[0] as i64, padding[1] as i64],
            &[1, 1],
            false,
        );
        let shape = Shape::from(tensor.size());

        TchTensor {
            kind: x.kind,
            tensor,
            shape,
        }
    }

    fn max_pooling2d_with_indices(
        x: &TchTensor<E, 4>,
        kernel_size: [usize; 2],
        stride: [usize; 2],
        padding: [usize; 2],
    ) -> MaxPooling2dWithIndices<TchBackend<E>> {
        let (tensor, indices) = tch::Tensor::max_pool2d_with_indices(
            &x.tensor,
            &[kernel_size[0] as i64, kernel_size[1] as i64],
            &[stride[0] as i64, stride[1] as i64],
            &[padding[0] as i64, padding[1] as i64],
            &[1, 1],
            false,
        );
        let shape = Shape::from(tensor.size());

        let output = TchTensor {
            kind: x.kind,
            tensor,
            shape,
        };
        let shape = Shape::from(indices.size());
        let indices = TchTensor {
            kind: TchKind::<i64>::new(),
            tensor: indices,
            shape,
        };

        MaxPooling2dWithIndices::new(output, indices)
    }

    fn max_pooling2d_backward(
        x: &TchTensor<E, 4>,
        kernel_size: [usize; 2],
        stride: [usize; 2],
        padding: [usize; 2],
        output_grad: &TchTensor<E, 4>,
        indices: &TchTensor<i64, 4>,
    ) -> MaxPooling2dBackward<TchBackend<E>> {
        let grad = tch::Tensor::max_pool2d_with_indices_backward(
            &x.tensor,
            &output_grad.tensor,
            &[kernel_size[0] as i64, kernel_size[1] as i64],
            &[stride[0] as i64, stride[1] as i64],
            &[padding[0] as i64, padding[1] as i64],
            &[1, 1],
            false,
            &indices.tensor,
        );

        let shape = Shape::from(grad.size());
        let tensor = TchTensor {
            kind: x.kind,
            tensor: grad,
            shape,
        };

        MaxPooling2dBackward::new(tensor)
    }
}
