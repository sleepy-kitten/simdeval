use ::std::slice::Iter;

use crate::error::SimdevalError;

use super::value::single::Value;


pub mod std;
pub mod macros;


pub trait Function<T>
where
    T: Function<T>,
{
    const NAMESPACE: &'static str;
    const MAX_ARGS: usize;
    fn from_string(namespaces: &mut Iter<&str>, identifier: &str) -> Result<T, SimdevalError>;
    fn call(&self, args: &[Value]) -> Result<Value, SimdevalError>;
    fn is_const(&self) -> bool {
        true
    }
}
