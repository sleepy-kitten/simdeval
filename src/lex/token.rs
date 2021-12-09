use std::{ops::Range, str::FromStr};

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
    pub(crate) fn parse<T>(&self, start: usize, s: &str) -> Result<Node<T>, SimdevalError>
    where
        T: Function<T> + FromStr,
    {
        Ok(match self.kind {
            TokenKind::Operator(o) => Node::operator(o, 0, 0),
            TokenKind::Literal(l) => {
                let string = std::str::from_utf8(&s.as_bytes()[start..self.span]).unwrap();
                let value;
                match l {
                    Literal::Float => value = Value::Float(string.parse::<f64>()?),
                    Literal::Int => value = Value::Int(string.parse::<u64>()?),
                    Literal::String => value = Value::String(string.to_owned()),
                }
                Node::Literal(l)
            } 
        }); 
    }
}
/*
impl TryFrom<u8> for Token {
    type Error = SimdevalError;
    fn try_from(chr: u8) -> Result<Self, Self::Error> {
        Ok(Token::new(match chr {
            b'0'..=b'9' => TokenKind::Literal(Literal::Int),

            b'a'..=b'z' | b'A'..=b'Z' => TokenKind::Identifier(Identifier::Variable),

            b'+' => TokenKind::Operator(Operator::Arithmetic(Arithmetic::Add)),
            b'-' => TokenKind::Operator(Operator::Arithmetic(Arithmetic::Sub)),
            b'*' => TokenKind::Operator(Operator::Arithmetic(Arithmetic::Mul)),
            b'/' => TokenKind::Operator(Operator::Arithmetic(Arithmetic::Div)),
            b'%' => TokenKind::Operator(Operator::Arithmetic(Arithmetic::Mod)),
            b'^' => TokenKind::Operator(Operator::Arithmetic(Arithmetic::Pow)),

            b'=' => TokenKind::Operator(Operator::Logical(Logical::Equal)),
            b'!' => TokenKind::Operator(Operator::Logical(Logical::Not)),
            b'>' => TokenKind::Operator(Operator::Logical(Logical::Greater)),
            b'<' => TokenKind::Operator(Operator::Logical(Logical::Less)),
            b'&' => TokenKind::Operator(Operator::Logical(Logical::And)),
            b'|' => TokenKind::Operator(Operator::Logical(Logical::Or)),
            b'#' => TokenKind::Operator(Operator::Logical(Logical::Xor)),

            b'(' => TokenKind::Separator(Separator::Bracket(Bracket::Opened)),
            b')' => TokenKind::Separator(Separator::Bracket(Bracket::Closed)),
            b'{' => TokenKind::Separator(Separator::WavyBracket(Bracket::Closed)),
            b'}' => TokenKind::Separator(Separator::WavyBracket(Bracket::Closed)),
            b'[' => TokenKind::Separator(Separator::SquareBracket(Bracket::Closed)),
            b']' => TokenKind::Separator(Separator::SquareBracket(Bracket::Closed)),

            b'.' => TokenKind::Literal(Literal::Float),
            b',' => TokenKind::Separator(Separator::Comma),

            _ => return Err(SimdevalError::UnkownCharacter(chr as char)),
        }))
    }
}
*/

impl Token {
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
    pub(crate) fn push(&mut self, kind: TokenKind) {
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
    Namespace,
    Space,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) enum Literal {
    String,
    Int,
    Float,
}

impl Literal {}
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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) enum Arithmetic {
    Add = 11,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) enum Logical {
    Not = 1,
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
