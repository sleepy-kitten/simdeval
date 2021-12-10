use std::{num::NonZeroUsize, slice::Iter, str::FromStr};

use crate::{
    error::SimdevalError,
    lex::{
        token::{Bracket, Operator, Token},
        tokens::Tokens,
    },
};

/// a function trait, intended to be used with enums where each variant represents a function
pub(crate) trait Function<T>
where
    T: Function<T>,
{
    /// prefix for faster parsing of function identifiers
    const NAMESPACE: &'static str;
    /// calls a function out of the possible functions of `T`
    fn call<S: Function<S>>(&self, node: &[Node<S>]) -> Value;
    /// whether a function returns the same solution each time it is called
    /// `true` by default so it can be evaluated during compilation
    /// if set to `true` even though the function is not constant the expression might not behave as expected
    fn is_const(&self) -> bool {
        true
    }
    fn parse(namespaces: &mut Iter<&str>, identifier: &str) -> Result<T, SimdevalError>;
}
#[derive(Debug)]
pub(crate) enum Node<T>
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
        identifier: String,
        index: Option<usize>,
    },
    // not sure if this will never be 0 but it should not be, since the arguments can not come before the function
    Function {
        function: T,
        args: Option<usize>,
    },
    Bracket(Bracket),
}
impl<T: Function<T>> Node<T> {
    pub(crate) fn zero() -> Self {
        Self::Literal(Value::Int(0))
    }
    pub(crate) fn is_instruction(&self) -> bool {
        matches!(self, Self::Instruction { .. })
    }
    pub(crate) fn instruction(operator: Operator, lhs: usize, rhs: usize) -> Self {
        Self::Instruction { operator, lhs, rhs }
    }
    pub(crate) fn variable(identifier: String, index: Option<usize>) -> Self {
        Self::Variable { identifier, index }
    }
    pub(crate) fn function(function: T, args: Option<usize>) -> Self {
        Self::Function { function, args }
    }
    pub(crate) fn as_mut_instruction_indices(&mut self) -> Option<(&mut usize, &mut usize)> {
        if let <Node<T>>::Instruction { operator, lhs, rhs } = self {
            Some((lhs, rhs))
        } else {
            None
        }
    }
    pub(crate) fn weight(&self) -> i16 {
        match self {
            Self::Instruction { operator, .. } => operator.weight(),
            _ => 0,
        }
    }
}

impl<T: Function<T> + FromStr> TryFrom<Token> for Node<T> {
    type Error = SimdevalError;
    fn try_from(value: Token) -> Result<Self, Self::Error> {
        todo!()
    }
}
#[derive(Debug)]
pub(crate) enum Value {
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),
}
pub(crate) struct Variable {
    identifier: String,
    index: usize,
}
#[derive(Debug)]
pub enum Std {
    NaturalLog,
    SquareRoot,
}

impl Function<Std> for Std {
    const NAMESPACE: &'static str = "std";
    fn call<S>(&self, _: &[Node<S>]) -> Value
    where
        S: Function<S>,
    {
        match self {
            Self::NaturalLog => Value::Int(313),
            Self::SquareRoot => Value::Float(2.14),
        }
    }
    fn parse(namespaces: &mut Iter<&str>, identifier: &str) -> Result<Std, SimdevalError> {
        if let Some(next) = namespaces.next() {
            let slice = &next[0..next.len() - 1];
            if slice == Self::NAMESPACE {
                Ok(match identifier {
                    "log" => Self::NaturalLog,
                    "sqrt" => Self::SquareRoot,
                    _ => return Err(SimdevalError::NoIdentifierMatch),
                })
            } else {
                Err(SimdevalError::InvalidNamespace)
            }
        } else {
            Ok(match identifier {
                "log" => Self::NaturalLog,
                "sqrt" => Self::SquareRoot,
                _ => return Err(SimdevalError::NoIdentifierMatch),
            })
        }
    }
}
#[derive(Debug)]
pub enum Test {
    Std(Std),
    Func,
    Sqrt,
}
impl Function<Test> for Test {
    const NAMESPACE: &'static str = "test";
    fn call<S>(&self, node: &[Node<S>]) -> Value
    where
        S: Function<S>,
    {
        match self {
            Test::Std(s) => s.call(node),
            Test::Func => Value::Bool(false),
            Test::Sqrt => Value::Bool(true),
        }
    }
    fn parse(namespaces: &mut Iter<&str>, identifier: &str) -> Result<Test, SimdevalError> {
        if let Some(next) = namespaces.next() {
            let slice = &next[0..next.len() - 1];
            match slice {
                Self::NAMESPACE => Ok(match identifier {
                    "func" => Self::Func,
                    "sqrt" => Self::Sqrt,
                    _ => return Err(SimdevalError::NoIdentifierMatch),
                }),
                Std::NAMESPACE => Ok(Test::Std(Std::parse(namespaces, identifier)?)),

                _ => Err(SimdevalError::InvalidNamespace),
            }
        } else {
            Err(SimdevalError::InvalidNamespace)
        }
    }
}
// 1 * 4 + ( 6 ^ 3 ^ 2 + 4 )
// 2 2 2 1 4 7 7 7 7 7 5 5 4

// * + ^ ^ +
// 2 1 7 7 5
