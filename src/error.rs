use std::{num::{ParseFloatError, ParseIntError}, array::TryFromSliceError, str::ParseBoolError};

#[derive(Debug)]
pub enum SimdevalError {
    UnkownCharacter(char),
    UnexpectedToken,
    NoIdentifierMatch,
    InvalidToken,
    InvalidNamespace,
    InvalidArgs,
    InvalidVariable,
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
impl From<ParseBoolError> for SimdevalError {
    fn from(_: ParseBoolError) -> Self {
        Self::InvalidToken
    }
}

impl From<TryFromSliceError> for SimdevalError {
    fn from(_: TryFromSliceError) -> Self {
        Self::InvalidArgs
    }
}
