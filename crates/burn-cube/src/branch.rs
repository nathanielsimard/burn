use std::{ops::Deref, rc::Rc};

use crate::{CubeContext, ExpandElement, UInt};
use burn_jit::gpu::{self, Branch, Elem, Item, Variable};

pub fn range<S, E>(start: S, end: E, _unroll: bool) -> core::ops::Range<usize>
where
    S: Into<UInt>,
    E: Into<UInt>,
{
    let start: UInt = start.into();
    let end: UInt = end.into();

    core::ops::Range {
        start: start.val as usize,
        end: end.val as usize,
    }
}

pub fn range_expand<F>(
    context: &mut CubeContext,
    start: ExpandElement,
    end: ExpandElement,
    unroll: bool,
    mut func: F,
) where
    F: FnMut(&mut CubeContext, ExpandElement),
{
    if unroll {
        let start = match start.deref() {
            Variable::ConstantScalar(val, _) => *val as usize,
            _ => panic!("Only constant start can be unrolled."),
        };
        let end = match end.deref() {
            Variable::ConstantScalar(val, _) => *val as usize,
            _ => panic!("Only constant end can be unrolled."),
        };

        for i in start..end {
            func(context, i.into())
        }
    } else {
        let mut child = context.child();
        let index_ty = Item::Scalar(Elem::UInt);
        let i = child.scope.borrow_mut().create_local_undeclared(index_ty);
        let i = ExpandElement::new(Rc::new(i));

        func(&mut child, i.clone());

        context.register(Branch::RangeLoop(gpu::RangeLoop {
            i: *i,
            start: *start,
            end: *end,
            scope: child.into_scope(),
        }));
    }
}

pub fn if_expand<IF>(context: &mut CubeContext, cond: ExpandElement, mut block: IF)
where
    IF: FnMut(&mut CubeContext),
{
    let mut child = context.child();

    block(&mut child);

    context.register(Branch::If(gpu::If {
        cond: *cond,
        scope: child.into_scope(),
    }));
}

pub fn loop_expand<FC, FB>(context: &mut CubeContext, mut cond_fn: FC, mut block: FB)
where
    FC: FnMut(&mut CubeContext) -> ExpandElement,
    FB: FnMut(&mut CubeContext),
{
    let mut inside_loop = context.child();
    let cond: ExpandElement = cond_fn(&mut inside_loop);

    if_expand(&mut inside_loop, cond, |context| {
        context.register(Branch::Break)
    });

    block(&mut inside_loop);
    context.register(Branch::Loop(gpu::Loop {
        scope: inside_loop.into_scope(),
    }));
}
