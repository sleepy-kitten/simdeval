use std::slice::Iter;

use crate::error::SimdevalError;

use super::{node::Node, enums::Value};

pub(crate) trait Function<T>
where
    T: Function<T>,
{
    const NAMESPACE: &'static str;
    const MAX_ARGS: u8;
    fn from_string(namespaces: &mut Iter<&str>, identifier: &str) -> Result<T, SimdevalError>;
    fn call<S: Function<S>>(&self, node: &[Node<S>]) -> Value;
    fn is_const() -> bool {
        true
    }
}
