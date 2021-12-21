use super::enums::{TokenKind, Special};
use std::ops::RangeInclusive;

#[derive(Debug, Clone)]
pub(crate) struct Token {
    token_kind: TokenKind,
    start: usize,
    end: usize,
}

impl<'a> Token {
    pub fn new(token_kind: TokenKind, start: usize) -> Self {
        Token {
            token_kind,
            start,
            end: start + 1,
        }
    }
    pub fn kind(&self) -> TokenKind {
        self.token_kind
    }
    /// Set the token's token kind.
    pub(crate) fn set_kind(&mut self, token_kind: TokenKind) {
        self.token_kind = token_kind;
    }
    pub(crate) fn inc_end(&mut self) {
        self.end += 1;
    }
    pub(crate) fn set_inc(&mut self, token_kind: TokenKind) {
        self.token_kind = token_kind;
        self.end += 1;
    }
    pub(crate) fn new_neg_zero() -> Self {
        Self {
            token_kind: TokenKind::Special(Special::NegZero),
            start: 0,
            end: 0,
        }
    }
    pub(crate) fn slice<'b>(&'a self, string: &'b str) -> &'b str {
        &string[self.start..self.end]
    }
}
