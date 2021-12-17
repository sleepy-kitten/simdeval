use std::{
    ops::{Index, IndexMut},
    simd::{LaneCount, SupportedLaneCount},
};

use crate::error::SimdevalError;

use super::value::{single::Single, Value};

#[derive(Debug)]
pub(crate) struct Variables<'a, const LANES: usize>
where
    LaneCount<LANES>: SupportedLaneCount,
{
    identifiers: Vec<&'a str>,
    values: Vec<Value<LANES>>,
}
impl<'a, const LANES: usize> Variables<'a, LANES>
where
    LaneCount<LANES>: SupportedLaneCount,
{
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
        self.values.push(Value::Single(Single::Int(0)));
    }
    pub(crate) fn set(
        &mut self,
        identifier: &'a str,
        value: Value<LANES>,
    ) -> Result<(), SimdevalError> {
        let index = self
            .identifiers
            .binary_search(&identifier)
            .map_err(|_| SimdevalError::InvalidVariable)?;
        self.values[index] = value;
        Ok(())
    }
    pub(crate) fn set_by_index(
        &mut self,
        index: usize,
        value: Value<LANES>,
    ) -> Result<(), SimdevalError> {
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
            self.values.push(Value::Single(Single::Int(0)));
            self.identifiers.len() - 1
        }
    }
    pub(crate) fn identifiers(&self) -> &[&str] {
        &self.identifiers
    }
}

impl<'a, const LANES: usize> Index<usize> for Variables<'a, LANES>
where
    LaneCount<LANES>: SupportedLaneCount,
{
    type Output = Value<LANES>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.values[index]
    }
}

impl<'a, const LANES: usize> IndexMut<usize> for Variables<'a, LANES> where
    LaneCount<LANES>: SupportedLaneCount
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.values[index]
    }
}
