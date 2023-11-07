use crate::{
    binary_float_ops,
    client::FusionClient,
    graph::{
        BinaryOpsDescription, FloatOpsDescription, NumericOpsDescription, Ops,
        ScalarOpsDescription, TensorOpsDescription,
    },
    ops::binary::binary_ops_shape,
    scalar_float_ops, FusedBackend, Fusion, TensorDescription,
};
use burn_tensor::{
    ops::{BoolTensor, FloatElem, FloatTensor, FullPrecisionBackend, IntTensor, TensorOps},
    Data, Device, Distribution, Reader, Shape,
};
use std::ops::Range;

impl<B: FusedBackend> TensorOps<Self> for Fusion<B> {
    fn from_data<const D: usize>(
        data: Data<FloatElem<Self>, D>,
        device: &Device<Self>,
    ) -> FloatTensor<Self, D> {
        let client = B::client(&device.clone().into());
        let out = client.create_float(data.value, data.shape.dims.into());
        out
    }

    fn random<const D: usize>(
        shape: Shape<D>,
        distribution: Distribution<FloatElem<Self>>,
        device: &Device<Self>,
    ) -> FloatTensor<Self, D> {
        struct RandomOps<const D: usize>;

        impl<const D: usize, B: FusedBackend> Ops<B> for RandomOps<D> {
            type Args = (TensorDescription, Distribution<FloatElem<B>>);

            fn execute(
                &self,
                (out, distribution): &Self::Args,
                handles: &mut crate::HandleContainer<B>,
            ) {
                let shape = Shape::from(out.shape.clone());
                let output: B::TensorPrimitive<D> =
                    B::random(shape, distribution.clone(), &handles.device);
                handles.register_float_tensor(&out.id, output);
            }
        }

        let shape: Vec<usize> = shape.dims.into();
        let client = B::client(&device.clone().into());
        let out = client.create_empty(shape);

        client.register(TensorOpsDescription::FloatOps(FloatOpsDescription::Random(
            (out.to_description_out(), distribution),
            Box::new(RandomOps::<D>),
        )));

        out
    }

    fn shape<const D: usize>(tensor: &FloatTensor<Self, D>) -> Shape<D> {
        tensor.shape()
    }

    fn into_data<const D: usize>(tensor: FloatTensor<Self, D>) -> Reader<Data<FloatElem<Self>, D>> {
        tensor.into_data()
    }

    fn device<const D: usize>(tensor: &FloatTensor<Self, D>) -> Device<Self> {
        tensor.client.device().clone().into()
    }

    fn to_device<const D: usize>(
        tensor: FloatTensor<Self, D>,
        device: &Device<Self>,
    ) -> FloatTensor<Self, D> {
        todo!()
    }

    fn empty<const D: usize>(shape: Shape<D>, device: &Device<Self>) -> FloatTensor<Self, D> {
        let client = B::client(&device.clone().into());
        let out = client.create_empty(shape.dims.into());
        out
    }

    fn add<const D: usize>(
        lhs: FloatTensor<Self, D>,
        rhs: FloatTensor<Self, D>,
    ) -> FloatTensor<Self, D> {
        binary_float_ops!(AddOps, B::add);

        let out = lhs
            .client
            .create_empty(binary_ops_shape(&lhs.shape, &rhs.shape));

        out.client.register(TensorOpsDescription::NumericOpsFloat(
            NumericOpsDescription::Add(
                BinaryOpsDescription {
                    lhs: lhs.into_description(),
                    rhs: rhs.into_description(),
                    out: out.to_description_out(),
                },
                Box::new(AddOps::<D>),
            ),
        ));

        out
    }

    fn add_scalar<const D: usize>(
        lhs: FloatTensor<Self, D>,
        rhs: FloatElem<Self>,
    ) -> FloatTensor<Self, D> {
        scalar_float_ops!(AddOps, B::add_scalar);

        let out = lhs.client.create_empty(lhs.shape.clone());

        out.client.register(TensorOpsDescription::NumericOpsFloat(
            NumericOpsDescription::AddScalar(
                ScalarOpsDescription {
                    lhs: lhs.into_description(),
                    rhs,
                    out: out.to_description_out(),
                },
                Box::new(AddOps::<D>),
            ),
        ));

        out
    }

    fn zeros<const D: usize>(shape: Shape<D>, device: &Device<Self>) -> FloatTensor<Self, D> {
        todo!()
    }

    fn full<const D: usize>(
        shape: Shape<D>,
        fill_value: FloatElem<Self>,
        device: &Device<Self>,
    ) -> FloatTensor<Self, D> {
        todo!()
    }

    fn ones<const D: usize>(shape: Shape<D>, device: &Device<Self>) -> FloatTensor<Self, D> {
        struct OnesOps<const D: usize>;

        impl<const D: usize, B: FusedBackend> Ops<B> for OnesOps<D> {
            type Args = TensorDescription;

            fn execute(&self, out: &Self::Args, handles: &mut crate::HandleContainer<B>) {
                let shape = Shape::from(out.shape.clone());
                let output: B::TensorPrimitive<D> = B::ones(shape, &handles.device);
                handles.register_float_tensor(&out.id, output);
            }
        }

        let shape: Vec<usize> = shape.dims.into();
        let client = B::client(&device.clone().into());
        let out = client.create_empty(shape);

        client.register(TensorOpsDescription::NumericOpsFloat(
            NumericOpsDescription::Ones(out.to_description_out(), Box::new(OnesOps::<D>)),
        ));

        out
    }

    fn sub<const D: usize>(
        lhs: FloatTensor<Self, D>,
        rhs: FloatTensor<Self, D>,
    ) -> FloatTensor<Self, D> {
        binary_float_ops!(SubOps, B::sub);

        let out = lhs
            .client
            .create_empty(binary_ops_shape(&lhs.shape, &rhs.shape));

        out.client.register(TensorOpsDescription::NumericOpsFloat(
            NumericOpsDescription::Sub(
                BinaryOpsDescription {
                    lhs: lhs.into_description(),
                    rhs: rhs.into_description(),
                    out: out.to_description_out(),
                },
                Box::new(SubOps::<D>),
            ),
        ));

        out
    }

    fn sub_scalar<const D: usize>(
        lhs: FloatTensor<Self, D>,
        rhs: FloatElem<Self>,
    ) -> FloatTensor<Self, D> {
        scalar_float_ops!(SubOps, B::sub_scalar);

        let out = lhs.client.create_empty(lhs.shape.clone());

        out.client.register(TensorOpsDescription::NumericOpsFloat(
            NumericOpsDescription::SubScalar(
                ScalarOpsDescription {
                    lhs: lhs.into_description(),
                    rhs,
                    out: out.to_description_out(),
                },
                Box::new(SubOps::<D>),
            ),
        ));

        out
    }

    fn mul<const D: usize>(
        lhs: FloatTensor<Self, D>,
        rhs: FloatTensor<Self, D>,
    ) -> FloatTensor<Self, D> {
        binary_float_ops!(MulOps, B::mul);

        let out = lhs
            .client
            .create_empty(binary_ops_shape(&lhs.shape, &rhs.shape));

        out.client.register(TensorOpsDescription::NumericOpsFloat(
            NumericOpsDescription::Mul(
                BinaryOpsDescription {
                    lhs: lhs.into_description(),
                    rhs: rhs.into_description(),
                    out: out.to_description_out(),
                },
                Box::new(MulOps::<D>),
            ),
        ));

        out
    }

    fn mul_scalar<const D: usize>(
        lhs: FloatTensor<Self, D>,
        rhs: FloatElem<Self>,
    ) -> FloatTensor<Self, D> {
        scalar_float_ops!(MulOps, B::mul_scalar);

        let out = lhs.client.create_empty(lhs.shape.clone());

        out.client.register(TensorOpsDescription::NumericOpsFloat(
            NumericOpsDescription::MulScalar(
                ScalarOpsDescription {
                    lhs: lhs.into_description(),
                    rhs,
                    out: out.to_description_out(),
                },
                Box::new(MulOps::<D>),
            ),
        ));

        out
    }

    fn div<const D: usize>(
        lhs: FloatTensor<Self, D>,
        rhs: FloatTensor<Self, D>,
    ) -> FloatTensor<Self, D> {
        binary_float_ops!(DivOps, B::div);

        let out = lhs
            .client
            .create_empty(binary_ops_shape(&lhs.shape, &rhs.shape));

        out.client.register(TensorOpsDescription::NumericOpsFloat(
            NumericOpsDescription::Div(
                BinaryOpsDescription {
                    lhs: lhs.into_description(),
                    rhs: rhs.into_description(),
                    out: out.to_description_out(),
                },
                Box::new(DivOps::<D>),
            ),
        ));

        out
    }

    fn div_scalar<const D: usize>(
        lhs: FloatTensor<Self, D>,
        rhs: FloatElem<Self>,
    ) -> FloatTensor<Self, D> {
        scalar_float_ops!(DivOps, B::div_scalar);

        let out = lhs.client.create_empty(lhs.shape.clone());

        out.client.register(TensorOpsDescription::NumericOpsFloat(
            NumericOpsDescription::DivScalar(
                ScalarOpsDescription {
                    lhs: lhs.into_description(),
                    rhs,
                    out: out.to_description_out(),
                },
                Box::new(DivOps::<D>),
            ),
        ));

        out
    }

    fn matmul<const D: usize>(
        lhs: FloatTensor<Self, D>,
        rhs: FloatTensor<Self, D>,
    ) -> FloatTensor<Self, D> {
        binary_float_ops!(MatmulOps, B::matmul);

        let mut shape = binary_ops_shape(&lhs.shape, &rhs.shape);

        shape[D - 2] = lhs.shape[D - 2];
        shape[D - 1] = rhs.shape[D - 1];

        let out = lhs.client.create_empty(shape);

        out.client
            .register(TensorOpsDescription::FloatOps(FloatOpsDescription::Matmul(
                BinaryOpsDescription {
                    lhs: lhs.into_description(),
                    rhs: rhs.into_description(),
                    out: out.to_description_out(),
                },
                Box::new(MatmulOps::<D>),
            )));

        out
    }

    fn swap_dims<const D: usize>(
        tensor: FloatTensor<Self, D>,
        dim1: usize,
        dim2: usize,
    ) -> FloatTensor<Self, D> {
        todo!()
    }

    fn reshape<const D1: usize, const D2: usize>(
        tensor: FloatTensor<Self, D1>,
        shape: Shape<D2>,
    ) -> FloatTensor<Self, D2> {
        todo!()
    }

    fn gather<const D: usize>(
        dim: usize,
        tensor: FloatTensor<Self, D>,
        indices: IntTensor<Self, D>,
    ) -> FloatTensor<Self, D> {
        todo!()
    }

    fn scatter<const D: usize>(
        dim: usize,
        tensor: FloatTensor<Self, D>,
        indices: IntTensor<Self, D>,
        value: FloatTensor<Self, D>,
    ) -> FloatTensor<Self, D> {
        todo!()
    }

    fn select<const D: usize>(
        tensor: FloatTensor<Self, D>,
        dim: usize,
        indices: IntTensor<Self, 1>,
    ) -> FloatTensor<Self, D> {
        todo!()
    }

    fn select_assign<const D: usize>(
        tensor: FloatTensor<Self, D>,
        dim: usize,
        indices: IntTensor<Self, 1>,
        value: FloatTensor<Self, D>,
    ) -> FloatTensor<Self, D> {
        todo!()
    }

    fn slice<const D1: usize, const D2: usize>(
        tensor: FloatTensor<Self, D1>,
        ranges: [Range<usize>; D2],
    ) -> FloatTensor<Self, D1> {
        todo!()
    }

    fn slice_assign<const D1: usize, const D2: usize>(
        tensor: FloatTensor<Self, D1>,
        ranges: [Range<usize>; D2],
        value: FloatTensor<Self, D1>,
    ) -> FloatTensor<Self, D1> {
        todo!()
    }

    fn mask_where<const D: usize>(
        tensor: FloatTensor<Self, D>,
        mask: BoolTensor<Self, D>,
        value: FloatTensor<Self, D>,
    ) -> FloatTensor<Self, D> {
        todo!()
    }

    fn mask_fill<const D: usize>(
        tensor: FloatTensor<Self, D>,
        mask: BoolTensor<Self, D>,
        value: FloatElem<Self>,
    ) -> FloatTensor<Self, D> {
        todo!()
    }

    fn equal<const D: usize>(
        lhs: FloatTensor<Self, D>,
        rhs: FloatTensor<Self, D>,
    ) -> BoolTensor<Self, D> {
        todo!()
    }

    fn equal_elem<const D: usize>(
        lhs: FloatTensor<Self, D>,
        rhs: FloatElem<Self>,
    ) -> BoolTensor<Self, D> {
        todo!()
    }

    fn greater<const D: usize>(
        lhs: FloatTensor<Self, D>,
        rhs: FloatTensor<Self, D>,
    ) -> BoolTensor<Self, D> {
        todo!()
    }

    fn greater_elem<const D: usize>(
        lhs: FloatTensor<Self, D>,
        rhs: FloatElem<Self>,
    ) -> BoolTensor<Self, D> {
        todo!()
    }

    fn greater_equal<const D: usize>(
        lhs: FloatTensor<Self, D>,
        rhs: FloatTensor<Self, D>,
    ) -> BoolTensor<Self, D> {
        todo!()
    }

    fn greater_equal_elem<const D: usize>(
        lhs: FloatTensor<Self, D>,
        rhs: FloatElem<Self>,
    ) -> BoolTensor<Self, D> {
        todo!()
    }

    fn lower<const D: usize>(
        lhs: FloatTensor<Self, D>,
        rhs: FloatTensor<Self, D>,
    ) -> BoolTensor<Self, D> {
        todo!()
    }

    fn lower_elem<const D: usize>(
        lhs: FloatTensor<Self, D>,
        rhs: FloatElem<Self>,
    ) -> BoolTensor<Self, D> {
        todo!()
    }

    fn lower_equal<const D: usize>(
        lhs: FloatTensor<Self, D>,
        rhs: FloatTensor<Self, D>,
    ) -> BoolTensor<Self, D> {
        todo!()
    }

    fn lower_equal_elem<const D: usize>(
        lhs: FloatTensor<Self, D>,
        rhs: FloatElem<Self>,
    ) -> BoolTensor<Self, D> {
        todo!()
    }

    fn sum<const D: usize>(tensor: FloatTensor<Self, D>) -> FloatTensor<Self, 1> {
        todo!()
    }

    fn sum_dim<const D: usize>(tensor: FloatTensor<Self, D>, dim: usize) -> FloatTensor<Self, D> {
        todo!()
    }

    fn mean_dim<const D: usize>(tensor: FloatTensor<Self, D>, dim: usize) -> FloatTensor<Self, D> {
        todo!()
    }

    fn to_full_precision<const D: usize>(
        tensor: &FloatTensor<Self, D>,
    ) -> FloatTensor<FullPrecisionBackend<Self>, D> {
        todo!()
    }

    fn from_full_precision<const D: usize>(
        tensor: FloatTensor<FullPrecisionBackend<Self>, D>,
    ) -> FloatTensor<Self, D> {
        todo!()
    }

    fn exp<const D: usize>(lhs: FloatTensor<Self, D>) -> FloatTensor<Self, D> {
        todo!()
    }

    fn log<const D: usize>(tensor: FloatTensor<Self, D>) -> FloatTensor<Self, D> {
        todo!()
    }

    fn log1p<const D: usize>(tensor: FloatTensor<Self, D>) -> FloatTensor<Self, D> {
        todo!()
    }

    fn powf<const D: usize>(lhs: FloatTensor<Self, D>, rhs: f32) -> FloatTensor<Self, D> {
        todo!()
    }

    fn sqrt<const D: usize>(tensor: FloatTensor<Self, D>) -> FloatTensor<Self, D> {
        todo!()
    }

    fn abs<const D: usize>(tensor: FloatTensor<Self, D>) -> FloatTensor<Self, D> {
        todo!()
    }

    fn cos<const D: usize>(tensor: FloatTensor<Self, D>) -> FloatTensor<Self, D> {
        todo!()
    }

    fn sin<const D: usize>(tensor: FloatTensor<Self, D>) -> FloatTensor<Self, D> {
        todo!()
    }

    fn tanh<const D: usize>(tensor: FloatTensor<Self, D>) -> FloatTensor<Self, D> {
        todo!()
    }

    fn erf<const D: usize>(tensor: FloatTensor<Self, D>) -> FloatTensor<Self, D> {
        todo!()
    }

    fn cat<const D: usize>(tensors: Vec<FloatTensor<Self, D>>, dim: usize) -> FloatTensor<Self, D> {
        todo!()
    }

    fn argmax<const D: usize>(tensor: FloatTensor<Self, D>, dim: usize) -> IntTensor<Self, D> {
        todo!()
    }

    fn argmin<const D: usize>(tensor: FloatTensor<Self, D>, dim: usize) -> IntTensor<Self, D> {
        todo!()
    }

    fn into_int<const D: usize>(tensor: FloatTensor<Self, D>) -> IntTensor<Self, D> {
        todo!()
    }

    fn clamp_min<const D: usize>(
        tensor: FloatTensor<Self, D>,
        min: FloatElem<Self>,
    ) -> FloatTensor<Self, D> {
        todo!()
    }

    fn clamp_max<const D: usize>(
        tensor: FloatTensor<Self, D>,
        max: FloatElem<Self>,
    ) -> FloatTensor<Self, D> {
        todo!()
    }

    fn clamp<const D: usize>(
        tensor: FloatTensor<Self, D>,
        min: FloatElem<Self>,
        max: FloatElem<Self>,
    ) -> FloatTensor<Self, D> {
        todo!()
    }
}
