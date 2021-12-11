use std::slice::Iter;

use crate::{
    error::SimdevalError,
    parse::{
        node::{Function, Node},
        nodes::Nodes,
    },
    stack::Stack,
};

use super::token::{
    Arithmetic, Bracket, Identifier, Literal, Logical, Operator, Separator, Special, Token,
    TokenKind,
};
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Tokens<'a> {
    tokens: Vec<Token>,
    source: &'a str,
}

impl<'a> Tokens<'a> {
    #[inline]
    pub(crate) fn try_to_nodes<T>(&self) -> Result<Nodes<T>, SimdevalError>
    where
        T: Function<T>,
    {
        let mut nodes = Nodes::with_capacity(self.len());
        let mut offset = 0;
        let mut namespaces = Stack::<&str, 4>::new();
        let mut span;
        for token in self.tokens.iter() {
            span = token.span();
            match token.kind() {
                TokenKind::Special(Special::Space) => offset += span,
                TokenKind::Special(Special::NegZero) => nodes.push(<Node<T>>::zero()),
                TokenKind::Special(Special::Namespace) => {
                    let slice = self.slice_span(offset, span);
                    namespaces.push(slice);
                    offset += span;
                }
                TokenKind::Separator(s) => {
                    let node = Token::parse_separator(*s)?;
                    nodes.push(node);
                    offset += span;
                }
                TokenKind::Operator(o) => {
                    let node = Token::parse_operator(*o)?;
                    nodes.push(node);
                    offset += span;
                }
                TokenKind::Literal(l) => {
                    let slice = self.slice_span(offset, span);
                    let node = Token::parse_literal(*l, slice)?;
                    nodes.push(node);
                    offset += span;
                }
                TokenKind::Identifier(i) => {
                    let slice = self.slice_span(offset, span);
                    let node = Token::parse_identifier(*i, &mut namespaces.iter(), slice)?;
                    nodes.push(node);
                    namespaces.clear();
                    offset += span;
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
    pub(crate) fn try_from_string(source: &'a str) -> Result<Self, SimdevalError> {
        let mut tokens = Tokens::with_capacity(source.len(), source);
        for &chr in source.as_bytes() {
            tokens.push(chr)?
        }
        //token_stream.shrink_to_fit();
        Ok(tokens)
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
        let back_2_not_literal = self
            .tokens
            .get(self.tokens.len().wrapping_sub(2))
            .and_then(|t| {
                if let TokenKind::Literal(_) = t.kind() {
                    Some(t)
                } else {
                    None
                }
            })
            .is_none();
        let token = self.tokens.last_mut();
        if let Some(token) = token {
            match chr {
                b' ' => self.new_space(),
                b':' => match token.kind() {
                    TokenKind::Identifier(_) => {
                        token.set_inc(TokenKind::Special(Special::Namespace))
                    }
                    _ => self.new_token(chr)?,
                },
                b'0'..=b'9' => match token.kind() {
                    TokenKind::Identifier(_) => token.inc_span(),
                    TokenKind::Literal(_) => token.inc_span(),
                    _ => self.new_token(chr)?,
                },

                b'a'..=b'z' | b'A'..=b'Z' => match token.kind() {
                    TokenKind::Identifier(_) => token.inc_span(),
                    //TokenKind::Literal(Literal::String) => token.inc_span(),
                    _ => self.new_token(chr)?,
                },

                b'.' | b',' | b'_' | b'\'' => match (chr, token.kind()) {
                    (b'.', TokenKind::Literal(Literal::Int)) => {
                        token.set_inc(TokenKind::Literal(Literal::Float))
                    }
                    _ => self.new_token(chr)?,
                },

                b'+' | b'-' | b'*' | b'/' | b'%' | b'^' | b'&' | b'|' | b'!' | b'=' | b'<'
                | b'>' | b'#' => match (chr, token.kind()) {
                    (b'-', TokenKind::Operator(_) | TokenKind::Separator(_)) => {
                        self.insert_neg();
                    }
                    (b'>', TokenKind::Operator(Operator::Logical(Logical::Equal))) => token
                        .set_inc(TokenKind::Operator(Operator::Logical(
                            Logical::GreaterEqual,
                        ))),
                    (b'<', TokenKind::Operator(Operator::Logical(Logical::Equal))) => {
                        token.set_inc(TokenKind::Operator(Operator::Logical(Logical::LessEqual)))
                    }
                    (b'!', TokenKind::Operator(Operator::Logical(Logical::Equal))) => {
                        token.set_inc(TokenKind::Operator(Operator::Logical(Logical::NotEqual)))
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
            if chr == b'-' {
                self.insert_neg();
            } else {
                self.new_token(chr)?;
            }
        }
        Ok(())
    }
    fn insert_neg(&mut self) {
        self.new_neg_zero();
        self.new_token_from_kind(TokenKind::Operator(Operator::Arithmetic(Arithmetic::Sub)));
    }
    fn new_neg_zero(&mut self) {
        self.tokens.push(Token::new_neg_zero());
    }
    fn new_space(&mut self) {
        self.tokens
            .push(Token::new(TokenKind::Special(Special::Space)));
    }
    fn new_token_from_kind(&mut self, kind: TokenKind) {
        self.tokens.push(Token::new(kind));
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
            //b'{' => TokenKind::Separator(Separator::WavyBracket(Bracket::Closed)),
            //b'}' => TokenKind::Separator(Separator::WavyBracket(Bracket::Closed)),
            //b'[' => TokenKind::Separator(Separator::SquareBracket(Bracket::Closed)),
            //b']' => TokenKind::Separator(Separator::SquareBracket(Bracket::Closed)),
            b'.' => TokenKind::Literal(Literal::Float),
            //b',' => TokenKind::Separator(Separator::Comma),
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