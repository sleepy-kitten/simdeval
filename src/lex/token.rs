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
    #[inline]
    pub(crate) fn try_to_node<T>(
        &self,
        namespaces: &mut Iter<&str>,
        slice: &str,
    ) -> Result<Node<T>, SimdevalError>
    where
        T: Function<T>,
    {
        //return Ok(Node::Literal(Value::Int(5)));
        Ok(match self.kind {
            TokenKind::Literal(l) => {
                let value = match l {
                    Literal::Float => Value::Float(slice.parse::<f64>()?),
                    Literal::Int => Value::Int(slice.parse::<u64>()?),
                    //Literal::String => Value::String(slice.to_owned()),
                };
                Node::Literal(value)
            }
            TokenKind::Operator(o) => Node::operator(o, None, None),
            TokenKind::Identifier(i) => match i {
                Identifier::Function => {
                    let function: T = <T as Function<T>>::parse(namespaces, slice)?;
                    Node::function(function, None)
                }
                Identifier::Variable => Node::variable(slice.to_owned(), None),
            },
            TokenKind::Space => return Err(SimdevalError::UnexpectedToken),
            _ => return Err(SimdevalError::UnexpectedToken),
        })
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
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
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
