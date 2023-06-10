use super::WorkGroup;
use std::sync::Arc;
use wgpu::{BindGroup, Buffer, CommandEncoder, ComputePipeline};

#[cfg(feature = "async")]
pub use async_server::{AsyncContextServer, ContextTask, CopyBufferTask, ReadBufferTask};
#[cfg(feature = "async")]
pub type ContextServer = AsyncContextServer;

#[cfg(not(feature = "async"))]
pub type ContextServer = SyncContextServer;

pub struct SyncContextServer {
    device: Arc<wgpu::Device>,
    queue: wgpu::Queue,
    encoder: CommandEncoder,
    tasks: Vec<ComputeTask>,
}

impl core::fmt::Debug for SyncContextServer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            format!(
                "SyncContextServer {{ device: {:?} num_tasks: {:?} }}",
                self.device,
                self.tasks.len()
            )
            .as_str(),
        )
    }
}

#[derive(new)]
pub struct ComputeTask {
    bind_group: BindGroup,
    pipeline: Arc<ComputePipeline>,
    work_group: WorkGroup,
}

impl SyncContextServer {
    pub fn new(device: Arc<wgpu::Device>, queue: wgpu::Queue) -> Self {
        let encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Command Encoder"),
        });

        Self {
            device,
            queue,
            encoder,
            tasks: Vec::new(),
        }
    }

    pub fn register_compute(&mut self, task: ComputeTask) {
        self.tasks.push(task)
    }

    fn register_tasks(&mut self) {
        let mut compute = self
            .encoder
            .begin_compute_pass(&wgpu::ComputePassDescriptor { label: None });
        for task in self.tasks.iter() {
            compute.set_pipeline(&task.pipeline);
            compute.set_bind_group(0, &task.bind_group, &[]);
            compute.dispatch_workgroups(task.work_group.x, task.work_group.y, task.work_group.z);
        }
        std::mem::drop(compute);
        self.tasks.clear();
    }

    fn submit(&mut self) {
        assert!(
            self.tasks.is_empty(),
            "Tasks should be completed before submiting the current encoder."
        );
        let mut new_encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        core::mem::swap(&mut new_encoder, &mut self.encoder);

        self.queue.submit(Some(new_encoder.finish()));
    }

    pub fn read(&mut self, buffer: &Buffer) -> Vec<u8> {
        // Register previous tasks before reading the buffer so that it is up to date.
        self.register_tasks();

        let size = buffer.size();
        let buffer_dest = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size,
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        self.encoder
            .copy_buffer_to_buffer(buffer, 0, &buffer_dest, 0, size);

        self.submit();

        let buffer_slice = buffer_dest.slice(..);
        let (sender, receiver) = futures_intrusive::channel::shared::oneshot_channel();
        buffer_slice.map_async(wgpu::MapMode::Read, move |v| {
            sender
                .send(v)
                .expect("Unable to send buffer slice result to async channel.")
        });

        self.device.poll(wgpu::Maintain::Wait);

        let result = pollster::block_on(receiver.receive());

        if let Some(Ok(())) = result {
            let data = buffer_slice.get_mapped_range();
            let result = bytemuck::cast_slice(&data).to_vec();

            drop(data);
            buffer_dest.unmap();
            result
        } else {
            panic!("Unable to read buffer {:?}", result)
        }
    }

    pub fn buffer_to_buffer(&mut self, buffer_src: Arc<Buffer>, buffer_dest: Arc<Buffer>) {
        self.encoder
            .copy_buffer_to_buffer(&buffer_src, 0, &buffer_dest, 0, buffer_src.size());
    }
}

#[cfg(feature = "async")]
mod async_server {
    use crate::context::client::AsyncContextClient;

    use super::{ComputeTask, SyncContextServer};
    use std::sync::{mpsc, Arc};
    use wgpu::Buffer;

    #[derive(new)]
    pub struct ReadBufferTask {
        buffer: Arc<Buffer>,
        sender: mpsc::Sender<Vec<u8>>,
    }

    #[derive(new)]
    pub struct CopyBufferTask {
        pub(crate) buffer_src: Arc<Buffer>,
        pub(crate) buffer_dest: Arc<Buffer>,
    }

    pub enum ContextTask {
        Compute(ComputeTask),
        ReadBuffer(ReadBufferTask),
        CopyBuffer(CopyBufferTask),
    }

    impl From<ComputeTask> for ContextTask {
        fn from(val: ComputeTask) -> Self {
            ContextTask::Compute(val)
        }
    }

    impl From<ReadBufferTask> for ContextTask {
        fn from(val: ReadBufferTask) -> Self {
            ContextTask::ReadBuffer(val)
        }
    }

    impl From<CopyBufferTask> for ContextTask {
        fn from(val: CopyBufferTask) -> Self {
            ContextTask::CopyBuffer(val)
        }
    }

    pub struct AsyncContextServer {
        server: SyncContextServer,
        receiver: mpsc::Receiver<ContextTask>,
    }

    impl AsyncContextServer {
        pub fn start(device: Arc<wgpu::Device>, queue: wgpu::Queue) -> AsyncContextClient {
            let (sender, receiver) = std::sync::mpsc::sync_channel(50);
            let server = SyncContextServer::new(device, queue);
            let context = Self { server, receiver };

            let handle = std::thread::spawn(|| context.run());

            AsyncContextClient::new(sender, handle)
        }

        fn run(mut self) {
            loop {
                let task = self.receiver.recv().unwrap();
                match task {
                    ContextTask::Compute(task) => self.server.register_compute(task),
                    ContextTask::CopyBuffer(task) => self
                        .server
                        .buffer_to_buffer(task.buffer_src, task.buffer_dest),
                    ContextTask::ReadBuffer(task) => {
                        let bytes = self.server.read(&task.buffer);
                        task.sender.send(bytes).unwrap();
                    }
                };

                // Submit the tasks to the GPU when more than 50 tasks are accumulated.
                const MAX_TASKS: usize = 50;

                if self.server.tasks.len() > MAX_TASKS {
                    self.server.register_tasks();
                    self.server.submit();
                }
            }
        }
    }
}

#[cfg(not(feature = "async"))]
mod sync_server {
    use super::SyncContextServer;
    use crate::context::client::SyncContextClient;
    use std::sync::Arc;

    impl SyncContextServer {
        pub fn start(device: Arc<wgpu::Device>, queue: wgpu::Queue) -> SyncContextClient {
            let server = Self::new(device, queue);

            SyncContextClient::new(server)
        }
    }
}
