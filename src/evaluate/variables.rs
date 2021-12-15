use std::ops::Index;

use crate::error::SimdevalError;

use super::value::single::Value;

#[derive(Debug)]
pub(crate) struct Variables<'a> {
    identifiers: Vec<&'a str>,
    values: Vec<Value>,
}
impl<'a> Variables<'a> {
    pub(crate) fn clear(&mut self) {
        self.identifiers.clear();
        self.values.clear();
    }
    pub(crate) fn with_capacity(capacity: usize) -> Self {
        Self {
            identifiers: Vec::with_capacity(capacity),
            values: Vec::with_capacity(capacity),
        }
    }
    pub(crate) fn push(&mut self, identifier: &'a str) {
        self.identifiers.push(identifier);
        self.values.push(Value::Int(0));
    }
    pub(crate) fn set(&mut self, identifier: &'a str, value: Value) -> Result<(), SimdevalError> {
        let index = self
            .identifiers
            .binary_search(&identifier)
            .map_err(|_| SimdevalError::InvalidVariable)?;
        self.values[index] = value;
        Ok(())
    }
    pub(crate) fn set_by_index(&mut self, index: usize, value: Value) -> Result<(), SimdevalError> {
        *self
            .values
            .get_mut(index)
            .ok_or(SimdevalError::InvalidVariable)? = value;
        Ok(())
    }
    pub(crate) fn find_or_set(&mut self, identifier: &'a str) -> usize {
        if let Ok(i) = self.identifiers.binary_search(&identifier) {
            i
        } else {
            self.identifiers.push(identifier);
            self.values.push(Value::Int(0));
            self.identifiers.len() - 1
        }
    }
    pub(crate) fn identifiers(&self) -> &[&str] {
        &self.identifiers
    }
}

impl<'a> Index<usize> for Variables<'a> {
    type Output = Value;

    fn index(&self, index: usize) -> &Self::Output {
        &self.values[index]
    }
}
