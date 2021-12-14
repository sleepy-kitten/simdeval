use crate::error::SimdevalError;

use super::enums::Value;

struct Variables<'a>{
    identifiers: Vec<&'a str>,
    values: Vec<Value>
}
impl<'a> Variables<'a> {
    pub fn with_capacity(capacity: usize) -> Self{
        Self {
            identifiers: Vec::with_capacity(capacity),
            values: Vec::with_capacity(capacity)
        }
    }
    pub fn push(&mut self, identifier: &'a str) {
        self.identifiers.push(identifier);
        self.values.push(Value::Int(0));
    }
    pub fn set(&mut self, identifier: &'a str, value: Value) -> Result<(), SimdevalError>{
        let index = self.identifiers.binary_search(&identifier).map_err(|_| SimdevalError::InvalidVariable)?;
        self.values[index] = value;
        Ok(())
    }
}