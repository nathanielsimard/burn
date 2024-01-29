use std::{cmp::min, fmt::Display};

use crate::{compute::WgpuAutotuneKey, fusion::kernel::AutotuneFusionKernel};
use burn_compute::tune::{AutotuneOperation, AutotuneOperationSet};
use serde::{Deserialize, Serialize};

#[derive(new)]
pub struct ElementWiseAutotuneOperationSet {
    key: WgpuAutotuneKey,
    kernel_1: AutotuneFusionKernel,
    kernel_2: AutotuneFusionKernel,
    kernel_default: AutotuneFusionKernel,
}

#[derive(Hash, Eq, PartialEq, Debug, Clone, Serialize, Deserialize)]
/// Autotune key representative of reduce versions
pub struct FusionElemWiseAutotuneKey {
    num_operations: usize,
    anchored_output_shape: Vec<usize>,
}

impl Display for FusionElemWiseAutotuneKey {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(
            format!(
                "Fusion ElemWise - num_operations: {:?} anchored_output_shape: {:?}",
                self.num_operations, self.anchored_output_shape
            )
            .as_str(),
        )
    }
}

impl AutotuneOperationSet<WgpuAutotuneKey> for ElementWiseAutotuneOperationSet {
    fn key(&self) -> WgpuAutotuneKey {
        self.key.clone()
    }

    fn autotunables(&self) -> Vec<Box<dyn burn_compute::tune::AutotuneOperation>> {
        let kernel_1: Box<dyn AutotuneOperation> = self.kernel_1.clone();
        let kernel_2: Box<dyn AutotuneOperation> = self.kernel_2.clone();

        vec![kernel_1, kernel_2]
    }

    fn fastest(self: Box<Self>, _: usize) -> Box<dyn AutotuneOperation> {
        Box::new(self.kernel_default)
    }
}

impl FusionElemWiseAutotuneKey {
    /// Create a matmul autotune key from the input shapes
    pub fn new(num_operations: usize, output_shape: &[usize]) -> Self {
        Self {
            anchored_output_shape: output_shape
                .into_iter()
                .map(|x| anchor(*x, Some(4096)))
                .collect(),
            num_operations,
        }
    }
}

fn anchor(x: usize, max: Option<usize>) -> usize {
    let exp = f32::ceil(f32::log2(x as f32)) as u32;
    let power_of_2 = 2_u32.pow(exp) as usize;
    if let Some(max) = max {
        min(power_of_2, max)
    } else {
        power_of_2
    }
}
