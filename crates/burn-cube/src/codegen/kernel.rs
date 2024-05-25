use crate::compute::{FullCompilationPhase, Kernel, WorkGroup};
use crate::dialect::Elem;
use crate::pod::CubeElement;
use crate::TensorHandle;
use crate::{elemwise_workgroup, GpuComputeShaderPhase, Runtime, WORKGROUP_DEFAULT};
use burn_compute::client::ComputeClient;
use burn_compute::server::{Binding, Handle};

pub struct BindingSettings<R: Runtime> {
    pub arrays: Vec<Binding<R::Server>>,
    pub array_lengths: Vec<u32>,
    pub info: Vec<u32>,
    pub scalar_bf16: Vec<half::bf16>,
    pub scalar_f16: Vec<half::f16>,
    pub scalar_f32: Vec<f32>,
    pub scalar_f64: Vec<f64>,
    pub scalar_usize: Vec<usize>,
    pub scalar_u64: Vec<u64>,
    pub scalar_u32: Vec<u32>,
    pub scalar_u16: Vec<u16>,
    pub scalar_i32: Vec<i32>,
    pub scalar_i16: Vec<i16>,
}

impl<R: Runtime> BindingSettings<R> {
    pub fn new() -> Self {
        Self {
            arrays: Vec::new(),
            array_lengths: Vec::new(),
            info: Vec::new(),
            scalar_bf16: Vec::new(),
            scalar_f16: Vec::new(),
            scalar_f32: Vec::new(),
            scalar_f64: Vec::new(),
            scalar_usize: Vec::new(),
            scalar_u64: Vec::new(),
            scalar_u32: Vec::new(),
            scalar_u16: Vec::new(),
            scalar_i32: Vec::new(),
            scalar_i16: Vec::new(),
        }
    }
}

pub fn execute_neo<R, K>(
    settings: BindingSettings<R>,
    workgroup: WorkGroup,
    kernel: K,
    client: ComputeClient<R::Server, R::Channel>,
) where
    K: GpuComputeShaderPhase + 'static,
    R: Runtime,
{
    let settings = execute_settings_neo(settings, &client, workgroup);
    let mut handles = settings.handles_tensors;
    let workgroup = settings.workgroup;

    handles.push(settings.handle_info.binding());

    for handle in settings.handles_scalars.into_iter() {
        handles.push(handle.binding());
    }

    let kernel = Kernel::JitGpu(Box::new(FullCompilationPhase::<R::Compiler, K>::new(
        kernel, workgroup,
    )));

    client.execute(kernel, handles);
}

fn execute_settings_neo<R: Runtime>(
    mut settings: BindingSettings<R>,
    client: &ComputeClient<R::Server, R::Channel>,
    workgroup: WorkGroup,
) -> ExecuteSettings<R> {
    if R::require_array_lengths() {
        for len in settings.array_lengths {
            settings.info.push(len);
        }
    }

    let info = client.create(bytemuck::cast_slice(&settings.info));

    let mut handles_scalars = Vec::new();

    if !settings.scalar_bf16.is_empty() {
        handles_scalars.push(client.create(bytemuck::cast_slice(&settings.scalar_bf16)));
    }

    if !settings.scalar_f16.is_empty() {
        handles_scalars.push(client.create(bytemuck::cast_slice(&settings.scalar_bf16)));
    }

    if !settings.scalar_f32.is_empty() {
        handles_scalars.push(client.create(bytemuck::cast_slice(&settings.scalar_f32)));
    }

    if !settings.scalar_f64.is_empty() {
        handles_scalars.push(client.create(bytemuck::cast_slice(&settings.scalar_f64)));
    }

    if !settings.scalar_u64.is_empty() {
        handles_scalars.push(client.create(bytemuck::cast_slice(&settings.scalar_u64)));
    }

    if !settings.scalar_u32.is_empty() {
        handles_scalars.push(client.create(bytemuck::cast_slice(&settings.scalar_u32)));
    }

    if !settings.scalar_u16.is_empty() {
        handles_scalars.push(client.create(bytemuck::cast_slice(&settings.scalar_u16)));
    }

    ExecuteSettings {
        handles_tensors: settings.arrays,
        handle_info: info,
        handles_scalars,
        workgroup,
    }
}

/// The position of the input or output to calculate the number of workgroups to launch.
pub enum WorkgroupLaunch {
    Input { pos: usize },
    Output { pos: usize },
    Custom(WorkGroup),
}

pub struct Execution<'h, K, R: Runtime, Scalars> {
    scalars: Scalars,
    client: ComputeClient<R::Server, R::Channel>,
    kernel: K,
    inputs: &'h [TensorHandle<'h, R>],
    outputs: &'h [TensorHandle<'h, R>],
}

impl<'h, K, R: Runtime> Execution<'h, K, R, ()> {
    pub fn start(
        kernel: K,
        client: ComputeClient<R::Server, R::Channel>,
    ) -> Execution<'h, K, R, ()> {
        Execution {
            scalars: (),
            client,
            kernel,
            inputs: &[],
            outputs: &[],
        }
    }

    #[allow(unused)]
    pub fn inputs(self, inputs: &'h [TensorHandle<'h, R>]) -> Execution<'h, K, R, ()> {
        Execution {
            scalars: self.scalars,
            client: self.client,
            kernel: self.kernel,
            inputs,
            outputs: self.outputs,
        }
    }

    pub fn outputs(self, outputs: &'h [TensorHandle<'h, R>]) -> Execution<'h, K, R, ()> {
        Execution {
            scalars: self.scalars,
            client: self.client,
            kernel: self.kernel,
            inputs: self.inputs,
            outputs,
        }
    }
}

impl<'h, K, R> Execution<'h, K, R, ()>
where
    K: GpuComputeShaderPhase + 'static,
    R: Runtime,
{
    pub fn with_scalars<E>(self, scalars: &[E]) -> Execution<'h, K, R, (&[E],)> {
        Execution {
            scalars: (scalars,),
            client: self.client,
            kernel: self.kernel,
            inputs: self.inputs,
            outputs: self.outputs,
        }
    }
    /// Execute a dynamic kernel.
    #[allow(unused)]
    pub fn execute(self, launch: WorkgroupLaunch) {
        execute_dynamic::<R, K, f32, f32, f32>(
            self.inputs,
            self.outputs,
            None,
            None,
            None,
            self.kernel,
            launch,
            self.client,
        )
    }
}

impl<'h, 'a, K, R, E> Execution<'h, K, R, (&'a [E],)>
where
    K: GpuComputeShaderPhase + 'static,
    R: Runtime,
    E: CubeElement,
{
    pub fn with_scalars<'b, E2>(
        self,
        scalars: &'b [E2],
    ) -> Execution<'h, K, R, (&'a [E], &'b [E2])> {
        Execution {
            scalars: (self.scalars.0, scalars),
            client: self.client,
            kernel: self.kernel,
            inputs: self.inputs,
            outputs: self.outputs,
        }
    }

    /// Execute a dynamic kernel.
    #[allow(unused)]
    pub fn execute(self, launch: WorkgroupLaunch) {
        execute_dynamic::<R, K, E, f32, f32>(
            self.inputs,
            self.outputs,
            Some(self.scalars.0),
            None,
            None,
            self.kernel,
            launch,
            self.client,
        )
    }
}

impl<'h, 'a, 'b, K, R, E1, E2> Execution<'h, K, R, (&'a [E1], &'b [E2])>
where
    K: GpuComputeShaderPhase + 'static,
    R: Runtime,
    E1: CubeElement,
    E2: CubeElement,
{
    #[allow(unused, clippy::type_complexity)]
    pub fn with_scalars<'c, E3>(
        self,
        scalars: &'c [E3],
    ) -> Execution<'h, K, R, (&'a [E1], &'b [E2], &'c [E3])> {
        Execution {
            scalars: (self.scalars.0, self.scalars.1, scalars),
            client: self.client,
            kernel: self.kernel,
            inputs: self.inputs,
            outputs: self.outputs,
        }
    }
    /// Execute a dynamic kernel.
    #[allow(clippy::too_many_arguments)]
    pub fn execute(self, launch: WorkgroupLaunch)
    where
        K: GpuComputeShaderPhase + 'static,
        R: Runtime,
    {
        execute_dynamic::<R, K, E1, E2, f32>(
            self.inputs,
            self.outputs,
            Some(self.scalars.0),
            Some(self.scalars.1),
            None,
            self.kernel,
            launch,
            self.client,
        )
    }
}

impl<'h, 'a, 'b, 'c, K, R, E1, E2, E3> Execution<'h, K, R, (&'a [E1], &'b [E2], &'c [E3])>
where
    K: GpuComputeShaderPhase + 'static,
    R: Runtime,
    E1: CubeElement,
    E2: CubeElement,
    E3: CubeElement,
{
    /// Execute a dynamic kernel.
    #[allow(unused)]
    pub fn execute(self, launch: WorkgroupLaunch) {
        execute_dynamic::<R, K, E1, E2, E3>(
            self.inputs,
            self.outputs,
            Some(self.scalars.0),
            Some(self.scalars.1),
            Some(self.scalars.2),
            self.kernel,
            launch,
            self.client,
        )
    }
}

#[allow(clippy::too_many_arguments)]
fn execute_dynamic<R, K, E1, E2, E3>(
    inputs: &[TensorHandle<R>],
    outputs: &[TensorHandle<R>],
    scalars_1: Option<&[E1]>,
    scalars_2: Option<&[E2]>,
    scalars_3: Option<&[E3]>,
    kernel: K,
    launch: WorkgroupLaunch,
    client: ComputeClient<R::Server, R::Channel>,
) where
    K: GpuComputeShaderPhase + 'static,
    R: Runtime,
    E1: CubeElement,
    E2: CubeElement,
    E3: CubeElement,
{
    let settings = execute_settings(
        inputs, outputs, scalars_1, scalars_2, scalars_3, launch, &client,
    );
    let mut handles = settings.handles_tensors;
    let workgroup = settings.workgroup;

    handles.push(settings.handle_info.binding());
    for handle in settings.handles_scalars.into_iter() {
        handles.push(handle.binding());
    }

    let kernel = Kernel::JitGpu(Box::new(FullCompilationPhase::<R::Compiler, K>::new(
        kernel, workgroup,
    )));

    client.execute(kernel, handles);
}

struct ExecuteSettings<R: Runtime> {
    handles_tensors: Vec<Binding<R::Server>>,
    handle_info: Handle<R::Server>,
    handles_scalars: Vec<Handle<R::Server>>,
    workgroup: WorkGroup,
}

fn execute_settings<'a, R: Runtime, E1: CubeElement, E2: CubeElement, E3: CubeElement>(
    inputs: &'a [TensorHandle<R>],
    outputs: &'a [TensorHandle<R>],
    scalars_1: Option<&[E1]>,
    scalars_2: Option<&[E2]>,
    scalars_3: Option<&[E3]>,
    launch: WorkgroupLaunch,
    client: &ComputeClient<R::Server, R::Channel>,
) -> ExecuteSettings<R> {
    let mut info = Vec::new();
    let mut handles = Vec::with_capacity(inputs.len() + outputs.len() + 2);

    // Inner function to fill the info buffer.
    let mut register_info_tensor = |strides: &[usize], shape: &[usize]| {
        if info.is_empty() {
            info.push(strides.len() as u32);
        }

        for s in strides.iter() {
            info.push(*s as u32);
        }
        for s in shape.iter() {
            info.push(*s as u32);
        }
    };

    let mut num_elems_output = 0;

    // We start by registering the inputs.
    for (i, input) in inputs.iter().enumerate() {
        if let WorkgroupLaunch::Input { pos } = &launch {
            if i == *pos {
                num_elems_output = calculate_num_elems_dyn_rank(input.shape);
            }
        };
        register_info_tensor(input.strides, input.shape);
        handles.push(input.handle.clone().binding());
    }

    // Then we follow with the outputs.
    for (i, output) in outputs.iter().enumerate() {
        if let WorkgroupLaunch::Output { pos } = &launch {
            if i == *pos {
                num_elems_output = calculate_num_elems_dyn_rank(output.shape);
            }
        };
        register_info_tensor(output.strides, output.shape);
        handles.push(output.handle.clone().binding());
    }

    // [2, I0stride0, I0stride1, I0shape0, I0shape1i, I1... O0...,  I0len, I1len1, O0len]
    if R::require_array_lengths() {
        for input in inputs.iter() {
            let len = calculate_num_elems_dyn_rank(input.shape);
            info.push(len as u32);
        }

        for output in outputs.iter() {
            let len = calculate_num_elems_dyn_rank(output.shape);
            info.push(len as u32);
        }
    }

    let info = client.create(bytemuck::cast_slice(&info));

    // Finally we finish with the named bindings.
    let handles_scalars =
        create_scalar_handles::<R, E1, E2, E3>(scalars_1, scalars_2, scalars_3, client);

    let workgroup = match launch {
        WorkgroupLaunch::Custom(workgroup) => workgroup,
        _ => elemwise_workgroup(num_elems_output, WORKGROUP_DEFAULT),
    };

    ExecuteSettings {
        handles_tensors: handles,
        handle_info: info,
        handles_scalars,
        workgroup,
    }
}

fn create_scalar_handles<R: Runtime, E1: CubeElement, E2: CubeElement, E3: CubeElement>(
    scalars_0: Option<&[E1]>,
    scalars_1: Option<&[E2]>,
    scalars_2: Option<&[E3]>,
    client: &ComputeClient<R::Server, R::Channel>,
) -> Vec<Handle<R::Server>> {
    // It is crucial that scalars follow this order: float, int, uint
    let element_priority = |elem: Elem| match elem {
        Elem::Float(_) => 0,
        Elem::Int(_) => 1,
        Elem::UInt => 2,
        Elem::Bool => panic!("Bool scalars are not supported"),
    };
    let scalar_priorities: [usize; 3] = [
        element_priority(E1::cube_elem()),
        element_priority(E2::cube_elem()),
        element_priority(E3::cube_elem()),
    ];

    let mut handles_scalars = Vec::new();
    for i in 0..3 {
        for (j, scalar_priority) in scalar_priorities.iter().enumerate() {
            if scalar_priority == &i {
                if j == 0 {
                    if let Some(values) = &scalars_0 {
                        handles_scalars.push(client.create(bytemuck::cast_slice(values)));
                    }
                } else if j == 1 {
                    if let Some(values) = &scalars_1 {
                        handles_scalars.push(client.create(bytemuck::cast_slice(values)));
                    }
                } else if j == 2 {
                    if let Some(values) = &scalars_2 {
                        handles_scalars.push(client.create(bytemuck::cast_slice(values)));
                    }
                }
            }
        }
    }

    handles_scalars
}

pub fn calculate_num_elems_dyn_rank(shape: &[usize]) -> usize {
    let mut num_elems = 1;
    for i in shape.iter() {
        num_elems *= i;
    }
    num_elems
}
