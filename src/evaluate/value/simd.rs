use std::{
    ops::{Add, Div, Mul, Rem, Sub},
    simd::{self, f64x8, i64x8, LaneCount, SupportedLaneCount},
};

#[derive(Debug, Clone, Copy)]
pub enum Simd<const LANES: usize>
where
    LaneCount<LANES>: SupportedLaneCount,
{
    //Int(simd::Simd<i64, LANES>),
    Float(simd::Simd<f64, LANES>),
    //Bool(simd::Simd<u64, LANES>),
}
impl<const LANES: usize> Simd<LANES>
where
    LaneCount<LANES>: SupportedLaneCount,
{
    fn as_float(&self) -> simd::Simd<f64, LANES> {
        match self {
            Simd::Float(v) => *v,
        }
    }
}

impl<const LANES: usize> Add for Simd<LANES>
where
    LaneCount<LANES>: SupportedLaneCount,
{
    type Output = Simd<LANES>;
    fn add(self, rhs: Self) -> Self::Output {
        Simd::Float(self.as_float() + rhs.as_float())
    }
}
impl<const LANES: usize> Sub for Simd<LANES>
where
    LaneCount<LANES>: SupportedLaneCount,
{
    type Output = Simd<LANES>;

    fn sub(self, rhs: Self) -> Self::Output {
        Simd::Float(self.as_float() - rhs.as_float())
    }
}
impl<const LANES: usize> Mul for Simd<LANES>
where
    LaneCount<LANES>: SupportedLaneCount,
{
    type Output = Simd<LANES>;
    fn mul(self, rhs: Self) -> Self::Output {
        Simd::Float(self.as_float() * rhs.as_float())
    }
}

impl<const LANES: usize> Div for Simd<LANES>
where
    LaneCount<LANES>: SupportedLaneCount,
{
    type Output = Simd<LANES>;
    fn div(self, rhs: Self) -> Self::Output {
        Simd::Float(self.as_float() / rhs.as_float())
    }
}
impl<const LANES: usize> Rem for Simd<LANES>
where
    LaneCount<LANES>: SupportedLaneCount,
{
    type Output = Simd<LANES>;
    fn rem(self, rhs: Self) -> Self::Output {
        Simd::Float(self.as_float() % rhs.as_float())
    }
}

impl<const LANES: usize> Simd<LANES>
where
    LaneCount<LANES>: SupportedLaneCount,
{
    pub(crate) fn pow(self, rhs: Self) -> Self {
        let lhs = self.as_float().to_array();
        let rhs = self.as_float().to_array();
        Simd::Float(lhs.zip(rhs).map(|(lhs, rhs)| lhs.powf(rhs)).into())
    }
}
/*
impl<const LANES: usize> Add for Simd<LANES>
where
    LaneCount<LANES>: SupportedLaneCount,
{
    type Output = Simd<LANES>;
    fn add(self, rhs: Self) -> Self::Output {
        match self {
            Simd::Int(l) => match rhs {
                Simd::Int(r) => Simd::Int(l + r),
                Simd::Float(r) => Simd::Float(<simd::Simd<f64, { LANES }>>::from(l) + r),
                Simd::Bool(r) => Simd::Int(l + r as i64),
            },
            Simd::Float(l) => match rhs {
                Simd::Int(r) => Simd::Float(l + r as f64),
                Simd::Float(r) => Simd::Float(l + r),
                Simd::Bool(r) => Simd::Float(l + r as i64 as f64),
            },
            Simd::Bool(l) => match rhs {
                Simd::Int(r) => Simd::Int(l as i64 + r),
                Simd::Float(r) => Simd::Float(l as i64 as f64 + r),
                Simd::Bool(r) => Simd::Int(l as i64 + r as i64),
            },
        }
    }
}
impl<const LANES: usize> Sub for Simd<LANES>
where
    LaneCount<LANES>: SupportedLaneCount,
{
    type Output = Simd<LANES>;

    fn sub(self, rhs: Self) -> Self::Output {
        match self {
            Simd::Int(l) => match rhs {
                Simd::Int(r) => Simd::Int(l - r),
                Simd::Float(r) => Simd::Float(l as f64 - r),
                Simd::Bool(r) => Simd::Int(l - r as i64),
            },
            Simd::Float(l) => match rhs {
                Simd::Int(r) => Simd::Float(l - r as f64),
                Simd::Float(r) => Simd::Float(l - r),
                Simd::Bool(r) => Simd::Float(l - r as i64 as f64),
            },
            Simd::Bool(l) => match rhs {
                Simd::Int(r) => Simd::Int(l as i64 - r),
                Simd::Float(r) => Simd::Float(l as i64 as f64 - r),
                Simd::Bool(r) => Simd::Int(l as i64 - r as i64),
            },
        }
    }
}
impl<const LANES: usize> Mul for Simd<LANES>
where
    LaneCount<LANES>: SupportedLaneCount,
{
    type Output = Simd<LANES>;
    fn mul(self, rhs: Self) -> Self::Output {
        match self {
            Simd::Int(l) => match rhs {
                Simd::Int(r) => Simd::Int(l * r),
                Simd::Float(r) => Simd::Float(l as f64 * r),
                Simd::Bool(r) => Simd::Int(l * r as i64),
            },
            Simd::Float(l) => match rhs {
                Simd::Int(r) => Simd::Float(l * r as f64),
                Simd::Float(r) => Simd::Float(l * r),
                Simd::Bool(r) => Simd::Float(l * r as i64 as f64),
            },
            Simd::Bool(l) => match rhs {
                Simd::Int(r) => Simd::Int(l as i64 * r),
                Simd::Float(r) => Simd::Float(l as i64 as f64 * r),
                Simd::Bool(r) => Simd::Int(l as i64 * r as i64),
            },
        }
    }
}

impl<const LANES: usize> Div for Simd<LANES>
where
    LaneCount<LANES>: SupportedLaneCount,
{
    type Output = Simd<LANES>;
    fn div(self, rhs: Self) -> Self::Output {
        match self {
            Simd::Int(l) => match rhs {
                Simd::Int(r) => Simd::Int(l / r),
                Simd::Float(r) => Simd::Float(l.try_into().unwrap() / r),
                Simd::Bool(r) => Simd::Int(l / r.into()),
            },
            Simd::Float(l) => match rhs {
                Simd::Int(r) => Simd::Float(l / r.into()),
                Simd::Float(r) => Simd::Float(l / r),
                Simd::Bool(r) => Simd::Float(l / r.into()),
            },
            Simd::Bool(l) => match rhs {
                Simd::Int(r) => Simd::Int(l.into() / r),
                Simd::Float(r) => Simd::Float(l.into() / r),
                Simd::Bool(r) => Simd::Int(l.into() / r.into()),
            },
        }
    }
}
impl<const LANES: usize> Rem for Simd<LANES>
where
    LaneCount<LANES>: SupportedLaneCount,
{
    type Output = Simd<LANES>;
    fn rem(self, rhs: Self) -> Self::Output {
        match self {
            Simd::Int(l) => match rhs {
                Simd::Int(r) => Simd::Int(l % r),
                Simd::Float(r) => Simd::Float(l as f64 % r),
                Simd::Bool(r) => Simd::Int(l % r as i64),
            },
            Simd::Float(l) => match rhs {
                Simd::Int(r) => Simd::Float(l % r as f64),
                Simd::Float(r) => Simd::Float(l % r),
                Simd::Bool(r) => Simd::Float(l % r as i64 as f64),
            },
            Simd::Bool(l) => match rhs {
                Simd::Int(r) => Simd::Int(l as i64 % r),
                Simd::Float(r) => Simd::Float(l as i64 as f64 % r),
                Simd::Bool(r) => Simd::Int(l as i64 % r as i64),
            },
        }
    }
}
*/
