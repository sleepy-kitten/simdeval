use std::{slice::SliceIndex, str::FromStr};

use super::partial_token::{ParsePartialTokenError, PartialToken};

pub(crate) struct PartialTokenStream {
    stream: Vec<PartialToken>,
    index: usize,
}

impl Iterator for PartialTokenStream {
    type Item = PartialToken;
    fn next(&mut self) -> Option<Self::Item> {
        let test = self.stream.iter();
        test.next();
        let temp = self.stream.get(self.index);
        self.index += 1;
        temp;
        todo!()
    }
}
pub(crate) struct ParsePartialTokenStreamError;

impl FromStr for PartialTokenStream {
    type Err = ParsePartialTokenError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_ascii() {
            let mut stream = Vec::with_capacity(s.len());
            for c in s.as_bytes() {
                stream.push(PartialToken::try_from(*c)?);
            }
            Ok(PartialTokenStream { stream, index: 0 })
        } else {
            Err(ParsePartialTokenError)
        }
    }
}
