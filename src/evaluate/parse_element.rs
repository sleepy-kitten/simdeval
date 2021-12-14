use crate::error::SimdevalError;

use super::{
    enums::{Identifier, Literal, TokenKind, Value},
    function::Function,
    node::Node,
    token::Token,
};
use std::{fmt::Debug, slice::Iter};

#[derive(Debug, Clone)]
pub(crate) enum ParseElement<'a, T>
where
    T: Function<T>,
{
    Token(Token),
    Node(Node<'a, T>),
}

impl<'a, T> ParseElement<'a, T>
where
    T: Function<T> + Clone + Debug,
{
    pub(crate) fn to_node_clone(
        &mut self,
        namespaces: &mut Iter<&str>,
        string: &str,
    ) -> Result<(), SimdevalError> {
        if let Self::Token(token) = self.clone() {
            match token.kind() {
                TokenKind::Literal(Literal::Float) => {
                    let value = Value::Float(token.slice(string).parse::<f64>()?);
                    *self = Self::Node(Node::Literal(value));
                }
                TokenKind::Literal(Literal::Int) => {
                    let value = Value::Int(token.slice(string).parse::<i64>()?);
                    *self = Self::Node(Node::Literal(value));
                }
                TokenKind::Operator(o) => {
                    *self = Self::Node(Node::Instruction {
                        operator: o,
                        lhs: 0,
                        rhs: 0,
                    })
                }
                TokenKind::Identifier(Identifier::Variable) => {
                    *self = Self::Node(Node::Variable { index: 0 })
                }
                TokenKind::Identifier(Identifier::Function) => {
                    let function =
                        <T as Function<T>>::from_string(namespaces, token.slice(string))?;
                    *self = Self::Node(Node::Function {
                        function,
                        args: None,
                    })
                }
                _ => unreachable!(),
            }
        }
        Ok(())
    }
}

impl<'a, T> ParseElement<'a, T>
where
    T: Function<T>,
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
                    let value = Value::Float(token.slice(string).parse::<f64>()?);
                    Self::Node(Node::Literal(value))
                }
                TokenKind::Literal(Literal::Int) => {
                    let value = Value::Int(token.slice(string).parse::<i64>()?);
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
                        <T as Function<T>>::from_string(namespaces, token.slice(string))?;
                    Self::Node(Node::Function {
                        function,
                        args: None,
                    })
                }
                _ => unreachable!(),
            })
        } else {
            Err(SimdevalError::InvalidToken)
        }
    }
}
