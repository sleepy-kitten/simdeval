use std::{num::{ParseFloatError, ParseIntError}, array::TryFromSliceError, str::ParseBoolError};

#[derive(Debug)]
pub enum Error {
    UnkownCharacter(char),
    UnexpectedToken,
    NoIdentifierMatch,
    InvalidToken,
    InvalidNamespace,
    InvalidArgs,
    InvalidVariable,
    NotCompiled,
    AlreadyCompiled,
    InvalidIndex,
    EmptyExpression,
}

impl From<ParseFloatError> for Error {
    fn from(_: ParseFloatError) -> Self {
        Self::InvalidToken
    }
}
impl From<ParseIntError> for Error {
    fn from(_: ParseIntError) -> Self {
        Self::InvalidToken
    }
}
impl From<ParseBoolError> for Error {
    fn from(_: ParseBoolError) -> Self {
        Self::InvalidToken
    }
}

impl From<TryFromSliceError> for Error {
    fn from(_: TryFromSliceError) -> Self {
        Self::InvalidArgs
    }
}
