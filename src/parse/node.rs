use std::str::FromStr;

use crate::{error::SimdevalError, lex::token_stream::TokenStream};

/// a function trait, intended to be used with enums where each variant represents a function
pub(crate) trait Function<T>
where
    T: Function<T>,
    T: FromStr,
{
    /// prefix for faster parsing of function identifiers
    const NAMESPACE: &'static str;
    /// calls a function out of the possible functions of `T`
    fn call(&self, node: &[Node<T>]) -> Literal;
    /// whether a function returns the same solution each time it is called
    /// `true` by default so it can be evaluated during compilation
    fn is_const(&self) -> bool {
        true
    }
}
pub(crate) enum Node<T>
where
    T: Function<T>,
    T: FromStr,
{
    Operator(Operator, Box<Node<T>>, Box<Node<T>>),
    Literal(Literal),
    Variable(Variable),
    Function(T, Vec<Node<T>>),
}

pub(crate) enum Operator {
    Add = 1,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
    Log,
}
pub(crate) enum Literal {
    Int(u64),
    Float(f64),
    Bool(bool),
    String(String),
}
pub(crate) struct Variable {
    identifier: String,
    index: usize,
}

pub enum Std {
    NaturalLog,
    SquareRoot,
}
impl Function<Std> for Std {
    const NAMESPACE: &'static str = "std";
    fn call(&self, _: &[Node<Std>]) -> Literal {
        match self {
            Self::NaturalLog => Literal::Int(313),
            Self::SquareRoot => Literal::Float(2.14),
        }
    }
}
impl FromStr for Std {
    type Err = SimdevalError;
    fn from_str(identifier: &str) -> Result<Self, Self::Err> {
        Ok(match identifier {
            "log" => Self::NaturalLog,
            "sqrt" => Self::SquareRoot,
            _ => return Err(Self::Err::NoIdentifierMatch),
        })
    }
}
fn test_2() {
    let node = Node::<Std>::Function(Std::SquareRoot, Vec::new());
}
fn test(mut stream: TokenStream) {
    for i in 1..stream.len() - 1 {}
}
// 1 * 4 + ( 6 ^ 3 ^ 2 + 4 )
// 2 2 2 1 4 7 7 7 7 7 5 5 4

// * + ^ ^ +
// 2 1 7 7 5
