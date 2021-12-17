use crate::{error::SimdevalError, stack::Stack};

use super::{
    enums::{Identifier, Literal, TokenKind},
    function::Function,
    node::Node,
    token::Token, value::{single::Single, Value},
};
use std::{fmt::Debug, slice::Iter, simd::{LaneCount, SupportedLaneCount}};

#[derive(Debug)]
pub(crate) enum ParseElement<T, const LANES: usize>
where
    T: Function<T, LANES>,
    LaneCount<LANES>: SupportedLaneCount,
    [(); T::MAX_ARGS]:
{
    Token(Token),
    Node(Node<T, LANES>),
}

impl<T, const LANES: usize> ParseElement<T, LANES>
where
    T: Function<T, LANES>,
    LaneCount<LANES>: SupportedLaneCount,
    [(); T::MAX_ARGS]:
{
    pub(crate) fn get_operands_indices(&self) -> Option<(usize, usize)> {
        if let Self::Node( Node::Instruction{lhs, rhs, ..}) = self {
                Some((*lhs, *rhs))
        } else {
            None
        }
    }
    pub(crate) fn to_node(self, namespaces: &mut Iter<&str>, string: &str) -> Result<Self, SimdevalError> {
        if let Self::Token(token) = self {
            Ok(match token.kind() {
                TokenKind::Literal(Literal::Float) => {
                    let value = Value::Single(Single::Float(token.slice(string).parse::<f64>()?));
                    Self::Node(Node::Literal(value))
                }
                TokenKind::Literal(Literal::Int) => {
                    let value = Value::Single(Single::Int(token.slice(string).parse::<i64>()?));
                    Self::Node(Node::Literal(value))
                }
                TokenKind::Operator(o) => Self::Node(Node::Instruction {
                    operator: o,
                    lhs: 0,
                    rhs: 0,
                }),
                TokenKind::Identifier(Identifier::Variable) => {
                    Self::Node(Node::Variable { index: 0 })
                }
                TokenKind::Identifier(Identifier::Function) => {
                    let function =
                        <T as Function<T, LANES>>::from_string(namespaces, token.slice(string))?;
                    Self::Node(Node::Function {
                        function,
                        args: Stack::new(),
                    })
                }
                _ => unreachable!(),
            })
        } else {
            Err(SimdevalError::InvalidToken)
        }
    }
}
