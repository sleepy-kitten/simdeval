use std::ops::{Add, Div, Mul, Rem, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum TokenKind {
    Literal(Literal),
    Bracket(Bracket),
    Identifier(Identifier),
    Operator(Operator),
    Special(Special),
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Literal {
    Int,
    Float,
    Bool,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Bracket {
    Opened,
    Closed,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Identifier {
    Function,
    Variable,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,

    Not,
    Equal,
    And,
    Or,
    Xor,
    NotEqual,
    Greater,
    GreaterEqual,
    Smaller,
    SmallerEqual,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Special {
    Namespace,
    NegZero,
}
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
pub enum Simd {

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
                },
                Value::Float(r) => Value::Float((l as f64).powf(r)),
                Value::Bool(r) => {
                    Value::Int(l.pow(r as u32))
                }
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
            Value::Bool(l) => l
        };
        let rhs = match rhs {
            Value::Int(r) => r != 0,
            Value::Float(r) => r != 0.0,
            Value::Bool(r) => r
        };
        Value::Bool(lhs && rhs)
    }
    pub(crate) fn or(self, rhs: Self) -> Value {
        let lhs = match self {
            Value::Int(l) => l != 0,
            Value::Float(l) => l != 0.0,
            Value::Bool(l) => l
        };
        let rhs = match rhs {
            Value::Int(r) => r != 0,
            Value::Float(r) => r != 0.0,
            Value::Bool(r) => r
        };
        Value::Bool(lhs || rhs)
    }
    pub(crate) fn xor(self, rhs: Self) -> Value {
        let lhs = match self {
            Value::Int(l) => l != 0,
            Value::Float(l) => l != 0.0,
            Value::Bool(l) => l
        };
        let rhs = match rhs {
            Value::Int(r) => r != 0,
            Value::Float(r) => r != 0.0,
            Value::Bool(r) => r
        };
        Value::Bool(lhs ^ rhs)
    }
}

impl Bracket {
    pub fn weight(&self) -> i16 {
        match self {
            Self::Opened => 100,
            Self::Closed => -100,
        }
    }
}
impl Operator {
    pub(crate) fn weight(&self) -> i16 {
        match self {
            Self::Add => 20,
            Self::Sub => 20,
            Self::Mul => 21,
            Self::Div => 21,
            Self::Mod => 22,
            Self::Pow => 22,

            Self::Not => 1,
            Self::Equal => 2,
            Self::And => 3,
            Self::Or => 4,
            Self::Xor => 5,
            Self::NotEqual => 7,
            Self::Greater => 8,
            Self::GreaterEqual => 9,
            Self::Smaller => 10,
            Self::SmallerEqual => 11,
        }
    }
    pub(crate) fn eval(&self, lhs: Value, rhs: Value) -> Value {
        match self {
            Self::Add => lhs + rhs,
            Self::Sub => lhs - rhs,
            Self::Mul => lhs * rhs,
            Self::Div => lhs / rhs,
            Self::Mod => lhs % rhs,
            Self::Pow => lhs.pow(rhs),
            Self::Equal => Value::Bool(lhs == rhs),
            Self::NotEqual => Value::Bool(lhs != rhs),
            Self::Greater => Value::Bool(lhs > rhs),
            Self::GreaterEqual => Value::Bool(lhs >= rhs),
            Self::Smaller => Value::Bool(lhs < rhs),
            Self::SmallerEqual => Value::Bool(lhs <= rhs),
            Self::And => lhs.and(rhs),
            Self::Or => lhs.or(rhs),
            Self::Xor => lhs.xor(rhs),
            _ => unreachable!(),
        }
    }
}
