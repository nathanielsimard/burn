use crate::codegen::dialect::gpu;
use burn_tensor::Element;

/// The base element trait for the wgpu backend.
pub trait JitElement:
    burn_tensor::Element + core::fmt::Debug + Send + Sync + 'static + Clone + bytemuck::Pod
where
    Self: Sized,
{
    fn type_name() -> &'static str;
    fn as_bytes(slice: &[Self]) -> &[u8];
    fn from_bytes(bytes: &[u8]) -> &[Self];
    fn gpu_elem() -> gpu::Elem;
}

/// The float element type for the wgpu backend.
pub trait FloatElement: JitElement + Element {}

/// The int element type for the wgpu backend.
pub trait IntElement: JitElement + Element {}

impl JitElement for u32 {
    fn type_name() -> &'static str {
        "u32"
    }
    fn as_bytes(slice: &[Self]) -> &[u8] {
        bytemuck::cast_slice(slice)
    }
    fn from_bytes(bytes: &[u8]) -> &[Self] {
        bytemuck::cast_slice(bytes)
    }
    fn gpu_elem() -> gpu::Elem {
        gpu::Elem::UInt
    }
}

impl JitElement for i32 {
    fn type_name() -> &'static str {
        "i32"
    }
    fn as_bytes(slice: &[Self]) -> &[u8] {
        bytemuck::cast_slice(slice)
    }
    fn from_bytes(bytes: &[u8]) -> &[Self] {
        bytemuck::cast_slice(bytes)
    }
    fn gpu_elem() -> gpu::Elem {
        gpu::Elem::Int
    }
}

impl JitElement for f32 {
    fn type_name() -> &'static str {
        "f32"
    }
    fn as_bytes(slice: &[Self]) -> &[u8] {
        bytemuck::cast_slice(slice)
    }
    fn from_bytes(bytes: &[u8]) -> &[Self] {
        bytemuck::cast_slice(bytes)
    }

    fn gpu_elem() -> gpu::Elem {
        gpu::Elem::Float
    }
}

impl FloatElement for f32 {}
impl IntElement for i32 {}
