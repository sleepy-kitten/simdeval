use std::ops::Index;

use crate::error::SimdevalError;

use super::value::single::Value;

#[derive(Debug)]
pub(crate) struct Variables<'a> {
    identifiers: Vec<&'a str>,
    values: Vec<Value>,
}
impl<'a> Variables<'a> {
    pub fn clear(&mut self) {
        self.identifiers.clear();
        self.values.clear();
    }
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            identifiers: Vec::with_capacity(capacity),
            values: Vec::with_capacity(capacity),
        }
    }
    pub fn push(&mut self, identifier: &'a str) {
        self.identifiers.push(identifier);
        self.values.push(Value::Int(0));
    }
    pub fn set(&mut self, identifier: &'a str, value: Value) -> Result<(), SimdevalError> {
        let index = self
            .identifiers
            .binary_search(&identifier)
            .map_err(|_| SimdevalError::InvalidVariable)?;
        self.values[index] = value;
        Ok(())
    }
    pub fn find_or_set(&mut self, identifier: &'a str) -> usize {
        if let Ok(i) = self.identifiers.binary_search(&identifier) {
            i
        } else {
            self.identifiers.push(identifier);
            self.values.push(Value::Int(0));
            self.identifiers.len() - 1
        }
    }
}

impl<'a> Index<usize> for Variables<'a> {
    type Output = Value;

    fn index(&self, index: usize) -> &Self::Output {
        &self.values[index]
    }
}
