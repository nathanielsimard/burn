use crate::kernel::{DynamicKernelSource, SourceTemplate, StaticKernelSource};
use alloc::sync::Arc;
use core::marker::PhantomData;

/// Kernel trait with the [source](SourceTemplate) that will be compiled and cached based on the
/// provided id.
///
/// The kernel will be launched with the given [workgroup](WorkGroup).
pub trait Kernel: 'static + Send + Sync {
    /// Source template for the kernel.
    fn source(&self) -> SourceTemplate;
    /// Identifier for the kernel, used for caching kernel compilation.
    fn id(&self) -> String;
    /// Launch information.
    fn workgroup(&self) -> WorkGroup;
}

impl Kernel for Arc<dyn Kernel> {
    fn source(&self) -> SourceTemplate {
        self.as_ref().source()
    }

    fn id(&self) -> String {
        self.as_ref().id()
    }

    fn workgroup(&self) -> WorkGroup {
        self.as_ref().workgroup()
    }
}

impl Kernel for Box<dyn Kernel> {
    fn source(&self) -> SourceTemplate {
        self.as_ref().source()
    }

    fn id(&self) -> String {
        self.as_ref().id()
    }

    fn workgroup(&self) -> WorkGroup {
        self.as_ref().workgroup()
    }
}

/// Provides launch information specifying the number of work groups to be used by a compute shader.
#[derive(new, Clone, Debug)]
pub struct WorkGroup {
    /// Work groups for the x axis.
    pub x: u32,
    /// Work groups for the y axis.
    pub y: u32,
    /// Work groups for the z axis.
    pub z: u32,
}

impl WorkGroup {
    /// Calculate the number of invocations of a compute shader.
    pub fn num_invocations(&self) -> usize {
        (self.x * self.y * self.z) as usize
    }
}

/// Wraps a [dynamic kernel source](DynamicKernelSource) into a [kernel](Kernel) with launch
/// information such as [workgroup](WorkGroup).
#[derive(new)]
pub struct DynamicKernel<K> {
    kernel: K,
    workgroup: WorkGroup,
}

/// Wraps a [static kernel source](StaticKernelSource) into a [kernel](Kernel) with launch
/// information such as [workgroup](WorkGroup).
#[derive(new)]
pub struct StaticKernel<K> {
    workgroup: WorkGroup,
    _kernel: PhantomData<K>,
}

impl<K> Kernel for DynamicKernel<K>
where
    K: DynamicKernelSource + 'static,
{
    fn source(&self) -> SourceTemplate {
        self.kernel.source()
    }

    fn id(&self) -> String {
        self.kernel.id()
    }

    fn workgroup(&self) -> WorkGroup {
        self.workgroup.clone()
    }
}

impl<K> Kernel for StaticKernel<K>
where
    K: StaticKernelSource + 'static,
{
    fn source(&self) -> SourceTemplate {
        K::source()
    }

    fn id(&self) -> String {
        format!("{:?}", core::any::TypeId::of::<K>())
    }

    fn workgroup(&self) -> WorkGroup {
        self.workgroup.clone()
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::{
//         binary,
//         codegen::dialect::gpu::{BinaryOperator, Elem, Operator, Scope},
//         kernel::{KernelSettings, WORKGROUP_DEFAULT},
//         tests::{TestCompiler, TestRuntime},
//         Runtime, WgpuDevice,
//     };
//
//     #[test]
//     fn can_run_kernel() {
//         binary!(
//             operation: |scope: &mut Scope, elem: Elem| Operator::Add(BinaryOperator {
//                 lhs: scope.read_array(0, elem),
//                 rhs: scope.read_array(1, elem),
//                 out: scope.create_local(elem),
//             }),
//             compiler: TestCompiler,
//             elem_in: f32,
//             elem_out: f32
//         );
//
//         let client = TestRuntime::client(&WgpuDevice::default());
//
//         let lhs: Vec<f32> = vec![0., 1., 2., 3., 4., 5., 6., 7.];
//         let rhs: Vec<f32> = vec![10., 11., 12., 6., 7., 3., 1., 0.];
//         let info: Vec<u32> = vec![1, 1, 8, 1, 8, 1, 8];
//
//         let lhs = client.create(bytemuck::cast_slice(&lhs));
//         let rhs = client.create(bytemuck::cast_slice(&rhs));
//         let out = client.empty(core::mem::size_of::<f32>() * 8);
//         let info = client.create(bytemuck::cast_slice(&info));
//
//         type Kernel = KernelSettings<
//             Ops<TestCompiler, f32, f32>,
//             f32,
//             i32,
//             WORKGROUP_DEFAULT,
//             WORKGROUP_DEFAULT,
//             1,
//         >;
//         let kernel = Box::new(StaticKernel::<Kernel>::new(WorkGroup::new(1, 1, 1)));
//
//         client.execute(kernel, &[&lhs, &rhs, &out, &info]);
//
//         let data = client.read(&out).read_sync().unwrap();
//         let output: &[f32] = bytemuck::cast_slice(&data);
//
//         assert_eq!(output, [10., 12., 14., 9., 11., 8., 7., 7.]);
//     }
// }
