
use crate::{error::SimdevalError, evaluate::{ node::Node, value::single::Value}, impl_functions};

use super::Function;

fn sqrt(values: [Value; 1]) -> Value {
    Value::Float(match values {
        [Value::Int(v)] => (v as f64).sqrt(),
        [Value::Float(v)] => v.sqrt(),
        [Value::Bool(v)] => v as i64 as f64
    })
}
fn print(values: [Value; 1]) -> Value {
    println!("{:#?}", values[0]);
    Value::Int(0)
}
fn log(values: [Value; 2]) -> Value {
    let num = values[0].as_float();
    let base = values[1].as_float();
    Value::Float(num.log(base))

}
fn abs(values: [Value; 1]) -> Value {
    let num = values[0];
    match num {
        Value::Float(v) => Value::Float(v.abs()),
        Value::Int(v) => Value::Int(v.abs()),
        Value::Bool(v) => num,
    }
}

impl_functions!(
    Std: std; 
    []; 
    [
        Sqrt: sqrt(1),
        Print: print(1); false,
        Log: log(2),
        Abs: abs(1)
    ]);


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