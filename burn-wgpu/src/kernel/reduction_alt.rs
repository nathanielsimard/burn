use super::{build_info, KernelSettings, SourceTemplate, StaticKernelSource, WORKGROUP_DEFAULT};
use crate::{
    compute::{StaticKernel, WorkGroup},
    element::WgpuElement,
    kernel_wgsl,
    tensor::WgpuTensor,
};

kernel_wgsl!(
    ReductionDimSharedMemoryRaw,
    "../template/reduction/reduce_dim_alt.wgsl"
);

pub(crate) struct SumDimSharedMemory;

impl StaticKernelSource for SumDimSharedMemory {
    fn source() -> SourceTemplate {
        ReductionDimSharedMemoryRaw::source()
            .register(
                "shared_size",
                (WORKGROUP_DEFAULT * WORKGROUP_DEFAULT).to_string(),
            )
            .register("initial", 0.0.to_string())
            .register("assign", "shared_memory[local_id] += value; ")
    }
}

/// Execute the sum dim kernel leveraging shared memory
/// Probably more efficient on tensors where the dimension to reduced
/// is much larger than the others
pub fn sum_dim_shared_memory<E: WgpuElement, const D: usize>(
    input: WgpuTensor<E, D>,
    dim: usize,
) -> WgpuTensor<E, D> {
    reduction_dim_shared_memory::<SumDimSharedMemory, E, D>(input, dim)
}

fn reduction_dim_shared_memory<K: StaticKernelSource, E: WgpuElement, const D: usize>(
    input: WgpuTensor<E, D>,
    reduce_dim: usize,
) -> WgpuTensor<E, D> {
    // Set output dimension with reduced dim
    let mut shape_out = input.shape.clone();
    shape_out.dims[reduce_dim] = 1;

    // Create output handle
    let num_elems_output = shape_out.num_elements();
    let handle = input
        .client
        .empty(num_elems_output * core::mem::size_of::<E>());
    let output = WgpuTensor::new(
        input.client.clone(),
        input.device.clone(),
        shape_out.clone(),
        handle,
    );

    let n_workgroups_x = f32::ceil(f32::sqrt(num_elems_output as f32));
    let n_workgroups_y = f32::ceil(num_elems_output as f32 / n_workgroups_x as f32);
    let grid = WorkGroup::new(n_workgroups_x as u32, n_workgroups_y as u32, 1);

    let kernel =
        StaticKernel::<KernelSettings<K, E, i32, WORKGROUP_DEFAULT, WORKGROUP_DEFAULT, 1>>::new(
            grid,
        );

    // Build info
    let mut info = build_info(&[&input, &output]);

    // Reduce groups are elements that are aligned along the reduce dim
    let reduce_group_size = input.shape.dims[reduce_dim];
    let n_invocation_per_workgroup = WORKGROUP_DEFAULT * WORKGROUP_DEFAULT;
    let n_reduce_elements_per_thread =
        f32::ceil(reduce_group_size as f32 / n_invocation_per_workgroup as f32) as u32;

    // Add dimension of reduction and how many reduce elements are treated per thread
    info.push(reduce_dim as u32);
    info.push(n_reduce_elements_per_thread);

    let info_handle = input.client.create(bytemuck::cast_slice(&info));

    input.client.execute(
        Box::new(kernel),
        &[&input.handle, &output.handle, &info_handle],
    );

    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::{ReferenceBackend, TestBackend};
    use burn_tensor::{Distribution, Tensor};

    #[test]
    fn reduction_sum_dim_shared_memory_small() {
        let tensor = Tensor::<TestBackend, 1>::random([700], Distribution::Default);
        let tensor_ref = Tensor::<ReferenceBackend, 1>::from_data(tensor.to_data());
        let reduce_dim = 0;

        let val = Tensor::<TestBackend, 1>::from_primitive(sum_dim_shared_memory(
            tensor.into_primitive(),
            reduce_dim,
        ));
        let val_ref = tensor_ref.sum_dim(reduce_dim);

        val_ref.into_data().assert_approx_eq(&val.into_data(), 3);
    }

    #[test]
    fn reduction_sum_dim_shared_memory_medium() {
        let tensor = Tensor::<TestBackend, 2>::random([6, 1024], Distribution::Default);
        let tensor_ref = Tensor::<ReferenceBackend, 2>::from_data(tensor.to_data());
        let reduce_dim = 1;

        let val = Tensor::<TestBackend, 2>::from_primitive(sum_dim_shared_memory(
            tensor.into_primitive(),
            reduce_dim,
        ));
        let val_ref = tensor_ref.sum_dim(reduce_dim);

        val_ref.into_data().assert_approx_eq(&val.into_data(), 3);
    }

    #[test]
    fn reduction_sum_dim_shared_memory_large() {
        let tensor = Tensor::<TestBackend, 3>::random([4, 1024, 50], Distribution::Default);
        let tensor_ref = Tensor::<ReferenceBackend, 3>::from_data(tensor.to_data());
        let reduce_dim = 2;

        let val = Tensor::<TestBackend, 3>::from_primitive(sum_dim_shared_memory(
            tensor.into_primitive(),
            reduce_dim,
        ));
        let val_ref = tensor_ref.sum_dim(reduce_dim);

        val_ref.into_data().assert_approx_eq(&val.into_data(), 3);
    }
}
