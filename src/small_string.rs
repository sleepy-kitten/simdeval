use std::{str::from_utf8, borrow::Borrow, hash::Hash};

use crate::stack::Stack;

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum SmallString<const SIZE: usize> {
    Small(Stack<u8, SIZE>),
    String(String),
}
impl<const SIZE: usize> Hash for SmallString<SIZE> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.as_str().hash(state);
    }
}

impl<const SIZE: usize> SmallString<SIZE> {
    pub fn new(string: &str) -> Self {
        let bytes = string.as_bytes();
        if bytes.len() > SIZE {
            Self::String(string.to_string())
        } else {
            Self::Small(bytes.into())
        }
    }
    pub fn as_str(&self) -> &str {
        match self {
            Self::Small(stack) => from_utf8(stack.slice()).unwrap(),
            Self::String(string) => string.as_str(),
        }
    }
}
impl<const SIZE: usize> Borrow<str> for SmallString<SIZE> {
    fn borrow(&self) -> &str {
        match self {
            Self::String(string) => &string,
            Self::Small(small) => from_utf8(small.slice()).unwrap()
        }
    }
}
impl<const SIZE: usize> From<&str> for SmallString<SIZE> {
    fn from(string: &str) -> Self {
        let bytes = string.as_bytes();
        if bytes.len() > SIZE {
            Self::String(string.to_string())
        } else {
            Self::Small(bytes.into())
        }
    }
}