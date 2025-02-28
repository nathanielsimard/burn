use core::mem::transmute;

use crate::{sharing::UnsafeSharedRef, tensor::NdArrayTensor};

use burn_common::{iter_range_par, run_par};
use burn_tensor::{quantization::QuantizationType, DType, Element, TensorMetadata};
use macerator::VOrd;
use ndarray::{s, Array4};
use nhwc::max_pool2d_nhwc;
use pulp::{Arch, Simd};

use super::{should_use_simd, MinMax};

macro_rules! launch_kernel {
    ($ty: ty, $func: ident, $x: expr, $($arg: expr),*) => {
        match <$ty as Element>::dtype() {
            DType::F64 => Ok(cast($func::<f64>(cast($x), $($arg),*))),
            DType::F32 => Ok(cast($func::<f32>(cast($x), $($arg),*))),
            DType::F16 | DType::BF16 => Err($x), // Once AVX-512 stabilizes we can use f16
            DType::I64 => Ok(cast($func::<i64>(cast($x), $($arg),*))),
            DType::I32 => Ok(cast($func::<i32>(cast($x), $($arg),*))),
            DType::I16 => Ok(cast($func::<i16>(cast($x), $($arg),*))),
            DType::I8 => Ok(cast($func::<i8>(cast($x), $($arg),*))),
            DType::U64 => Ok(cast($func::<u64>(cast($x), $($arg),*))),
            DType::U32 => Ok(cast($func::<u32>(cast($x), $($arg),*))),
            DType::U16 => Ok(cast($func::<u16>(cast($x), $($arg),*))),
            DType::U8 => Ok(cast($func::<u8>(cast($x), $($arg),*))),
            DType::Bool => Ok(cast($func::<u8>(cast($x), $($arg),*))),
            DType::QFloat(scheme) => match scheme.q_type() {
                QuantizationType::QInt8 => Ok(cast($func::<i8>(cast($x), $($arg),*))),
            },
        }
    };
}

pub(crate) fn try_max_pool2d_simd<E: Element>(
    x: NdArrayTensor<E>,
    ksize: [usize; 2],
    stride: [usize; 2],
    padding: [usize; 2],
    dilation: [usize; 2],
) -> Result<NdArrayTensor<E>, NdArrayTensor<E>> {
    let [_, c, _, _] = x.shape().dims();
    if !should_use_simd(c) || x.array.strides()[1] != 1 {
        return Err(x);
    }

    launch_kernel!(E, max_pool2d_nhwc, x, ksize, stride, padding, dilation)
}

fn cast<T, E>(tensor: NdArrayTensor<T>) -> NdArrayTensor<E> {
    unsafe { transmute::<NdArrayTensor<T>, NdArrayTensor<E>>(tensor) }
}

mod nhwc {
    use itertools::Itertools;
    use ndarray::{ArrayView3, ArrayViewMut3, Ix4};
    use seq_macro::seq;

    use macerator::SimdExt;

    use crate::ops::simd::lanes;

    use super::*;

    // Until you can use associated constants as array size, we need to hardcode this.
    // The most common config (x86-v3) has 16 registers, so use half of them for accumulators.
    const BLOCK_REGISTERS: usize = 8;

    pub(crate) fn max_pool2d_nhwc<E: Element + VOrd + MinMax>(
        x: NdArrayTensor<E>,
        kernel_size: [usize; 2],
        stride: [usize; 2],
        padding: [usize; 2],
        dilation: [usize; 2],
    ) -> NdArrayTensor<E> {
        let [kernel_height, kernel_width] = kernel_size;
        let [pad_h, pad_w] = padding;
        let [stride_height, stride_width] = stride;
        let [dilation_height, dilation_width] = dilation;
        let [batch_size, channels, x_height, x_width] = x.shape().dims();
        let lanes = lanes::<E>();

        let ch_block = lanes * BLOCK_REGISTERS;

        let out_height = ((x_height + 2 * pad_h - dilation_height * (kernel_height - 1) - 1)
            / stride_height)
            + 1;
        let out_width =
            ((x_width + 2 * pad_w - dilation_width * (kernel_width - 1) - 1) / stride_width) + 1;

        let mut output = unsafe {
            Array4::<E>::uninit((batch_size, out_height, out_width, channels)).assume_init()
        };
        let unsafe_shared_out = UnsafeSharedRef::new(&mut output);

        let x = x.array.into_dimensionality::<Ix4>().unwrap();
        let x = x.view();
        let x = x.permuted_axes([0, 2, 3, 1]);

        let blocks = channels / ch_block;
        let blocks_end = blocks * ch_block;
        let simd_end = channels / lanes * lanes;
        let simd_unblocked = (simd_end - blocks_end) / lanes;
        let remainder = channels - simd_end;

        run_par!(|| {
            iter_range_par!(0, batch_size * blocks).for_each(|k| unsafe {
                let block = k % blocks;
                let b = k / blocks;

                let output = unsafe_shared_out.get();
                let x = x.slice(s![b, .., .., ..]);
                let out = output.slice_mut(s![b, .., .., ..]);
                loop_blocked(x, out, kernel_size, stride, padding, dilation, block);
            });
            iter_range_par!(0, batch_size * simd_unblocked).for_each(|k| unsafe {
                let ch = (k % simd_unblocked) * lanes + blocks_end;
                let b = k / simd_unblocked;

                let output = unsafe_shared_out.get();
                let x = x.slice(s![b, .., .., ..]);
                let out = output.slice_mut(s![b, .., .., ..]);
                loop_unblocked(x, out, kernel_size, stride, padding, dilation, ch);
            });
            iter_range_par!(0, batch_size * remainder).for_each(|k| unsafe {
                let ch = (k % remainder) + simd_end;
                let b = k / remainder;

                let output = unsafe_shared_out.get();
                let x = x.slice(s![b, .., .., ..]);
                let out = output.slice_mut(s![b, .., .., ..]);
                loop_scalar(x, out, kernel_size, stride, padding, dilation, ch);
            });
        });

        output = output.permuted_axes([0, 3, 1, 2]);

        NdArrayTensor::new(output.into_dyn().into_shared())
    }

    #[allow(clippy::too_many_arguments, clippy::erasing_op, clippy::identity_op)]
    #[inline(always)]
    #[pulp::with_simd(loop_blocked = Arch::new())]
    unsafe fn loop_nhwc_simd_blocked<S: Simd, E: Element + VOrd + MinMax>(
        simd: S,
        x: ArrayView3<'_, E>,
        mut out: ArrayViewMut3<'_, E>,
        kernel_size: [usize; 2],
        stride: [usize; 2],
        padding: [usize; 2],
        dilation: [usize; 2],
        block: usize,
    ) {
        let [kernel_height, kernel_width] = kernel_size;
        let [pad_h, pad_w] = padding;
        let [stride_height, stride_width] = stride;
        let [dilation_height, dilation_width] = dilation;

        let (x_height, x_width, _) = x.dim();
        let (out_height, out_width, _) = out.dim();
        let lanes = E::lanes::<S>();
        let ch_block = lanes * BLOCK_REGISTERS;

        let min = simd.splat(E::MIN);
        // If outside padding area, kernels are guaranteed to be in bounds
        for oh in pad_h..out_height - pad_h {
            for ow in pad_w..out_width - pad_w {
                seq!(N in 0..8 {
                    let mut acc~N = min;
                });
                let ch = block * ch_block;
                let ch_end = ch + ch_block;
                let mut out = out.slice_mut(s![oh, ow, ch..ch_end]);

                for kh in 0..kernel_height {
                    let ih = oh * stride_height + kh * dilation_height - pad_h;

                    for kw in 0..kernel_width {
                        let iw = ow * stride_width + kw * dilation_width - pad_w;
                        let x = x.slice(s![ih, iw, ch..ch_end]);

                        seq!(N in 0..8 {
                            let s~N = simd.vload_unaligned(x.as_ptr().add(N * lanes));
                            acc~N = E::vmax(simd, acc~N, s~N);
                        });
                    }
                }

                seq!(N in 0..8 {
                    simd.vstore_unaligned(out.as_mut_ptr().add(N * lanes), acc~N);
                });
            }
        }

        // Border pixels need bounds checks
        if (pad_h, pad_w) != (0, 0) {
            let v_borders = (0..pad_h)
                .chain(out_height - pad_h..out_height)
                .cartesian_product(0..out_width);
            let h_borders =
                (0..out_height).cartesian_product((0..pad_w).chain(out_width - pad_w..out_width));

            for (oh, ow) in v_borders.chain(h_borders) {
                seq!(N in 0..8 {
                    let mut acc~N = min;
                });
                let ch = block * ch_block;
                let ch_end = ch + ch_block;
                let mut out = out.slice_mut(s![oh, ow, ch..ch_end]);

                for kh in 0..kernel_height {
                    let ih = oh * stride_height + kh * dilation_height;
                    if ih < pad_h || ih >= x_height + pad_h {
                        continue;
                    }
                    let ih = ih - pad_h;

                    for kw in 0..kernel_width {
                        let iw = ow * stride_width + kw * dilation_width;
                        if iw < pad_w || iw >= x_width + pad_w {
                            continue;
                        }
                        let iw = iw - pad_w;

                        let x = x.slice(s![ih, iw, ch..ch_end]);

                        seq!(N in 0..8 {
                            let s~N = simd.vload_unaligned(x.as_ptr().add(N * lanes));
                            acc~N = E::vmax(simd, acc~N, s~N);
                        });
                    }
                }

                seq!(N in 0..8 {
                    simd.vstore_unaligned(out.as_mut_ptr().add(N * lanes), acc~N);
                });
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    #[inline(always)]
    #[pulp::with_simd(loop_unblocked = Arch::new())]
    unsafe fn loop_nhwc_simd_unblocked<S: Simd, E: Element + VOrd + MinMax>(
        simd: S,
        x: ArrayView3<'_, E>,
        mut out: ArrayViewMut3<'_, E>,
        kernel_size: [usize; 2],
        stride: [usize; 2],
        padding: [usize; 2],
        dilation: [usize; 2],
        ch: usize,
    ) {
        let [kernel_height, kernel_width] = kernel_size;
        let [pad_h, pad_w] = padding;
        let [stride_height, stride_width] = stride;
        let [dilation_height, dilation_width] = dilation;

        let (x_height, x_width, _) = x.dim();
        let (out_height, out_width, _) = out.dim();

        for oh in pad_h..out_height - pad_h {
            for ow in pad_w..out_width - pad_w {
                let mut acc = simd.splat(E::MIN);
                let out = &mut out[[oh, ow, ch]];

                for kh in 0..kernel_height {
                    let ih = oh * stride_height + kh * dilation_height - pad_h;

                    for kw in 0..kernel_width {
                        let iw = ow * stride_width + kw * dilation_width - pad_w;
                        let s0 = simd.vload_unaligned(&x[[ih, iw, ch]]);
                        acc = E::vmax(simd, acc, s0);
                    }
                }

                simd.vstore_unaligned(out, acc);
            }
        }

        // Border pixels need bounds checks
        if (pad_h, pad_w) != (0, 0) {
            let v_borders = (0..pad_h)
                .chain(out_height - pad_h..out_height)
                .cartesian_product(0..out_width);
            let h_borders =
                (0..out_height).cartesian_product((0..pad_w).chain(out_width - pad_w..out_width));

            for (oh, ow) in v_borders.chain(h_borders) {
                let mut acc = simd.splat(E::MIN);
                let out = &mut out[[oh, ow, ch]];

                for kh in 0..kernel_height {
                    let ih = oh * stride_height + kh * dilation_height;
                    if ih < pad_h || ih >= x_height + pad_h {
                        continue;
                    }
                    let ih = ih - pad_h;

                    for kw in 0..kernel_width {
                        let iw = ow * stride_width + kw * dilation_width;
                        if iw < pad_w || iw >= x_width + pad_w {
                            continue;
                        }
                        let iw = iw - pad_w;

                        let s0 = simd.vload_unaligned(&x[[ih, iw, ch]]);
                        acc = E::vmax(simd, acc, s0);
                    }
                }

                simd.vstore_unaligned(out, acc);
            }
        }
    }

    unsafe fn loop_scalar<E: Element + MinMax>(
        x: ArrayView3<'_, E>,
        mut out: ArrayViewMut3<'_, E>,
        kernel_size: [usize; 2],
        stride: [usize; 2],
        padding: [usize; 2],
        dilation: [usize; 2],
        ch: usize,
    ) {
        let [kernel_height, kernel_width] = kernel_size;
        let [pad_h, pad_w] = padding;
        let [stride_height, stride_width] = stride;
        let [dilation_height, dilation_width] = dilation;

        let (x_height, x_width, _) = x.dim();
        let (out_height, out_width, _) = out.dim();

        for oh in 0..out_height {
            for ow in 0..out_width {
                let mut acc = E::MIN;

                for kh in 0..kernel_height {
                    let ih = oh * stride_height + kh * dilation_height;
                    if ih < pad_h || ih >= x_height + pad_h {
                        continue;
                    }
                    let ih = ih - pad_h;

                    for kw in 0..kernel_width {
                        let iw = ow * stride_width + kw * dilation_width;
                        if iw < pad_w || iw >= x_width + pad_w {
                            continue;
                        }
                        let iw = iw - pad_w;
                        acc = acc.max(x[[ih, iw, ch]]);
                    }
                }

                out[[oh, ow, ch]] = acc;
            }
        }
    }
}
