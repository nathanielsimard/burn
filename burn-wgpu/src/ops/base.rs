use crate::{
    comparison,
    context::WorkGroup,
    element::WgpuElement,
    kernel::{build_info, comparison, KernelSettings},
    kernel_wgsl,
    pool::get_context,
    tensor::WgpuTensor,
    GraphicsApi, WgpuDevice,
};
use burn_tensor::{backend::Backend, Data, Shape};
use std::{marker::PhantomData, ops::Range};

pub type FloatElem<B> = <B as Backend>::FloatElem;
pub type Device<B> = <B as Backend>::Device;

pub type FloatTensor<B, const D: usize> = <B as Backend>::TensorPrimitive<D>;

pub type IntElem<B> = <B as Backend>::IntElem;
pub type IntTensor<B, const D: usize> = <B as Backend>::IntTensorPrimitive<D>;
pub type BoolTensor<B, const D: usize> = <B as Backend>::BoolTensorPrimitive<D>;

pub struct BaseOps<G: GraphicsApi> {
    _g: PhantomData<G>,
}

impl<G: GraphicsApi> BaseOps<G> {
    pub fn from_data<E: WgpuElement, const D: usize>(
        data: Data<E, D>,
        device: &WgpuDevice,
    ) -> WgpuTensor<E, D> {
        let context = get_context::<G>(device);
        let buffer = context.create_buffer_with_data(E::as_bytes(&data.value));

        WgpuTensor::new(context, data.shape, buffer)
    }

    pub fn into_data<E: WgpuElement, const D: usize>(tensor: WgpuTensor<E, D>) -> Data<E, D> {
        let tensor = Self::into_continuous(tensor);
        let bytes = tensor.context.read_buffer(tensor.buffer);
        let values = E::from_bytes(&bytes);

        Data::new(values.to_vec(), tensor.shape)
    }

    pub fn to_device<E: WgpuElement, const D: usize>(
        tensor: WgpuTensor<E, D>,
        device: &WgpuDevice,
    ) -> WgpuTensor<E, D> {
        if &tensor.context.device == device {
            return tensor;
        }

        let context = get_context::<G>(device);
        tensor.to_context(context)
    }

    pub fn empty<E: WgpuElement, const D: usize>(
        shape: Shape<D>,
        device: &WgpuDevice,
    ) -> WgpuTensor<E, D> {
        let context = get_context::<G>(device);
        let buffer = context.create_buffer(shape.num_elements() * core::mem::size_of::<E>());

        WgpuTensor::new(context, shape, buffer)
    }

    pub fn swap_dims<E: WgpuElement, const D: usize>(
        mut tensor: WgpuTensor<E, D>,
        dim1: usize,
        dim2: usize,
    ) -> WgpuTensor<E, D> {
        tensor.strides.swap(dim1, dim2);

        tensor.shape.dims.swap(dim1, dim2);

        tensor
    }

    pub fn reshape<E: WgpuElement, const D1: usize, const D2: usize>(
        tensor: WgpuTensor<E, D1>,
        shape: Shape<D2>,
    ) -> WgpuTensor<E, D2> {
        // TODO: Not force standard layout all the time (improve performance).
        let tensor = Self::into_continuous(tensor);

        WgpuTensor::new(tensor.context, shape, tensor.buffer)
    }

    pub fn into_continuous<E: WgpuElement, const D: usize>(
        tensor: WgpuTensor<E, D>,
    ) -> WgpuTensor<E, D> {
        if tensor.is_continuous() {
            return tensor;
        }

        kernel_wgsl!(ContinuousRaw, "../template/continuous.wgsl");

        let buffer = tensor
            .context
            .create_buffer(tensor.shape.num_elements() * core::mem::size_of::<E>());
        let output = WgpuTensor::new(tensor.context.clone(), tensor.shape.clone(), buffer);
        let info = build_info(&[&tensor, &output]);
        let info_buffer = tensor
            .context
            .create_buffer_with_data(bytemuck::cast_slice(&info));

        let kernel = tensor
            .context
            .compile_static::<KernelSettings<ContinuousRaw, E, i32, 256, 1, 1>>();

        tensor.context.execute(
            WorkGroup::new(
                f32::ceil(output.shape.num_elements() as f32 / 256_f32) as u32,
                1,
                1,
            ),
            kernel,
            &[&tensor.buffer, &output.buffer, &info_buffer],
        );

        output
    }

    pub fn index<E: WgpuElement, const D1: usize, const D2: usize>(
        tensor: WgpuTensor<E, D1>,
        indexes: [Range<usize>; D2],
    ) -> WgpuTensor<E, D1> {
        kernel_wgsl!(IndexRaw, "../template/index.wgsl");

        let mut dims = tensor.shape.dims;

        for i in 0..D2 {
            dims[i] = indexes[i].end - indexes[i].start;
        }

        let shape_output = Shape::new(dims);

        let buffer = tensor
            .context
            .create_buffer(shape_output.num_elements() * core::mem::size_of::<E>());
        let output = WgpuTensor::new(tensor.context.clone(), shape_output, buffer);
        let mut info = build_info(&[&tensor, &output]);

        for i in 0..D1 {
            let start = indexes.get(i).map(|index| index.start).unwrap_or(0);
            info.push(start as u32);
        }

        let info_buffer = tensor
            .context
            .create_buffer_with_data(bytemuck::cast_slice(&info));

        let kernel = tensor
            .context
            .compile_static::<KernelSettings<IndexRaw, E, i32, 256, 1, 1>>();

        tensor.context.execute(
            WorkGroup::new(
                f32::ceil(output.shape.num_elements() as f32 / 256_f32) as u32,
                1,
                1,
            ),
            kernel,
            &[&tensor.buffer, &output.buffer, &info_buffer],
        );

        output
    }

    pub fn index_assign<E: WgpuElement, const D1: usize, const D2: usize>(
        tensor: WgpuTensor<E, D1>,
        indexes: [Range<usize>; D2],
        value: WgpuTensor<E, D1>,
    ) -> WgpuTensor<E, D1> {
        kernel_wgsl!(
            IndexAssignInplaceRaw,
            "../template/index_assign_inplace.wgsl"
        );

        let tensor = match tensor.can_mut() {
            true => tensor,
            false => tensor.copy(),
        };

        let mut info = build_info(&[&tensor, &value]);

        for i in 0..D1 {
            let start = indexes.get(i).map(|index| index.start).unwrap_or(0);
            info.push(start as u32);
        }

        let info_buffer = tensor
            .context
            .create_buffer_with_data(bytemuck::cast_slice(&info));

        let kernel = tensor
            .context
            .compile_static::<KernelSettings<IndexAssignInplaceRaw, E, i32, 256, 1, 1>>();

        tensor.context.execute(
            WorkGroup::new(
                f32::ceil(value.shape.num_elements() as f32 / 256_f32) as u32,
                1,
                1,
            ),
            kernel,
            &[&tensor.buffer, &value.buffer, &info_buffer],
        );

        tensor
    }

    pub fn equal<E: WgpuElement, const D: usize>(
        lhs: WgpuTensor<E, D>,
        rhs: WgpuTensor<E, D>,
    ) -> WgpuTensor<u32, D> {
        comparison!(Equal, "==");

        comparison::<Equal, E, D>(lhs, rhs)
    }
    pub fn greater<E: WgpuElement, const D: usize>(
        lhs: WgpuTensor<E, D>,
        rhs: WgpuTensor<E, D>,
    ) -> WgpuTensor<u32, D> {
        comparison!(Equal, ">");

        comparison::<Equal, E, D>(lhs, rhs)
    }
    pub fn greater_equal<E: WgpuElement, const D: usize>(
        lhs: WgpuTensor<E, D>,
        rhs: WgpuTensor<E, D>,
    ) -> WgpuTensor<u32, D> {
        comparison!(Equal, ">=");

        comparison::<Equal, E, D>(lhs, rhs)
    }
    pub fn lower<E: WgpuElement, const D: usize>(
        lhs: WgpuTensor<E, D>,
        rhs: WgpuTensor<E, D>,
    ) -> WgpuTensor<u32, D> {
        comparison!(Equal, "<");

        comparison::<Equal, E, D>(lhs, rhs)
    }
    pub fn lower_equal<E: WgpuElement, const D: usize>(
        lhs: WgpuTensor<E, D>,
        rhs: WgpuTensor<E, D>,
    ) -> WgpuTensor<u32, D> {
        comparison!(Equal, "<=");

        comparison::<Equal, E, D>(lhs, rhs)
    }
}
