use std::{
    ops::{Add, Div, Mul, Rem, Sub},
    simd::{LaneCount, SupportedLaneCount},
};

use self::{simd::Simd, single::Single};

pub mod simd;
pub mod single;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Value<const LANES: usize>
where
    LaneCount<LANES>: SupportedLaneCount,
{
    Simd(Simd<LANES>),
    Single(Single),
}

impl<const LANES: usize> std::fmt::Display for Value<LANES> where
    LaneCount<LANES>: SupportedLaneCount
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Simd(v) => v.fmt(f),
            Self::Single(v) => v.fmt(f)
        }
    }
}
impl<const LANES: usize> Value<LANES>
where
    LaneCount<LANES>: SupportedLaneCount,
{
    pub fn to_simd(&mut self) {
        if let Value::Single(single) = self {
            let value = single.as_float();
            let array = [value; LANES];
            *self = Value::Simd(Simd::Float(array.into()));
        }
    }
}

impl<const LANES: usize> Add for Value<LANES>
where
    LaneCount<LANES>: SupportedLaneCount,
{
    type Output = Value<LANES>;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Single(lhs), Value::Simd(rhs)) => todo!(),
            (Value::Simd(lhs), Value::Single(rhs)) => todo!(),
            (Value::Simd(lhs), Value::Simd(rhs)) => Value::Simd(lhs + rhs),
            (Value::Single(lhs), Value::Single(rhs)) => Value::Single(lhs + rhs),
        }
    }
}
impl<const LANES: usize> Sub for Value<LANES>
where
    LaneCount<LANES>: SupportedLaneCount,
{
    type Output = Value<LANES>;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Single(lhs), Value::Simd(rhs)) => todo!(),
            (Value::Simd(lhs), Value::Single(rhs)) => todo!(),
            (Value::Simd(lhs), Value::Simd(rhs)) => Value::Simd(lhs - rhs),
            (Value::Single(lhs), Value::Single(rhs)) => Value::Single(lhs - rhs),
        }
    }
}
impl<const LANES: usize> Mul for Value<LANES>
where
    LaneCount<LANES>: SupportedLaneCount,
{
    type Output = Value<LANES>;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Single(lhs), Value::Simd(rhs)) => todo!(),
            (Value::Simd(lhs), Value::Single(rhs)) => todo!(),
            (Value::Simd(lhs), Value::Simd(rhs)) => Value::Simd(lhs * rhs),
            (Value::Single(lhs), Value::Single(rhs)) => Value::Single(lhs * rhs),
        }
    }
}
impl<const LANES: usize> Div for Value<LANES>
where
    LaneCount<LANES>: SupportedLaneCount,
{
    type Output = Value<LANES>;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Single(lhs), Value::Simd(rhs)) => todo!(),
            (Value::Simd(lhs), Value::Single(rhs)) => todo!(),
            (Value::Simd(lhs), Value::Simd(rhs)) => Value::Simd(lhs / rhs),
            (Value::Single(lhs), Value::Single(rhs)) => Value::Single(lhs / rhs),
        }
    }
}
impl<const LANES: usize> Rem for Value<LANES>
where
    LaneCount<LANES>: SupportedLaneCount,
{
    type Output = Value<LANES>;

    fn rem(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Single(lhs), Value::Simd(rhs)) => todo!(),
            (Value::Simd(lhs), Value::Single(rhs)) => todo!(),
            (Value::Simd(lhs), Value::Simd(rhs)) => Value::Simd(lhs % rhs),
            (Value::Single(lhs), Value::Single(rhs)) => Value::Single(lhs % rhs),
        }
    }
}

impl<const LANES: usize> Value<LANES>
where
    LaneCount<LANES>: SupportedLaneCount,
{
    pub(crate) fn pow(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Value::Single(lhs), Value::Simd(rhs)) => todo!(),
            (Value::Simd(lhs), Value::Single(rhs)) => todo!(),
            (Value::Simd(lhs), Value::Simd(rhs)) => Value::Simd(lhs.pow(rhs)),
            (Value::Single(lhs), Value::Single(rhs)) => Value::Single(lhs.pow(rhs)),
        }
    }
}
