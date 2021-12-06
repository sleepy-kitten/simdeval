use std::fmt::Debug;

use crate::error::SimdevalError;

#[derive(Clone, Copy)]
pub(crate) enum PartialToken {
    Digit(u8),
    Letter(u8),
    Delimiter(u8),
    Operator(u8),
    Bracket(u8),
}
impl Debug for PartialToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use PartialToken::*;
        match self {
            Digit(c) => write!(f, "Digit: {}", *c as char),
            Letter(c) => write!(f, "Letter: {}", *c as char),
            Delimiter(c) => write!(f, "Delimiter: {}", *c as char),
            Operator(c) => write!(f, "Operator: {}", *c as char),
            Bracket(c) => write!(f, "Bracket: {}", *c as char),
        }
    }
}
impl TryFrom<u8> for PartialToken {
    type Error = SimdevalError;
    fn try_from(chr: u8) -> Result<Self, Self::Error> {
        Ok(match chr {
            b'0'..=b'9' => PartialToken::Digit(chr),
            b'a'..=b'z' | b'A'..=b'Z' => PartialToken::Letter(chr),
            b'.' | b',' | b'_' | b' ' | b'"' => PartialToken::Delimiter(chr),
            b'{' | b'}' | b'(' | b')' | b'[' | b']' => PartialToken::Bracket(chr),
            b'+' | b'-' | b'*' | b'/' | b'%' | b'^' => PartialToken::Operator(chr),
            b'&' | b'|' | b'!' | b'=' | b'<' | b'>' | b'#' => PartialToken::Operator(chr),
            _ => return Err(SimdevalError::UnkownCharacter)
        })
    }
}

