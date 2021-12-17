use ::std::{slice::Iter, simd::{LaneCount, SupportedLaneCount}};

use crate::error::SimdevalError;

use super::value::Value;


pub mod std;
pub mod macros;


pub trait Function<T, const LANES: usize>
where
    T: Function<T, LANES>,
    LaneCount<LANES>: SupportedLaneCount
{
    const NAMESPACE: &'static str;
    const MAX_ARGS: usize;
    fn from_string(namespaces: &mut Iter<&str>, identifier: &str) -> Result<T, SimdevalError>;
    fn call(&self, args: &[Value<LANES>]) -> Result<Value<LANES>, SimdevalError>;
    fn is_const(&self) -> bool {
        true
    }
}
