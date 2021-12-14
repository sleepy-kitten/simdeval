use std::{
    ops::{Add, Div, Mul, Rem, Sub},
    simd::{self, i64x8, f64x8, LaneCount, SupportedLaneCount},
};

pub enum Simd64<const LANES: usize> 
where LaneCount<LANES>: SupportedLaneCount{
    Int(simd::Simd<i64, LANES>),
    Float(simd::Simd<f64, LANES>),
    Bool(()),
}