use std::{
    ops::{Add, Div, Mul, Rem, Sub},
    simd::{self, i64x8, f64x8, LaneCount, SupportedLaneCount},
};
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Single {
    Int(i64),
    Float(f64),
    Bool(bool),
}

impl Single {
    pub fn to_float(&mut self) {
        *self = Single::Float(match self {
            Single::Int(v) => *v as f64,
            Single::Float(v) => *v,
            Single::Bool(v) => *v as u8 as f64
        })
    }
    pub fn to_int(&mut self) {
        *self = Single::Int(match self {
            Single::Int(v) => *v,
            Single::Float(v) => *v as i64,
            Single::Bool(v) => *v as i64
        })
    }
    pub fn to_bool(&mut self) {
        *self = Single::Bool(match self {
            Single::Int(v) => *v != 0,
            Single::Float(v) => *v != 0.0,
            Single::Bool(v) => *v
        })
    }

    pub fn as_float(&self) -> f64 {
        match self {
            Single::Int(v) => *v as f64,
            Single::Float(v) => *v,
            Single::Bool(v) => *v as u8 as f64
        }
    }
    pub fn as_int(&self) -> i64 {
        match self {
            Single::Int(v) => *v,
            Single::Float(v) => *v as i64,
            Single::Bool(v) => *v as i64
        }
    }
    pub fn as_bool(&self) -> bool {
        match self {
            Single::Int(v) => *v != 0,
            Single::Float(v) => *v != 0.0,
            Single::Bool(v) => *v
        }
    }
}
impl Add for Single {
    type Output = Single;
    fn add(self, rhs: Self) -> Self::Output {
        match self {
            Single::Int(l) => match rhs {
                Single::Int(r) => Single::Int(l + r),
                Single::Float(r) => Single::Float(l as f64 + r),
                Single::Bool(r) => Single::Int(l + r as i64),
            },
            Single::Float(l) => match rhs {
                Single::Int(r) => Single::Float(l + r as f64),
                Single::Float(r) => Single::Float(l + r),
                Single::Bool(r) => Single::Float(l + r as i64 as f64),
            },
            Single::Bool(l) => match rhs {
                Single::Int(r) => Single::Int(l as i64 + r),
                Single::Float(r) => Single::Float(l as i64 as f64 + r),
                Single::Bool(r) => Single::Int(l as i64 + r as i64),
            },
        }
    }
}
impl Sub for Single {
    type Output = Single;

    fn sub(self, rhs: Self) -> Self::Output {
        match self {
            Single::Int(l) => match rhs {
                Single::Int(r) => Single::Int(l - r),
                Single::Float(r) => Single::Float(l as f64 - r),
                Single::Bool(r) => Single::Int(l - r as i64),
            },
            Single::Float(l) => match rhs {
                Single::Int(r) => Single::Float(l - r as f64),
                Single::Float(r) => Single::Float(l - r),
                Single::Bool(r) => Single::Float(l - r as i64 as f64),
            },
            Single::Bool(l) => match rhs {
                Single::Int(r) => Single::Int(l as i64 - r),
                Single::Float(r) => Single::Float(l as i64 as f64 - r),
                Single::Bool(r) => Single::Int(l as i64 - r as i64),
            },
        }
    }
}
impl Mul for Single {
    type Output = Single;
    fn mul(self, rhs: Self) -> Self::Output {
        match self {
            Single::Int(l) => match rhs {
                Single::Int(r) => Single::Int(l * r),
                Single::Float(r) => Single::Float(l as f64 * r),
                Single::Bool(r) => Single::Int(l * r as i64),
            },
            Single::Float(l) => match rhs {
                Single::Int(r) => Single::Float(l * r as f64),
                Single::Float(r) => Single::Float(l * r),
                Single::Bool(r) => Single::Float(l * r as i64 as f64),
            },
            Single::Bool(l) => match rhs {
                Single::Int(r) => Single::Int(l as i64 * r),
                Single::Float(r) => Single::Float(l as i64 as f64 * r),
                Single::Bool(r) => Single::Int(l as i64 * r as i64),
            },
        }
    }
}

impl Div for Single {
    type Output = Single;
    fn div(self, rhs: Self) -> Self::Output {
        match self {
            Single::Int(l) => match rhs {
                Single::Int(r) => Single::Int(l / r),
                Single::Float(r) => Single::Float(l as f64 / r),
                Single::Bool(r) => Single::Int(l / r as i64),
            },
            Single::Float(l) => match rhs {
                Single::Int(r) => Single::Float(l / r as f64),
                Single::Float(r) => Single::Float(l / r),
                Single::Bool(r) => Single::Float(l / r as i64 as f64),
            },
            Single::Bool(l) => match rhs {
                Single::Int(r) => Single::Int(l as i64 / r),
                Single::Float(r) => Single::Float(l as i64 as f64 / r),
                Single::Bool(r) => Single::Int(l as i64 / r as i64),
            },
        }
    }
}
impl Rem for Single {
    type Output = Single;
    fn rem(self, rhs: Self) -> Self::Output {
        match self {
            Single::Int(l) => match rhs {
                Single::Int(r) => Single::Int(l % r),
                Single::Float(r) => Single::Float(l as f64 % r),
                Single::Bool(r) => Single::Int(l % r as i64),
            },
            Single::Float(l) => match rhs {
                Single::Int(r) => Single::Float(l % r as f64),
                Single::Float(r) => Single::Float(l % r),
                Single::Bool(r) => Single::Float(l % r as i64 as f64),
            },
            Single::Bool(l) => match rhs {
                Single::Int(r) => Single::Int(l as i64 % r),
                Single::Float(r) => Single::Float(l as i64 as f64 % r),
                Single::Bool(r) => Single::Int(l as i64 % r as i64),
            },
        }
    }
}

impl Single {
    pub(crate) fn pow(self, rhs: Self) -> Single {
        match self {
            Single::Int(l) => match rhs {
                Single::Int(r) => {
                    if r.is_positive() {
                        Single::Int(l.pow(r.try_into().expect("exponent out of range")))
                    } else {
                        Single::Float((l as f64).powi(r.try_into().expect("exponent out of range")))
                    }
                }
                Single::Float(r) => Single::Float((l as f64).powf(r)),
                Single::Bool(r) => Single::Int(l.pow(r as u32)),
            },
            Single::Float(l) => match rhs {
                Single::Int(r) => Single::Float(l.powi(r.try_into().expect("exponent ouf of range"))),
                Single::Float(r) => Single::Float(l.powf(r)),
                Single::Bool(r) => Single::Float(l.powi(r as i32)),
            },
            Single::Bool(l) => match rhs {
                Single::Int(r) => Single::Bool(l),
                Single::Float(r) => Single::Bool(l),
                Single::Bool(r) => Single::Bool(l),
            },
        }
    }
    pub(crate) fn and(self, rhs: Self) -> Single {
        let lhs = match self {
            Single::Int(l) => l != 0,
            Single::Float(l) => l != 0.0,
            Single::Bool(l) => l,
        };
        let rhs = match rhs {
            Single::Int(r) => r != 0,
            Single::Float(r) => r != 0.0,
            Single::Bool(r) => r,
        };
        Single::Bool(lhs && rhs)
    }
    pub(crate) fn or(self, rhs: Self) -> Single {
        let lhs = match self {
            Single::Int(l) => l != 0,
            Single::Float(l) => l != 0.0,
            Single::Bool(l) => l,
        };
        let rhs = match rhs {
            Single::Int(r) => r != 0,
            Single::Float(r) => r != 0.0,
            Single::Bool(r) => r,
        };
        Single::Bool(lhs || rhs)
    }
    pub(crate) fn xor(self, rhs: Self) -> Single {
        let lhs = match self {
            Single::Int(l) => l != 0,
            Single::Float(l) => l != 0.0,
            Single::Bool(l) => l,
        };
        let rhs = match rhs {
            Single::Int(r) => r != 0,
            Single::Float(r) => r != 0.0,
            Single::Bool(r) => r,
        };
        Single::Bool(lhs ^ rhs)
    }
}
