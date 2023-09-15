use super::DummyServer;
use burn_compute::channel::MutexComputeChannel;
use burn_compute::client::ComputeClient;
use burn_compute::memory_management::SimpleMemoryManagement;
use burn_compute::storage::BytesStorage;
use burn_compute::Compute;

/// The dummy device.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct DummyDevice;

static COMPUTE: Compute<DummyDevice, DummyServer> = Compute::new();

pub fn get(device: &DummyDevice) -> ComputeClient<DummyServer> {
    COMPUTE.client(device, || {
        let storage = BytesStorage::default();
        let memory_management = SimpleMemoryManagement::never_dealloc(storage);
        let server = DummyServer::new(memory_management);
        let channel = MutexComputeChannel::new(server);

        ComputeClient::new(channel)
    })
}
