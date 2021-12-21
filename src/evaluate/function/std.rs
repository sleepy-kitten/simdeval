use std::simd::{LaneCount, SupportedLaneCount};

use crate::{
    error::Error,
    evaluate::{
        node::Node,
        value::{single::Single, Value},
    },
    impl_functions,
};

use super::Function;

fn sqrt<const LANES: usize>(values: [Value<LANES>; 1]) -> Value<LANES>
where
    LaneCount<LANES>: SupportedLaneCount,
{   
    Value::Single(Single::Float(match values {
        [Value::Single(Single::Int(v))] => (v as f64).sqrt(),
        [Value::Single(Single::Float(v))] => v.sqrt(),
        [Value::Single(Single::Bool(v))] => v as i64 as f64,
        _ => todo!()
    }))
}
fn print<const LANES: usize>(values: [Value<LANES>; 1]) -> Value<LANES>
where
    LaneCount<LANES>: SupportedLaneCount,
{
    println!("{:#?}", values[0]);
    Value::Single(Single::Int(0))
}
fn log<const LANES: usize>(values: [Value<LANES>; 2]) -> Value<LANES>
where
    LaneCount<LANES>: SupportedLaneCount,
{
    if let [Value::Single(lhs), Value::Single(rhs)] = values {
        let num = lhs.as_float();
        let base = rhs.as_float();
        Value::Single(Single::Float(num.log(base)))
    } else {
        todo!()
    }
}
fn abs<const LANES: usize>(values: [Value<LANES>; 1]) -> Value<LANES>
where
    LaneCount<LANES>: SupportedLaneCount,
{
    let num = values[0];
    match num {
        Value::Single(Single::Float(v)) => Value::Single(Single::Float(v.abs())),
        Value::Single(Single::Int(v)) => Value::Single(Single::Int(v.abs())),
        Value::Single(Single::Bool(v)) => num,
        _ => panic!(),
    }
}
/*
impl<const LANES: usize> StdTest<LANES> {

}

pub enum StdTest<const LANES: usize> {
    Test,
}
impl<const LANES: usize> Function<StdTest<LANES>, LANES> for StdTest<LANES>
where
    StdTest<LANES>: Function<StdTest<LANES>, LANES>,
    LaneCount<LANES>: SupportedLaneCount,

{
    const NAMESPACE: &'static str = "std";

    const MAX_ARGS: usize = 2;

    fn from_string(namespaces: &mut std::slice::Iter<&str>, identifier: &str) -> Result<StdTest<LANES>, SimdevalError> {
        todo!()
    }

    fn call(&self, args: &[Value<LANES>]) -> Result<Value<LANES>, SimdevalError> {
        match self {
            StdTest::Test => todo!()
        }
    }
}
*/

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
