#![feature(test)]
//#![allow(dead_code)]

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

mod tests;
