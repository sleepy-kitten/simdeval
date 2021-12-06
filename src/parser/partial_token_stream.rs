use std::{str::FromStr, fmt::Debug};

use super::partial_token::PartialToken;
use crate::error::SimdevalError;
#[derive(Debug)]
pub(crate) struct PartialTokenStream {
    stream: Vec<PartialToken>,
    index: usize,
}
impl Iterator for PartialTokenStream {
    type Item = PartialToken;
    fn next(&mut self) -> Option<Self::Item> {
        let temp = self.stream.get(self.index);
        self.index += 1;
        temp.copied()
    }
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.stream.get(n).copied()
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.index, Some(self.stream.len()))
    }
}

impl FromIterator<PartialToken> for PartialTokenStream {
    fn from_iter<T: IntoIterator<Item = PartialToken>>(iter: T) -> Self {
        Self {
            stream: FromIterator::from_iter(iter),
            index: 0,
        }
    }
}

impl FromStr for PartialTokenStream {
    type Err = SimdevalError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.as_bytes()
            .iter()
            .map(|c| PartialToken::try_from(*c))
            .collect::<Result<PartialTokenStream, SimdevalError>>()
    }
}
