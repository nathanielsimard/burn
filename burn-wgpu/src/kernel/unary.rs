use super::{KernelSettings, StaticKernel};
use crate::{context::WorkGroup, element::WgpuElement, kernel_wgsl, tensor::WgpuTensor};

kernel_wgsl!(UnaryRaw, "../template/unary.wgsl");
kernel_wgsl!(UnaryInplaceRaw, "../template/unary_inplace.wgsl");

/// Creates a unary kernel.
#[macro_export]
macro_rules! unary {
    (
        $struct:ident,
        func $func:expr
    ) => {
        pub struct $struct;

        impl $crate::kernel::StaticKernel for $struct {
            fn source_template() -> $crate::kernel::SourceTemplate {
                let source = $crate::kernel::UnaryRaw::source_template();
                source.register("body", format!("output[id] = {}(input[id]);", $func))
            }
        }
    };
    (
        $struct:ident,
        body $body:expr
    ) => {
        pub struct $struct;

        impl $crate::kernel::StaticKernel for $struct {
            fn source_template() -> $crate::kernel::SourceTemplate {
                $crate::kernel::UnaryRaw::source_template().register("body", $body)
            }
        }
    };
    (
        $struct:ident,
        func $func:expr,
        include $file:expr
    ) => {
        pub struct $struct;

        impl $crate::kernel::StaticKernel for $struct {
            fn source_template() -> $crate::kernel::SourceTemplate {
                $crate::kernel::UnaryRaw::source_template()
                    .register("body", format!("output[id] = {}(input[id]);", $func))
                    .add_template(include_str!($file))
            }
        }
    };
}

/// Creates a unary inplace kernel.
#[macro_export]
macro_rules! unary_inplace {
    (
        $struct:ident,
        func $func:expr
    ) => {
        pub struct $struct;

        impl $crate::kernel::StaticKernel for $struct {
            fn source_template() -> $crate::kernel::SourceTemplate {
                $crate::kernel::UnaryInplaceRaw::source_template()
                    .register("body", format!("input[id] = {}(input[id]);", $func))
            }
        }
    };
    (
        $struct:ident,
        body $body:expr
    ) => {
        pub struct $struct;

        impl $crate::kernel::StaticKernel for $struct {
            fn source_template() -> $crate::kernel::SourceTemplate {
                $crate::kernel::UnaryInplaceRaw::source_template().register("body", $body)
            }
        }
    };
    (
        $struct:ident,
        func $func:expr,
        include $file:expr
    ) => {
        pub struct $struct;

        impl $crate::kernel::StaticKernel for $struct {
            fn source_template() -> $crate::kernel::SourceTemplate {
                $crate::kernel::UnaryInplaceRaw::source_template()
                    .register("body", format!("input[id] = {}(input[id]);", $func))
                    .add_template(include_str!($file))
            }
        }
    };
}

/// Execute a unary kernel using the default settings.
pub fn unary_default<K: StaticKernel, E: WgpuElement, const D: usize>(
    input: WgpuTensor<E, D>,
) -> WgpuTensor<E, D> {
    unary::<K, E, D, 32>(input)
}

/// Execute a unary inplace kernel using the default settings.
pub fn unary_inplace_default<K: StaticKernel, E: WgpuElement, const D: usize>(
    input: WgpuTensor<E, D>,
) -> WgpuTensor<E, D> {
    unary_inplace::<K, E, D, 32>(input)
}

/// Execute a unary inplace kernel using the given WORKGROUP.
pub fn unary_inplace<K: StaticKernel, E: WgpuElement, const D: usize, const WORKGROUP: usize>(
    input: WgpuTensor<E, D>,
) -> WgpuTensor<E, D> {
    let num_elems = input.shape.num_elements();
    let kernel = input
        .context
        .compile_static::<KernelSettings<K, E, i32, WORKGROUP, WORKGROUP, 1>>();

    input.context.execute(
        unary_workgroup(num_elems, WORKGROUP),
        kernel,
        &[&input.buffer],
    );

    input
}

/// Execute a unary kernel using the provided WORKGROUP.
pub fn unary<K: StaticKernel, E: WgpuElement, const D: usize, const WORKGROUP: usize>(
    input: WgpuTensor<E, D>,
) -> WgpuTensor<E, D> {
    let num_elems = input.shape.num_elements();
    let buffer = input
        .context
        .create_buffer(num_elems * core::mem::size_of::<E>());
    let mut output = WgpuTensor::new(input.context.clone(), input.shape, buffer);
    // Since we don't handle the stride inside the kernel, the output tensor have the same strides
    // as the input tensor. It might not be in the default format.
    output.strides = input.strides;

    let kernel = input
        .context
        .compile_static::<KernelSettings<K, E, i32, WORKGROUP, WORKGROUP, 1>>();

    input.context.execute(
        unary_workgroup(num_elems, WORKGROUP),
        kernel,
        &[&input.buffer, &output.buffer],
    );

    output
}

pub(crate) fn unary_workgroup(num_elems: usize, workgroup_size: usize) -> WorkGroup {
    let num_elem_per_invocation = workgroup_size * workgroup_size;
    let workgroups = f32::ceil(num_elems as f32 / num_elem_per_invocation as f32);
    let workgroup_x = f32::ceil(f32::sqrt(workgroups)) as u32;
    let mut workgroup_y = workgroup_x;

    if workgroup_y > 1 {
        let num_total_covered = workgroup_x * workgroup_y * num_elem_per_invocation as u32;
        if num_total_covered as usize - workgroup_size > num_elems {
            workgroup_y -= 1;
        }
    }

    WorkGroup::new(workgroup_x, workgroup_y, 1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::{ReferenceBackend, TestBackend};
    use burn_tensor::{Distribution, Tensor};

    unary!(TestKernel, func "log");
    unary_inplace!(TestKernelInplace, func "log");

    #[test]
    fn unary_should_work_with_multiple_invocations() {
        let tensor = Tensor::<TestBackend, 2>::random([6, 256], Distribution::Standard);
        let tensor_ref = Tensor::<ReferenceBackend, 2>::from_data(tensor.to_data());

        let actual = unary::<TestKernel, _, 2, 16>(tensor.into_primitive());
        let expected = tensor_ref.log();

        expected.into_data().assert_approx_eq(
            &Tensor::<TestBackend, 2>::from_primitive(actual).into_data(),
            3,
        );
    }

    #[test]
    fn unary_inplace_should_work_with_multiple_invocations() {
        let tensor = Tensor::<TestBackend, 2>::random([6, 256], Distribution::Standard);
        let tensor_ref = Tensor::<ReferenceBackend, 2>::from_data(tensor.to_data());

        let actual = unary_inplace::<TestKernelInplace, _, 2, 16>(tensor.into_primitive());
        let expected = tensor_ref.log();

        expected.into_data().assert_approx_eq(
            &Tensor::<TestBackend, 2>::from_primitive(actual).into_data(),
            3,
        );
    }
}
