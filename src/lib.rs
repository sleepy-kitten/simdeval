#![feature(test)]

#![feature(maybe_uninit_slice)]
#![feature(maybe_uninit_extra)]
#![feature(maybe_uninit_array_assume_init)]

#![feature(more_qualified_paths)]
#![feature(portable_simd)]
#![feature(generic_const_exprs)]
#![feature(array_zip)]


#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]


//! # Simdeval
//!
//! `simdeval` is a crate that allows you to evaluate expressions at runtime.
//! 
//! it is optimized for expressions that will be evaluated multiple times in quick succession
//! with different values and includes support for custom functions written in rust which can be 
//! invoked using function like syntax.

pub use error::*;
pub use evaluate::expression::Expression;
pub use evaluate::function::std::Std;

#[macro_use]

pub mod error;
pub mod stack;
pub mod evaluate;
mod tests;
mod small_string;
