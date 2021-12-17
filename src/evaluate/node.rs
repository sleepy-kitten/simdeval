use std::simd::{LaneCount, SupportedLaneCount};

use crate::stack::Stack;

use super::{enums::Operator, function::Function, value::{single::Single, Value}};

#[derive(Debug)]
pub(crate) enum Node<T, const LANES: usize>
where
    T: Function<T, LANES>,
    [(); T::MAX_ARGS]:,
    LaneCount<LANES>: SupportedLaneCount
{
    Instruction {
        operator: Operator,
        lhs: usize,
        rhs: usize,
    },
    Literal(Value<LANES>),
    Variable {
        index: usize,
    },
    Function {
        function: T,
        args: Stack<usize, { T::MAX_ARGS }>,
    },
}

impl<'a, T, const LANES: usize> Node<T, LANES>
where
    T: Function<T, LANES>,
    [(); T::MAX_ARGS]:,
    LaneCount<LANES>: SupportedLaneCount
{
    pub(crate) fn weight(&self) -> i16 {
        match self {
            Self::Instruction { operator, .. } => operator.weight(),
            _ => 0,
        }
    }
    #[inline(always)]
    pub(crate) fn as_mut_instruction_indices(&mut self) -> (&mut usize, &mut usize) {
        if let <Node<T, LANES>>::Instruction { operator, lhs, rhs } = self {
            (lhs, rhs)
        } else {
            panic!()
        }
    }
}
