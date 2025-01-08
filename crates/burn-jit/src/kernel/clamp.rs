use cubecl::prelude::*;

use crate::{
    element::JitElement,
    kernel::{launch_unary_numeric, NumericUnaryOp, NumericUnaryOpFamily},
    tensor::JitTensor,
    JitRuntime,
};

#[derive(CubeLaunch)]
struct Options<C: Numeric> {
    min_value: C,
    max_value: C,
}

pub(crate) fn clamp<R: JitRuntime, E: JitElement>(
    input: JitTensor<R>,
    min_value: E,
    max_value: E,
) -> JitTensor<R> {
    struct ClampOp;

    #[cube]
    impl<N: Numeric> NumericUnaryOp<N> for ClampOp {
        type Options = Options<N>;

        fn execute(input: Line<N>, options: &Self::Options) -> Line<N> {
            Line::clamp(
                input,
                Line::new(options.min_value),
                Line::new(options.max_value),
            )
        }
    }

    impl NumericUnaryOpFamily for ClampOp {
        type Options<N: Numeric> = Options<N>;
        type Unary<N: Numeric> = Self;
    }

    launch_unary_numeric::<R, E, ClampOp, _>(input, |_| {
        OptionsLaunch::new(ScalarArg::new(min_value), ScalarArg::new(max_value))
    })
}
