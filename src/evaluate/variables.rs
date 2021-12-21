use std::{
    collections::{hash_map::Iter, HashMap},
    ops::{Index, IndexMut},
    simd::{LaneCount, SupportedLaneCount},
};

use crate::{error::Error, small_string::SmallString};

use super::value::{single::Single, Value};

#[derive(Debug)]
pub(crate) struct Variables<const LANES: usize>
where
    LaneCount<LANES>: SupportedLaneCount,
{
    identifiers: HashMap<SmallString<16>, usize>,
    values: Vec<Value<LANES>>,
}
impl<'a, const LANES: usize> Variables<LANES>
where
    LaneCount<LANES>: SupportedLaneCount,
{
    pub(crate) fn clear(&mut self) {
        self.identifiers.clear();
        self.values.clear();
    }
    pub(crate) fn with_capacity(capacity: usize) -> Self {
        Self {
            identifiers: HashMap::with_capacity(capacity),
            values: Vec::with_capacity(capacity),
        }
    }
    pub(crate) fn push(&mut self, identifier: &'a str) {
        self.identifiers
            .insert(identifier.into(), self.values.len());
        self.values.push(Value::Single(Single::Int(0)));
    }
    pub(crate) fn set(
        &mut self,
        identifier: &'a str,
        value: Value<LANES>,
    ) -> Result<(), Error> {
        let index = *self
            .identifiers
            .get(identifier)
            .ok_or(Error::InvalidVariable)?;
        self.values[index] = value;
        Ok(())
    }
    pub(crate) fn set_by_index(
        &mut self,
        index: usize,
        value: Value<LANES>,
    ) -> Result<(), Error> {
        *self
            .values
            .get_mut(index)
            .ok_or(Error::InvalidVariable)? = value;
        Ok(())
    }
    pub(crate) fn find_or_set(&mut self, identifier: &'a str) -> usize {
        if let Some(index) = self.identifiers.get(identifier) {
            *index
        } else {
            self.push(identifier);
            self.values.len() - 1
        }
    }
    pub(crate) fn identifiers_iter(&self) -> Iter<SmallString<16>, usize> {
        self.identifiers.iter()
    }
}

impl<const LANES: usize> Index<usize> for Variables<LANES>
where
    LaneCount<LANES>: SupportedLaneCount,
{
    type Output = Value<LANES>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.values[index]
    }
}

impl<const LANES: usize> IndexMut<usize> for Variables<LANES>
where
    LaneCount<LANES>: SupportedLaneCount,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.values[index]
    }
}

impl<'a, const LANES: usize> Default for Variables<LANES>
where
    LaneCount<LANES>: SupportedLaneCount,
{
    fn default() -> Self {
        Self {
            identifiers: Default::default(),
            values: Default::default(),
        }
    }
}
