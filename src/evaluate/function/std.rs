
use crate::{error::SimdevalError, evaluate::{ node::Node, value::single::Value}, impl_functions};

use super::Function;

fn sqrt(values: [Value; 1]) -> Value {
    Value::Float(match values {
        [Value::Int(v)] => (v as f64).sqrt(),
        [Value::Float(v)] => v.sqrt(),
        [Value::Bool(v)] => v as i64 as f64
    })
}

impl_functions!(Std: std; []; [Sqrt: sqrt, 1]);


/*
#[derive(Debug, Clone)]
pub(crate) enum Std {
    Sqrt,
    Log,
}
impl Function<Std> for Std {
    const NAMESPACE: &'static str = "std";
    const MAX_ARGS: usize = 4;
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
    fn call(&self, node: &[Value]) -> Result<Value, SimdevalError> {
        Ok(match self {
            Self::Log => Value::Int(1),
            Self::Sqrt => Value::Int(2),
        })
    }
}
*/