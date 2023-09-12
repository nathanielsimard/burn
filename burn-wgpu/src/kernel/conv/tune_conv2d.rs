use crate::{
    benchmark::Benchmark,
    element::{FloatElement, WgpuElement},
    kernel,
    tensor::{WgpuTensor, WgpuTensorDyn},
    tune::{AutoTuneFunction, AutoTuneKey, Execution, KernelFunction, Tunable},
    GraphicsApi, WgpuDevice,
};
use burn_tensor::ops::ConvOptions;
use std::{marker::PhantomData, sync::Arc};

const WORKGROUP_SIZES: [usize; 3] = [8, 16, 32];

struct Conv2dKernel<E> {
    workgroup_size: usize,
    _marker: PhantomData<E>,
}

impl<E: WgpuElement + FloatElement> KernelFunction for Conv2dKernel<E> {
    type Input = (
        WgpuTensorDyn<E>,
        WgpuTensorDyn<E>,
        Option<WgpuTensorDyn<E>>,
        ConvOptions<2>,
    );
    type Output = WgpuTensorDyn<E>;

    fn call(&self, input: Self::Input) -> Self::Output {
        let (input_tensor, weight_tensor, bias, options) = input;
        kernel::conv::conv2d(
            WgpuTensor::<E, 4>::from(input_tensor),
            WgpuTensor::<E, 4>::from(weight_tensor),
            bias.map(|b| b.into()),
            options,
            Some(self.workgroup_size),
        )
        .into()
    }

    fn description(&self) -> String {
        format!("Conv2d with workgroup size {}", self.workgroup_size)
    }
}

#[derive(new)]
struct Conv2dBenchmark<E: WgpuElement, const D: usize> {
    input: WgpuTensor<E, 4>,
    weight: WgpuTensor<E, 4>,
    bias: Option<WgpuTensor<E, 1>>,
    options: ConvOptions<2>,
    num_repeats: usize,
    conv2d: PhantomData<E>,
    func: AutoTuneFunction<
        (
            WgpuTensorDyn<E>,
            WgpuTensorDyn<E>,
            Option<WgpuTensorDyn<E>>,
            ConvOptions<2>,
        ),
        WgpuTensorDyn<E>,
    >,
}

impl<E, const D: usize, G> Benchmark<G> for Conv2dBenchmark<E, D>
where
    E: WgpuElement + FloatElement,
    G: GraphicsApi,
{
    type Args = (
        WgpuTensorDyn<E>,
        WgpuTensorDyn<E>,
        Option<WgpuTensorDyn<E>>,
        ConvOptions<2>,
    );

    fn name(&self) -> String {
        format!("{:?} Convolution", self.input.shape.dims) // update later
    }

    fn num_samples(&self) -> usize {
        self.num_repeats
    }

    fn execute(&self, (input, weight, bias, conv_options): Self::Args) {
        for _ in 0..self.num_repeats {
            self.func.call((
                input.clone(),
                weight.clone(),
                bias.clone(),
                conv_options.clone(),
            ));
        }
    }

    fn prepare(&self, device: &WgpuDevice) -> Self::Args {
        (
            self.input.clone().into(),
            self.weight.clone().into(),
            self.bias.clone().map(|b| b.into()),
            self.options.clone(),
        )
    }
}

/// Choose the best convolution kernel via autotuning.
pub fn tune<G: GraphicsApi, E>(
    input: WgpuTensor<E, 4>,
    weight: WgpuTensor<E, 4>,
    bias: Option<WgpuTensor<E, 1>>,
    options: ConvOptions<2>,
) -> WgpuTensor<E, 4>
where
    E: WgpuElement + FloatElement,
{
    // Create an AutoTuneKey and assign to id
    let id = AutoTuneKey::new(
        vec![input.shape.dims.to_vec(), weight.shape.dims.to_vec()], // this is wrong probably
        format!("conv2d {}", E::type_name()),
        &input.context,
    );

    let context = input.context.clone();
    // let bias_dyn: Option<WgpuTensorDyn<E>> = bias.map(|b| b.into());
    let execution_input: (
        WgpuTensorDyn<E>,
        WgpuTensorDyn<E>,
        Option<WgpuTensorDyn<E>>,
        ConvOptions<2>,
    ) = (
        input.clone().into(),
        weight.clone().into(),
        bias.map(|b| b.into()),
        options,
    );
    let output: WgpuTensorDyn<E> = match context.tuner.execute(&id, execution_input) {
        Execution::Executed(output) => output,
        Execution::NoCacheFound((input, weight, bias, options)) => {
            let tunables = conv2d_candidates::<G, E>(
                input.into(),
                weight.into(),
                bias.map(|b| b.into()),
                options,
            );
            context.tuner.tune(
                id,
                (input, weight, bias, options),
                tunables,
                &context.device,
                &context,
            )
        }
    };

    output.into()
}

type Conv2dTunable<G, E> = Tunable<
    G,
    (
        WgpuTensorDyn<E>,
        WgpuTensorDyn<E>,
        Option<WgpuTensorDyn<E>>,
        ConvOptions<2>,
    ),
    WgpuTensorDyn<E>,
>;
/// Enumerates all convolution compute pipelines that are candidates for autotuning
fn conv2d_candidates<G: GraphicsApi, E>(
    input: WgpuTensor<E, 4>,
    weight: WgpuTensor<E, 4>,
    bias: Option<WgpuTensor<E, 1>>,
    options: ConvOptions<2>,
) -> Vec<Conv2dTunable<G, E>>
where
    E: WgpuElement + FloatElement,
{
    let conv2d_benchmark = |func: AutoTuneFunction<
        (
            WgpuTensorDyn<E>,
            WgpuTensorDyn<E>,
            Option<WgpuTensorDyn<E>>,
            ConvOptions<2>,
        ),
        WgpuTensorDyn<E>,
    >| {
        Tunable::<G, _, _>::new(
            func.clone(),
            Arc::new(Conv2dBenchmark::new(
                input.clone(),
                weight.clone(),
                bias.clone(),
                options.clone(),
                5,
                func.clone(),
            )),
        )
    };

    let mut candidates = Vec::new(); //Vec(Tunable<G, (WgputTensorDyn<...>, ...), ...>)
    for workgroup_size in WORKGROUP_SIZES {
        let kernel = Conv2dKernel {
            workgroup_size,
            _marker: PhantomData,
        };
        let func: AutoTuneFunction<_, _> = Arc::new(kernel);
        candidates.push(conv2d_benchmark(func));
    }

    candidates.into()
}
