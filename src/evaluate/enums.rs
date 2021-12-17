use std::{
    ops::{Add, Div, Mul, Rem, Sub},
    simd::{LaneCount, SupportedLaneCount},
};

use super::value::Value;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum TokenKind {
    Literal(Literal),
    Bracket(Bracket),
    Identifier(Identifier),
    Operator(Operator),
    Special(Special),
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Literal {
    Int,
    Float,
    Bool,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Bracket {
    Opened,
    Closed,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Identifier {
    Function,
    Variable,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,

    Not,
    Equal,
    And,
    Or,
    Xor,
    NotEqual,
    Greater,
    GreaterEqual,
    Smaller,
    SmallerEqual,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Special {
    Namespace,
    NegZero,
    Comma,
}

impl Bracket {
    pub fn weight(&self) -> i16 {
        match self {
            Self::Opened => 100,
            Self::Closed => -100,
        }
    }
}
impl Operator {
    pub(crate) fn weight(&self) -> i16 {
        match self {
            Self::Add => 20,
            Self::Sub => 20,
            Self::Mul => 21,
            Self::Div => 21,
            Self::Mod => 22,
            Self::Pow => 22,

            Self::Not => 1,
            Self::Equal => 2,
            Self::And => 3,
            Self::Or => 4,
            Self::Xor => 5,
            Self::NotEqual => 7,
            Self::Greater => 8,
            Self::GreaterEqual => 9,
            Self::Smaller => 10,
            Self::SmallerEqual => 11,
        }
    }
    pub(crate) fn eval<const LANES: usize>(
        &self,
        lhs: Value<LANES>,
        rhs: Value<LANES>,
    ) -> Value<LANES>
    where
        LaneCount<LANES>: SupportedLaneCount,
    {
        match self {
            Self::Add => lhs + rhs,
            Self::Sub => lhs - rhs,
            Self::Mul => lhs * rhs,
            Self::Div => lhs / rhs,
            Self::Mod => lhs % rhs,
            Self::Pow => lhs.pow(rhs),
            _ => todo!(),
            /*
            Self::Equal => Single::Bool(lhs == rhs),
            Self::NotEqual => Single::Bool(lhs != rhs),
            Self::Greater => Single::Bool(lhs > rhs),
            Self::GreaterEqual => Single::Bool(lhs >= rhs),
            Self::Smaller => Single::Bool(lhs < rhs),
            Self::SmallerEqual => Single::Bool(lhs <= rhs),
            Self::And => lhs.and(rhs),
            Self::Or => lhs.or(rhs),
            Self::Xor => lhs.xor(rhs),
            _ => unreachable!(),
            */
        }
    }
}
