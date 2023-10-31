use std::{ops::Range, sync::atomic::AtomicU64};

use burn_tensor::{Distribution, Element};

const ID_COUNTER: AtomicU64 = AtomicU64::new(0);

#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct TensorId {
    value: u64,
}

#[derive(Clone, Debug)]
pub struct TensorDefinition {
    pub id: TensorId,
    pub shape: Vec<usize>,
}

impl TensorId {
    pub(crate) fn new() -> Self {
        let id = ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);

        Self { value: id.into() }
    }
}

#[derive(Debug)]
pub enum TensorOps<F: Element, I: Element> {
    BaseOpsFloat(BaseOps<F>),
    BaseOpsInt(BaseOps<I>),
    BaseOpsBool(BaseOps<bool>),
    NumericOpsFloat(NumericOps<F>),
    NumericOpsInt(NumericOps<F>),
    BoolOps(BoolOps),
    IntOps(IntOps),
    FloatOps(FloatOps<F>),
}

#[derive(Debug)]
pub enum FloatOps<E: core::fmt::Debug> {
    Exp {
        tensor: TensorDefinition,
        out: TensorDefinition,
    },
    Log {
        tensor: TensorDefinition,
        out: TensorDefinition,
    },
    Log1p {
        tensor: TensorDefinition,
        out: TensorDefinition,
    },
    Erf {
        tensor: TensorDefinition,
        out: TensorDefinition,
    },
    Powf {
        tensor: TensorDefinition,
        value: E,
        out: TensorDefinition,
    },
    Sqrt {
        tensor: TensorDefinition,
        out: TensorDefinition,
    },
    Cos {
        tensor: TensorDefinition,
        out: TensorDefinition,
    },
    Sin {
        tensor: TensorDefinition,
        out: TensorDefinition,
    },
    Tanh {
        tensor: TensorDefinition,
        out: TensorDefinition,
    },
    IntoInt {
        tensor: TensorDefinition,
        out: TensorDefinition,
    },
    Matmul {
        lhs: TensorDefinition,
        rhs: TensorDefinition,
        out: TensorDefinition,
    },
    Random {
        shape: Vec<usize>,
        distribution: Distribution<E>,
    },
}

#[derive(Debug)]
pub enum BaseOps<E> {
    Empty {
        shape: Vec<usize>,
        out: TensorDefinition,
    },
    Reshape {
        tensor: TensorDefinition,
        shape: Vec<usize>,
        out: TensorDefinition,
    },
    SwapDims {
        tensor: TensorDefinition,
        dim1: usize,
        dim2: usize,
        out: TensorDefinition,
    },
    Slice {
        tensor: TensorDefinition,
        ranges: Vec<Range<usize>>,
        out: TensorDefinition,
    },
    SliceAssign {
        tensor: TensorDefinition,
        ranges: Vec<Range<usize>>,
        values: TensorDefinition,
        out: TensorDefinition,
    },
    FromData {
        value: Vec<E>,
        shape: Vec<usize>,
        out: TensorDefinition,
    },
    Repeat {
        tensor: TensorDefinition,
        dim: usize,
        times: usize,
        shape: Vec<usize>,
        out: TensorDefinition,
    },
    Equal {
        lhs: TensorDefinition,
        rhs: TensorDefinition,
        out: TensorDefinition,
    },
    Cat {
        tensors: Vec<TensorDefinition>,
        dim: usize,
        out: TensorDefinition,
    },
}

#[derive(Debug)]
pub enum NumericOps<E: Element> {
    Add {
        lhs: TensorDefinition,
        rhs: TensorDefinition,
        out: TensorDefinition,
    },
    AddScalar {
        lhs: TensorDefinition,
        rhs: E,
        out: TensorDefinition,
    },
    Sub {
        lhs: TensorDefinition,
        rhs: TensorDefinition,
        out: TensorDefinition,
    },
    SubScalar {
        lhs: TensorDefinition,
        rhs: E,
        out: TensorDefinition,
    },
    Div {
        lhs: TensorDefinition,
        rhs: TensorDefinition,
        out: TensorDefinition,
    },
    DivScalar {
        lhs: TensorDefinition,
        rhs: E,
        out: TensorDefinition,
    },
    Mul {
        lhs: TensorDefinition,
        rhs: TensorDefinition,
        out: TensorDefinition,
    },
    MulScalar {
        lhs: TensorDefinition,
        rhs: E,
        out: TensorDefinition,
    },
    Neg {
        tensor: TensorDefinition,
        out: TensorDefinition,
    },
    Abs {
        tensor: TensorDefinition,
        out: TensorDefinition,
    },
    Zeros {
        shape: Vec<usize>,
        out: TensorDefinition,
    },
    Ones {
        shape: Vec<usize>,
        out: TensorDefinition,
    },
    Full {
        shape: Vec<usize>,
        value: E,
        out: TensorDefinition,
    },
    Mean {
        tensor: TensorDefinition,
        out: TensorDefinition,
    },
    MeanDim {
        tensor: TensorDefinition,
        dim: usize,
        out: TensorDefinition,
    },
    Sum {
        tensor: TensorDefinition,
        out: TensorDefinition,
    },
    SumDim {
        tensor: TensorDefinition,
        dim: usize,
        out: TensorDefinition,
    },
    EqualElem {
        tensor: TensorDefinition,
        elem: E,
        out: TensorDefinition,
    },
    Greater {
        lhs: TensorDefinition,
        rhs: TensorDefinition,
        out: TensorDefinition,
    },
    GreaterElem {
        tensor: TensorDefinition,
        elem: E,
        out: TensorDefinition,
    },
    GreaterEqual {
        lhs: TensorDefinition,
        rhs: TensorDefinition,
        out: TensorDefinition,
    },
    GreaterEqualElem {
        tensor: TensorDefinition,
        elem: E,
        out: TensorDefinition,
    },
    Lower {
        lhs: TensorDefinition,
        rhs: TensorDefinition,
        out: TensorDefinition,
    },
    LowerElem {
        tensor: TensorDefinition,
        elem: E,
        out: TensorDefinition,
    },
    LowerEqual {
        lhs: TensorDefinition,
        rhs: TensorDefinition,
        out: TensorDefinition,
    },
    LowerEqualElem {
        tensor: TensorDefinition,
        elem: E,
        out: TensorDefinition,
    },
    MaskWhere {
        tensor: TensorDefinition,
        mask: TensorDefinition,
        value: TensorDefinition,
        out: TensorDefinition,
    },
    MaskFill {
        tensor: TensorDefinition,
        mask: TensorDefinition,
        value: E,
        out: TensorDefinition,
    },
    Gather {
        tensor: TensorDefinition,
        dim: usize,
        indices: TensorDefinition,
        out: TensorDefinition,
    },
    Scatter {
        tensor: TensorDefinition,
        dim: usize,
        indices: TensorDefinition,
        values: TensorDefinition,
        out: TensorDefinition,
    },
    Select {
        tensor: TensorDefinition,
        dim: usize,
        indices: TensorDefinition,
    },
    SelectAssign {
        tensor: TensorDefinition,
        dim: usize,
        indices: TensorDefinition,
        values: TensorDefinition,
    },
    ArgMax {
        tensor: TensorDefinition,
        dim: usize,
        out: TensorDefinition,
    },
    ArgMin {
        tensor: TensorDefinition,
        dim: usize,
        out: TensorDefinition,
    },
    Max {
        tensor: TensorDefinition,
        out: TensorDefinition,
    },
    MaxDim {
        tensor: TensorDefinition,
        dim: usize,
        out: TensorDefinition,
    },
    MaxDimWithIndices {
        tensor: TensorDefinition,
        dim: usize,
        out: TensorDefinition,
        out_indices: TensorDefinition,
    },
    Min {
        tensor: TensorDefinition,
        out: TensorDefinition,
    },
    MinDim {
        tensor: TensorDefinition,
        dim: usize,
        out: TensorDefinition,
    },
    Clamp {
        tensor: TensorDefinition,
        min: usize,
        max: usize,
        out: TensorDefinition,
    },
    ClampMax {
        tensor: TensorDefinition,
        max: usize,
        out: TensorDefinition,
    },
    ClampMin {
        tensor: TensorDefinition,
        min: usize,
        out: TensorDefinition,
    },
}

#[derive(Debug)]
pub enum IntOps {
    Arange {
        tensor: TensorDefinition,
        range: Range<usize>,
        out: TensorDefinition,
    },
    ArangeStep {
        tensor: TensorDefinition,
        range: Range<usize>,
        step: usize,
        out: TensorDefinition,
    },
    IntoFloat {
        tensor: TensorDefinition,
        out: TensorDefinition,
    },
}

#[derive(Debug)]
pub enum BoolOps {
    IntoFloat {
        tensor: TensorDefinition,
        out: TensorDefinition,
    },
    IntoInt {
        tensor: TensorDefinition,
        out: TensorDefinition,
    },
    Not {
        tensor: TensorDefinition,
        out: TensorDefinition,
    },
}
