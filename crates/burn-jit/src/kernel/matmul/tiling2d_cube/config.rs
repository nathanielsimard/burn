use burn_cube::frontend::{CubeContext, Init, UInt};

use crate::kernel::matmul::Tiling2dConfig;

impl Init for CubeTiling2dConfig {
    fn init(self, _context: &mut CubeContext) -> Self {
        self
    }
}

#[derive(Debug, Clone, Copy)]
/// Tiling 2D parameters
pub struct CubeTiling2dConfig {
    /// Block size along dimension of lhs
    pub block_size_m: UInt,
    /// Block size along common dimension
    pub block_size_k: UInt,
    /// Block size along dimension of rhs
    pub block_size_n: UInt,
    /// Loop unrolling for outer loop on blocks. Unclear if faster
    pub unroll_block: bool,
    /// Loop unrolling for inner compute loop. Probably slower
    pub unroll_compute: bool,
    /// Loop unrolling for all loops related to vectorization/tile size. Probably faster
    pub unroll_tile: bool,
    /// Bounds must be checked on lhs dimension
    pub check_m_bounds: bool,
    /// Bounds must be checked on common dimension
    pub check_k_bounds: bool,
    /// Bounds must be checked on rhs dimension
    pub check_n_bounds: bool,
    /// Bounds must be checked on shared memory write
    pub check_sm_bounds: bool,
    /// Tile size. Should correspond to vectorization of inputs/outputs/shared memory
    pub tile_size: UInt,
}

impl CubeTiling2dConfig {
    pub fn new(config: &Tiling2dConfig, m: usize, k: usize, n: usize, tile_size: usize) -> Self {
        assert!(config.block_size_k <= config.block_size_m);
        assert!(config.block_size_k <= config.block_size_n);
        assert!(config.block_size_m % tile_size == 0);
        assert!(config.block_size_k % tile_size == 0);
        assert!(config.block_size_n % tile_size == 0);
        assert!(config.grid_x == config.block_size_m / tile_size);
        assert!(config.grid_y == config.block_size_n / tile_size);

        CubeTiling2dConfig {
            block_size_m: UInt::new(config.block_size_m as u32),
            block_size_k: UInt::new(config.block_size_k as u32),
            block_size_n: UInt::new(config.block_size_n as u32),
            unroll_block: config.unroll,
            unroll_compute: false,
            unroll_tile: true,
            check_m_bounds: m % config.block_size_m != 0,
            check_k_bounds: k % config.block_size_k != 0,
            check_n_bounds: n % config.block_size_n != 0,
            check_sm_bounds: config.block_size_k != config.block_size_m
                || config.block_size_k != config.block_size_n,
            tile_size: UInt::new(tile_size as u32),
        }
    }
}
