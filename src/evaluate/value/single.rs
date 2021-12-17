use std::{
    ops::{Add, Div, Mul, Rem, Sub},
    simd::{self, i64x8, f64x8, LaneCount, SupportedLaneCount},
};
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Value {
    Int(i64),
    Float(f64),
    Bool(bool),
}
pub enum Single {
    Int(i64),
    Float(f64),
    Bool(bool),
}
impl Value {
    pub fn to_float(&mut self) {
        *self = Value::Float(match self {
            Value::Int(v) => *v as f64,
            Value::Float(v) => *v,
            Value::Bool(v) => *v as u8 as f64
        })
    }
    pub fn to_int(&mut self) {
        *self = Value::Int(match self {
            Value::Int(v) => *v,
            Value::Float(v) => *v as i64,
            Value::Bool(v) => *v as i64
        })
    }
    pub fn to_bool(&mut self) {
        *self = Value::Bool(match self {
            Value::Int(v) => *v != 0,
            Value::Float(v) => *v != 0.0,
            Value::Bool(v) => *v
        })
    }

    pub fn as_float(&self) -> f64 {
        match self {
            Value::Int(v) => *v as f64,
            Value::Float(v) => *v,
            Value::Bool(v) => *v as u8 as f64
        }
    }
    pub fn as_int(&self) -> i64 {
        match self {
            Value::Int(v) => *v,
            Value::Float(v) => *v as i64,
            Value::Bool(v) => *v as i64
        }
    }
    pub fn as_bool(&self) -> bool {
        match self {
            Value::Int(v) => *v != 0,
            Value::Float(v) => *v != 0.0,
            Value::Bool(v) => *v
        }
    }
}
impl Add for Value {
    type Output = Value;
    fn add(self, rhs: Self) -> Self::Output {
        match self {
            Value::Int(l) => match rhs {
                Value::Int(r) => Value::Int(l + r),
                Value::Float(r) => Value::Float(l as f64 + r),
                Value::Bool(r) => Value::Int(l + r as i64),
            },
            Value::Float(l) => match rhs {
                Value::Int(r) => Value::Float(l + r as f64),
                Value::Float(r) => Value::Float(l + r),
                Value::Bool(r) => Value::Float(l + r as i64 as f64),
            },
            Value::Bool(l) => match rhs {
                Value::Int(r) => Value::Int(l as i64 + r),
                Value::Float(r) => Value::Float(l as i64 as f64 + r),
                Value::Bool(r) => Value::Int(l as i64 + r as i64),
            },
        }
    }
}
impl Sub for Value {
    type Output = Value;

    fn sub(self, rhs: Self) -> Self::Output {
        match self {
            Value::Int(l) => match rhs {
                Value::Int(r) => Value::Int(l - r),
                Value::Float(r) => Value::Float(l as f64 - r),
                Value::Bool(r) => Value::Int(l - r as i64),
            },
            Value::Float(l) => match rhs {
                Value::Int(r) => Value::Float(l - r as f64),
                Value::Float(r) => Value::Float(l - r),
                Value::Bool(r) => Value::Float(l - r as i64 as f64),
            },
            Value::Bool(l) => match rhs {
                Value::Int(r) => Value::Int(l as i64 - r),
                Value::Float(r) => Value::Float(l as i64 as f64 - r),
                Value::Bool(r) => Value::Int(l as i64 - r as i64),
            },
        }
    }
}
impl Mul for Value {
    type Output = Value;
    fn mul(self, rhs: Self) -> Self::Output {
        match self {
            Value::Int(l) => match rhs {
                Value::Int(r) => Value::Int(l * r),
                Value::Float(r) => Value::Float(l as f64 * r),
                Value::Bool(r) => Value::Int(l * r as i64),
            },
            Value::Float(l) => match rhs {
                Value::Int(r) => Value::Float(l * r as f64),
                Value::Float(r) => Value::Float(l * r),
                Value::Bool(r) => Value::Float(l * r as i64 as f64),
            },
            Value::Bool(l) => match rhs {
                Value::Int(r) => Value::Int(l as i64 * r),
                Value::Float(r) => Value::Float(l as i64 as f64 * r),
                Value::Bool(r) => Value::Int(l as i64 * r as i64),
            },
        }
    }
}

impl Div for Value {
    type Output = Value;
    fn div(self, rhs: Self) -> Self::Output {
        match self {
            Value::Int(l) => match rhs {
                Value::Int(r) => Value::Int(l / r),
                Value::Float(r) => Value::Float(l as f64 / r),
                Value::Bool(r) => Value::Int(l / r as i64),
            },
            Value::Float(l) => match rhs {
                Value::Int(r) => Value::Float(l / r as f64),
                Value::Float(r) => Value::Float(l / r),
                Value::Bool(r) => Value::Float(l / r as i64 as f64),
            },
            Value::Bool(l) => match rhs {
                Value::Int(r) => Value::Int(l as i64 / r),
                Value::Float(r) => Value::Float(l as i64 as f64 / r),
                Value::Bool(r) => Value::Int(l as i64 / r as i64),
            },
        }
    }
}
impl Rem for Value {
    type Output = Value;
    fn rem(self, rhs: Self) -> Self::Output {
        match self {
            Value::Int(l) => match rhs {
                Value::Int(r) => Value::Int(l % r),
                Value::Float(r) => Value::Float(l as f64 % r),
                Value::Bool(r) => Value::Int(l % r as i64),
            },
            Value::Float(l) => match rhs {
                Value::Int(r) => Value::Float(l % r as f64),
                Value::Float(r) => Value::Float(l % r),
                Value::Bool(r) => Value::Float(l % r as i64 as f64),
            },
            Value::Bool(l) => match rhs {
                Value::Int(r) => Value::Int(l as i64 % r),
                Value::Float(r) => Value::Float(l as i64 as f64 % r),
                Value::Bool(r) => Value::Int(l as i64 % r as i64),
            },
        }
    }
}

impl Value {
    pub(crate) fn pow(self, rhs: Self) -> Value {
        match self {
            Value::Int(l) => match rhs {
                Value::Int(r) => {
                    if r.is_positive() {
                        Value::Int(l.pow(r.try_into().expect("exponent out of range")))
                    } else {
                        Value::Float((l as f64).powi(r.try_into().expect("exponent out of range")))
                    }
                }
                Value::Float(r) => Value::Float((l as f64).powf(r)),
                Value::Bool(r) => Value::Int(l.pow(r as u32)),
            },
            Value::Float(l) => match rhs {
                Value::Int(r) => Value::Float(l.powi(r.try_into().expect("exponent ouf of range"))),
                Value::Float(r) => Value::Float(l.powf(r)),
                Value::Bool(r) => Value::Float(l.powi(r as i32)),
            },
            Value::Bool(l) => match rhs {
                Value::Int(r) => Value::Bool(l),
                Value::Float(r) => Value::Bool(l),
                Value::Bool(r) => Value::Bool(l),
            },
        }
    }
    pub(crate) fn and(self, rhs: Self) -> Value {
        let lhs = match self {
            Value::Int(l) => l != 0,
            Value::Float(l) => l != 0.0,
            Value::Bool(l) => l,
        };
        let rhs = match rhs {
            Value::Int(r) => r != 0,
            Value::Float(r) => r != 0.0,
            Value::Bool(r) => r,
        };
        Value::Bool(lhs && rhs)
    }
    pub(crate) fn or(self, rhs: Self) -> Value {
        let lhs = match self {
            Value::Int(l) => l != 0,
            Value::Float(l) => l != 0.0,
            Value::Bool(l) => l,
        };
        let rhs = match rhs {
            Value::Int(r) => r != 0,
            Value::Float(r) => r != 0.0,
            Value::Bool(r) => r,
        };
        Value::Bool(lhs || rhs)
    }
    pub(crate) fn xor(self, rhs: Self) -> Value {
        let lhs = match self {
            Value::Int(l) => l != 0,
            Value::Float(l) => l != 0.0,
            Value::Bool(l) => l,
        };
        let rhs = match rhs {
            Value::Int(r) => r != 0,
            Value::Float(r) => r != 0.0,
            Value::Bool(r) => r,
        };
        Value::Bool(lhs ^ rhs)
    }
}
