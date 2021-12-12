
use crate::{error::SimdevalError, evaluate::{enums::Value, node::Node}};

use super::Function;
#[derive(Debug, Clone)]
pub(crate) enum Std {
    Sqrt,
    Log,
}
impl Function<Std> for Std {
    const NAMESPACE: &'static str = "std";
    const MAX_ARGS: u8 = 4;
    fn from_string(
        namespaces: &mut std::slice::Iter<&str>,
        identifier: &str,
    ) -> Result<Self, SimdevalError> {
        if let Some(next) = namespaces.next() {
            let slice = &next[0..next.len() - 1];
            if slice == Self::NAMESPACE {
                Ok(match identifier {
                    "log" => Self::Log,
                    "sqrt" => Self::Sqrt,
                    _ => return Err(SimdevalError::NoIdentifierMatch),
                })
            } else {
                Err(SimdevalError::InvalidNamespace)
            }
        } else {
            Ok(match identifier {
                "log" => Self::Log,
                "sqrt" => Self::Sqrt,
                _ => return Err(SimdevalError::NoIdentifierMatch),
            })
        }
    }
    fn call<S: Function<S>>(&self, node: &[Node<S>]) -> Value {
        match self {
            Self::Log => Value::Int(1),
            Self::Sqrt => Value::Int(2),
        }
    }
}
