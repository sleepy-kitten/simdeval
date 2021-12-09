use std::{slice::Iter, str::FromStr};

use crate::{
    error::SimdevalError,
    parse::{node::Function, nodes::Nodes},
};

use super::token::{
    Arithmetic, Bracket, Identifier, Literal, Logical, Operator, Separator, Token, TokenKind,
};
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Tokens<'a> {
    tokens: Vec<Token>,
    source: &'a str,
}
impl<'a> Tokens<'a> {
    pub(crate) fn try_to_nodes<T>(&self) -> Result<Nodes<T>, SimdevalError>
    where
        T: Function<T>,
    {
        let mut offset = 0;
        let mut nodes = Nodes::with_capacity(self.len());
        let mut namespaces = Vec::with_capacity(2);

        for token in self.tokens.iter() {
            let slice = self.slice_span(offset, token.span());
            match token.kind() {
                TokenKind::Namespace => namespaces.push(slice),
                TokenKind::Separator(_) => (),
                TokenKind::Space => (),
                _ => {
                    let node = token.to_node::<T>(&mut namespaces.iter(), slice)?;
                    nodes.push(node);
                    offset += token.span();
                    namespaces.clear();
                }
            }
        }
        Ok(nodes)
    }
    fn slice_span(&self, offset: usize, span: usize) -> &str {
        &self.source[offset..offset + span]
    }
    pub(crate) fn iter(&self) -> Iter<Token> {
        self.tokens.iter()
    }
    pub(crate) fn len(&self) -> usize {
        self.tokens.len()
    }
    pub(crate) fn from_string(source: &'a str) -> Result<Self, SimdevalError> {
        let mut token_stream = Tokens::with_capacity(source.len(), source);
        for &chr in source.as_bytes() {
            token_stream.push(chr)?
        }
        token_stream.shrink_to_fit();
        Ok(token_stream)
    }
    fn get_char(&self, index: usize) -> Option<u8> {
        self.source.as_bytes().get(index).copied()
    }
    fn with_capacity(capacity: usize, source: &'a str) -> Self {
        Self {
            tokens: Vec::with_capacity(capacity),
            source,
        }
    }
    fn shrink_to_fit(&mut self) {
        self.tokens.shrink_to_fit()
    }
    fn push(&mut self, chr: u8) -> Result<(), SimdevalError> {
        let token = self.tokens.last_mut();
        if let Some(token) = token {
            match chr {
                b' ' => self.new_space(),
                b':' => match token.kind() {
                    TokenKind::Identifier(_) => token.push(TokenKind::Namespace),
                    _ => self.new_token(chr)?,
                },
                b'0'..=b'9' => match token.kind() {
                    TokenKind::Identifier(_) => token.inc_span(),
                    TokenKind::Literal(_) => token.inc_span(),
                    _ => self.new_token(chr)?,
                },

                b'a'..=b'z' | b'A'..=b'Z' => match token.kind() {
                    TokenKind::Identifier(_) => token.inc_span(),
                    TokenKind::Literal(Literal::String) => token.inc_span(),
                    _ => self.new_token(chr)?,
                },

                b'.' | b',' | b'_' | b'\'' => match (chr, token.kind()) {
                    (b'.', TokenKind::Literal(Literal::Int)) => {
                        token.push(TokenKind::Literal(Literal::Float))
                    }
                    _ => self.new_token(chr)?,
                },

                b'+' | b'-' | b'*' | b'/' | b'%' | b'^' | b'&' | b'|' | b'!' | b'=' | b'<'
                | b'>' | b'#' => match (chr, token.kind()) {
                    (b'>', TokenKind::Operator(Operator::Logical(Logical::Equal))) => token.push(
                        TokenKind::Operator(Operator::Logical(Logical::GreaterEqual)),
                    ),
                    (b'<', TokenKind::Operator(Operator::Logical(Logical::Equal))) => {
                        token.push(TokenKind::Operator(Operator::Logical(Logical::LessEqual)))
                    }
                    (b'!', TokenKind::Operator(Operator::Logical(Logical::Equal))) => {
                        token.push(TokenKind::Operator(Operator::Logical(Logical::NotEqual)))
                    }
                    _ => self.new_token(chr)?,
                },
                b'(' | b')' => match (chr, token.kind()) {
                    (b'(', TokenKind::Identifier(_)) => {
                        token.set_kind(TokenKind::Identifier(Identifier::Function));
                        self.new_token(chr)?;
                    }
                    _ => self.new_token(chr)?,
                },
                _ => return Err(SimdevalError::UnexpectedToken),
            }
        } else {
            self.new_token(chr)?;
        }
        Ok(())
    }
    fn new_space(&mut self) {
        self.tokens.push(Token::new(TokenKind::Space));
    }
    fn new_token(&mut self, chr: u8) -> Result<(), SimdevalError> {
        let token = Token::new(match chr {
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
            //b'\'' => TokenKind::Literal(Literal::String),
            //b'"' => TokenKind::Literal(Literal::String),
            _ => return Err(SimdevalError::UnkownCharacter(chr as char)),
        });
        self.tokens.push(token);
        Ok(())
    }

    /// Get a reference to the token stream's tokens.
    pub(crate) fn tokens(&self) -> &[Token] {
        self.tokens.as_ref()
    }
}
