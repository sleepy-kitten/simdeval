#![feature(test)]
#![feature(maybe_uninit_slice)]
#![feature(maybe_uninit_extra)]
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


mod lex;
mod error;
mod parse;
mod stack;

mod tests;
