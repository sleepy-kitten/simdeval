use ::std::slice::Iter;

use crate::error::SimdevalError;

use super::enums::Value;

pub mod std;
pub mod macros;


pub trait Function<T>
where
    T: Function<T>,
{
    const NAMESPACE: &'static str;
    const MAX_ARGS: u8;
    fn from_string(namespaces: &mut Iter<&str>, identifier: &str) -> Result<T, SimdevalError>;
    fn call(&self, node: &[Value]) -> Result<Value, SimdevalError>;
    fn is_const(&self) -> bool {
        true
    }
}
