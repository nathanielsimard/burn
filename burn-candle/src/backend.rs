use burn_tensor::backend::Backend;

use crate::{element::CandleElement, CandleTensor};

/// The device type for the candle backend.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// The device struct when using the `candle` backend.
///
/// Note that you need to provide the device index when using Cuda.
pub enum CandleDevice {
    /// CPU device.
    Cpu,

    /// Cuda device with the given index. The index is the index of the Cuda device in the list of
    /// all Cuda devices found on the system.
    Cuda(usize),
}

impl From<CandleDevice> for candle_core::Device {
    fn from(device: CandleDevice) -> Self {
        match device {
            CandleDevice::Cpu => candle_core::Device::Cpu,
            CandleDevice::Cuda(ordinal) => candle_core::Device::new_cuda(ordinal).unwrap(),
        }
    }
}

impl From<candle_core::Device> for CandleDevice {
    fn from(device: candle_core::Device) -> Self {
        match device {
            candle_core::Device::Cpu => CandleDevice::Cpu,
            candle_core::Device::Cuda(cuda_device) => CandleDevice::Cuda(0), // TODO replace 0
        }
    }
}

impl Default for CandleDevice {
    fn default() -> Self {
        Self::Cpu
    }
}

/// The Candle backend.
#[derive(Clone, Copy, Default, Debug)]
pub struct CandleBackend<E> {
    _e: E,
}

impl<E: CandleElement> Backend for CandleBackend<E> {
    type Device = CandleDevice;

    type FullPrecisionBackend = CandleBackend<f32>;
    type FullPrecisionElem = f32;

    type TensorPrimitive<const D: usize> = CandleTensor<Self::FloatElem, D>;
    type FloatElem = E;

    type IntTensorPrimitive<const D: usize> = CandleTensor<Self::IntElem, D>;
    type IntElem = u32;
    type BoolTensorPrimitive<const D: usize> = CandleTensor<u32, D>;

    fn ad_enabled() -> bool {
        false
    }

    fn name() -> String {
        "candle".to_string()
    }

    fn seed(seed: u64) {
        todo!()
    }
}
