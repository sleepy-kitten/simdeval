use super::{
    enums::{Operator, Value},
    function::Function,
};

#[derive(Debug, Clone)]
pub(crate) enum Node<'a, T>
where
    T: Function<T>,
{
    Instruction {
        operator: Operator,
        lhs: usize,
        rhs: usize,
    },
    Literal(Value),
    Variable {
        index: usize,
    },
    Function {
        function: T,
        args: Option<&'a [usize]>,
    },
}

impl<'a, T> Node<'a, T>
where
    T: Function<T>,
{
    pub(crate) fn weight(&self) -> i16 {
        match self {
            Self::Instruction { operator, .. } => operator.weight(),
            _ => 0,
        }
    }
    pub(crate) fn as_mut_instruction_indices(&mut self) -> Option<(&mut usize, &mut usize)> {
        if let <Node<T>>::Instruction { operator, lhs, rhs } = self {
            Some((lhs, rhs))
        } else {
            None
        }
    }
}
