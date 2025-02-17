use std::{collections::BTreeMap, marker::PhantomData};

use burn_fusion::stream::Context;
use burn_tensor::DType;
use cubecl::{
    client::ComputeClient,
    prelude::{Sequence, TensorArg},
    CubeElement, Runtime,
};

use super::{HandleInput, HandleOutput, LaunchPlan, TensorView, TraceRunner};
use crate::{
    elem_dtype,
    on_write::{
        ir::{ElemwiseConfig, ElemwiseOp, ElemwisePrecision, GlobalArgsLaunch},
        tensor::{GlobalScalar, GlobalTensorArg},
    },
    CubeFusionHandle,
};

/// Execute a [plan](LaunchPlan) using a [runner](TraceRunner) modifying the [context](Context).
pub struct LaunchPlanExecutor<'a, R: Runtime> {
    scalars: &'a BTreeMap<ElemwisePrecision, u32>,
    views: &'a Vec<TensorView>,
    ops: &'a Vec<ElemwiseOp>,
    _r: PhantomData<R>,
}

#[derive(new)]
pub struct ExecutionError<R: Runtime, Runner: TraceRunner<R>> {
    pub runner_error: Runner::Error,
    pub handles_input: Vec<HandleInput<R>>,
    pub handles_output: Vec<HandleOutput<R>>,
}

impl<'a, R: Runtime> LaunchPlanExecutor<'a, R> {
    pub fn new(
        scalars: &'a BTreeMap<ElemwisePrecision, u32>,
        views: &'a Vec<TensorView>,
        ops: &'a Vec<ElemwiseOp>,
    ) -> Self {
        Self {
            scalars,
            views,
            ops,
            _r: PhantomData,
        }
    }

    pub fn execute<Runner: TraceRunner<R>, BT: CubeElement>(
        self,
        client: &ComputeClient<R::Server, R::Channel>,
        runner: &Runner,
        context: &mut Context<'_, CubeFusionHandle<R>>,
        plan: LaunchPlan<'a, R>,
    ) -> Result<(), ExecutionError<R, Runner>> {
        let reference = match plan.reference {
            Some(reference) => reference,
            None => {
                if plan.writes.is_empty() {
                    // Nothing to write, can skip execution.
                    return Ok(());
                } else {
                    panic!("An output should exist for the fused kernel")
                }
            }
        };

        let inputs = self.register_inputs(context, &plan.handle_inputs);
        let outputs = self.register_outputs::<BT>(&plan.handle_outputs);

        let mut ops = Sequence::<ElemwiseOp>::new();

        for read_ops in plan.reads.into_values() {
            for op in read_ops {
                ops.push(op);
            }
        }

        for op in self.ops.iter() {
            ops.push(op.clone());
        }

        for op in plan.writes.into_values() {
            ops.push(op);
        }

        let config = ElemwiseConfig {
            rank: plan.rank as u32,
            ref_layout: reference.layout,
            ops,
        };

        Runner::run(runner, client, inputs, outputs, &config)
            .map_err(|err| ExecutionError::new(err, plan.handle_inputs, plan.handle_outputs))
    }

    fn register_inputs<'h>(
        &self,
        context: &mut Context<'_, CubeFusionHandle<R>>,
        handle_inputs: &'h [HandleInput<R>],
    ) -> GlobalArgsLaunch<'h, R> {
        let mut inputs = GlobalArgsLaunch::default();

        for hi in handle_inputs.iter() {
            let arg = hi.handle.as_tensor_arg(&hi.global_shape, hi.vectorization);
            inputs
                .tensors
                .push(GlobalTensorArg::new(arg, hi.precision.into_elem()));
        }

        for (precision, count) in self.scalars.iter() {
            for i in 0..(*count as usize) {
                match precision {
                    ElemwisePrecision::F32 => {
                        inputs
                            .scalars
                            .push(GlobalScalar::F32(context.scalar_f32[i]));
                    }
                    ElemwisePrecision::F16 => {
                        inputs
                            .scalars
                            .push(GlobalScalar::F16(context.scalar_f16[i]));
                    }
                    ElemwisePrecision::BF16 => {
                        inputs
                            .scalars
                            .push(GlobalScalar::BF16(context.scalar_bf16[i]));
                    }
                    ElemwisePrecision::I64 => {
                        inputs
                            .scalars
                            .push(GlobalScalar::I64(context.scalar_i64[i]));
                    }
                    ElemwisePrecision::I32 => {
                        inputs
                            .scalars
                            .push(GlobalScalar::I32(context.scalar_i32[i]));
                    }
                    ElemwisePrecision::I16 => {
                        inputs
                            .scalars
                            .push(GlobalScalar::I16(context.scalar_i16[i]));
                    }
                    ElemwisePrecision::I8 => {
                        inputs.scalars.push(GlobalScalar::I8(context.scalar_i8[i]));
                    }
                    ElemwisePrecision::U64 => {
                        inputs
                            .scalars
                            .push(GlobalScalar::U64(context.scalar_u64[i]));
                    }
                    ElemwisePrecision::U32 => {
                        inputs
                            .scalars
                            .push(GlobalScalar::U32(context.scalar_u32[i]));
                    }
                    ElemwisePrecision::U16 => {
                        inputs
                            .scalars
                            .push(GlobalScalar::U16(context.scalar_u16[i]));
                    }
                    ElemwisePrecision::U8 => {
                        inputs.scalars.push(GlobalScalar::U8(context.scalar_u8[i]));
                    }
                    ElemwisePrecision::Bool => todo!(),
                }
            }
        }

        // Reshape values are pushed in reverse in the same scalar buffer for all `u32`
        for relative in self.views.iter().rev() {
            if let TensorView::Reshape { reshaped, .. } = relative {
                let global = context.tensors.get(reshaped).unwrap();

                for shape in global.shape.iter().rev() {
                    inputs.scalars.push(GlobalScalar::U32(*shape as u32));
                }
            }
        }

        inputs
    }

    fn register_outputs<'s, BT: CubeElement>(
        &self,
        handle_outputs: &'s [HandleOutput<R>],
    ) -> GlobalArgsLaunch<'s, R> {
        let mut outputs = GlobalArgsLaunch::default();

        for item in handle_outputs.iter() {
            match item {
                HandleOutput::Alias {
                    input_pos,
                    precision,
                } => {
                    println!("Pusing alias");
                    outputs.tensors.push(GlobalTensorArg::new(
                        TensorArg::alias(*input_pos),
                        precision.into_elem(),
                    ));
                }
                HandleOutput::Owned {
                    precision,
                    handle,
                    global_shape,
                    vectorization,
                    ..
                } => {
                    let arg = handle.as_tensor_arg(global_shape, *vectorization);

                    let elem = match precision {
                        ElemwisePrecision::Bool => match elem_dtype::<BT>() {
                            DType::U32 => ElemwisePrecision::U32.into_elem(),
                            DType::U8 => ElemwisePrecision::U8.into_elem(),
                            _ => todo!(),
                        },
                        _ => precision.into_elem(),
                    };
                    outputs.tensors.push(GlobalTensorArg::new(arg, elem));
                }
            }
        }

        outputs
    }
}
