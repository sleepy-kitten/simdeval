use std::slice::Iter;

use crate::{
    error::SimdevalError,
    parse::node::{Function, Node, Value},
};

/// a `Token` type
/// stores the token kind and the lenght of that token in the source code

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) struct Token {
    kind: TokenKind,
    span: usize,
}

impl Token {
    pub(crate) fn parse_literal<T>(l: Literal, slice: &str) -> Result<Node<T>, SimdevalError>
    where
        T: Function<T>,
    {
        let value = match l {
            Literal::Float => Value::Float(slice.parse::<f64>()?),
            Literal::Int => Value::Int(slice.parse::<i64>()?),
            //Literal::String => Value::String(slice.to_owned()),
        };
        Ok(Node::Literal(value))
    }
    pub(crate) fn parse_operator<T>(o: Operator) -> Result<Node<T>, SimdevalError>
    where
        T: Function<T>,
    {
        let node = Node::instruction(o, 0, 0);
        Ok(node)
    }
    pub(crate) fn parse_separator<T>(s: Separator) -> Result<Node<T>, SimdevalError>
    where
        T: Function<T>,
    {
        if let Separator::Bracket(b) = s {
            let node = Node::Bracket(b);
            Ok(node)
        } else {
            Err(SimdevalError::UnexpectedToken)
        }
    }

    pub(crate) fn parse_identifier<T>(
        i: Identifier,
        namespaces: &mut Iter<&str>,
        slice: &str,
    ) -> Result<Node<T>, SimdevalError>
    where
        T: Function<T>,
    {
        Ok(match i {
            Identifier::Function => {
                let function: T = <T as Function<T>>::parse(namespaces, slice)?;
                Node::function(function, None)
            }
            Identifier::Variable => Node::variable(slice.to_owned(), None),
        })
    }
    pub(crate) fn try_to_node<T>(
        &self,
        namespaces: &mut Iter<&str>,
        slice: &str,
    ) -> Result<Node<T>, SimdevalError>
    where
        T: Function<T>,
    {
        Ok(match self.kind {
            TokenKind::Literal(l) => {
                let value = match l {
                    Literal::Float => Value::Float(slice.parse::<f64>()?),
                    Literal::Int => Value::Int(slice.parse::<i64>()?),
                    //Literal::String => Value::String(slice.to_owned()),
                };
                Node::Literal(value)
            }
            TokenKind::Operator(o) => Node::instruction(o, 0, 0),
            TokenKind::Identifier(i) => match i {
                Identifier::Function => {
                    let function: T = <T as Function<T>>::parse(namespaces, slice)?;
                    Node::function(function, None)
                }
                Identifier::Variable => Node::variable(slice.to_owned(), None),
            },
            TokenKind::Separator(s) => match s {
                Separator::Bracket(b) => Node::Bracket(b),
                _ => unreachable!(),
            },
            TokenKind::Special(_) => return Err(SimdevalError::UnexpectedToken),
            _ => return Err(SimdevalError::UnexpectedToken),
        })
    }
}

impl Token {
    pub(crate) fn new_neg_zero() -> Self {
        Self {
            kind: TokenKind::Special(Special::NegZero),
            span: 0,
        }
    }
    /// Get the token's span.
    pub(crate) fn span(&self) -> usize {
        self.span
    }
    /// constructs a new token
    pub(crate) const fn new(kind: TokenKind) -> Self {
        Self { kind, span: 1 }
    }

    /// Get a reference to the token's kind.
    pub(crate) fn kind(&self) -> &TokenKind {
        &self.kind
    }
    /// increments the span by 1
    pub(crate) fn inc_span(&mut self) {
        self.span += 1
    }

    /// set the kind and increments the span
    pub(crate) fn set_inc(&mut self, kind: TokenKind) {
        self.kind = kind;
        self.inc_span()
    }

    /// Set the token's kind.
    pub(crate) fn set_kind(&mut self, kind: TokenKind) {
        self.kind = kind;
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) enum TokenKind {
    Literal(Literal),
    Operator(Operator),
    Identifier(Identifier),
    Separator(Separator),
    Special(Special),
}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) enum Special {
    Namespace,
    Space,
    NegZero,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) enum Literal {
    //String,
    Int,
    Float,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) enum Separator {
    Comma,
    WavyBracket(Bracket),
    Bracket(Bracket),
    SquareBracket(Bracket),
}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) enum Bracket {
    Opened,
    Closed,
}
impl Bracket {
    pub(crate) fn weight(&self) -> i16 {
        match self {
            Bracket::Opened => 100,
            Bracket::Closed => -100,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) enum Identifier {
    Function,
    Variable,
}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) enum Operator {
    Arithmetic(Arithmetic),
    Logical(Logical),
}
impl Operator {
    pub(crate) fn weight(&self) -> i16 {
        match self {
            Operator::Arithmetic(a) => a.weight(),
            Operator::Logical(l) => l.weight(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) enum Arithmetic {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
}
impl Arithmetic {
    pub(crate) fn weight(&self) -> i16 {
        match self {
            Arithmetic::Add => 20,
            Arithmetic::Sub => 20,
            Arithmetic::Mul => 22,
            Arithmetic::Div => 22,
            Arithmetic::Mod => 22,
            Arithmetic::Pow => 23,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) enum Logical {
    Not,
    Equal,
    And,
    Or,
    Xor,
    NotEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
}
impl Logical {
    pub(crate) fn weight(&self) -> i16 {
        match self {
            Logical::Not => 1,
            Logical::Equal => 2,
            Logical::And => 3,
            Logical::Or => 4,
            Logical::Xor => 5,
            Logical::NotEqual => 6,
            Logical::Greater => 7,
            Logical::GreaterEqual => 8,
            Logical::Less => 9,
            Logical::LessEqual => 10,
        }
    }
}
