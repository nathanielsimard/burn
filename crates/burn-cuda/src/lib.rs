extern crate alloc;

use burn_jit::JitBackend;
use cubecl::cuda::CudaRuntime;
pub use cubecl::cuda::CudaDevice;

#[cfg(not(feature = "fusion"))]
pub type Cuda<F = f32, I = i32> = JitBackend<CudaRuntime, F, I>;

#[cfg(feature = "fusion")]
pub type Cuda<F = f32, I = i32> = burn_fusion::Fusion<JitBackend<CudaRuntime, F, I>>;

#[cfg(test)]
mod tests {
    use burn_jit::JitBackend;

    pub type TestRuntime = cubecl::cuda::CudaRuntime;

    burn_jit::testgen_all!();
}
