use std::num::{ParseFloatError, ParseIntError};

#[derive(Debug)]
pub enum SimdevalError {
    UnkownCharacter(char),
    UnexpectedToken,
    NoIdentifierMatch,
    InvalidToken,
    InvalidNamespace,
}

impl From<ParseFloatError> for SimdevalError {
    fn from(_: ParseFloatError) -> Self {
        Self::InvalidToken
    }
}

impl From<ParseIntError> for SimdevalError {
    fn from(_: ParseIntError) -> Self {
        Self::InvalidToken
    }
}