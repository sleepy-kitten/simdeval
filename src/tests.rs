#[cfg(test)]
extern crate test;

use std::mem::size_of;

use fasteval::{Expression, Parser, Slab};

use crate::{lex::{tokens::Tokens, token::{Token, TokenKind}}, parse::{node::{Std, Node}, nodes::Nodes}};

#[test]
fn test_tokenizing_fast() {
    let expression = "std:sqrt(2)";
    let stream_fast = Tokens::from_string(expression).unwrap();
    let sum: usize = stream_fast.tokens().iter().map(|t| t.span()).sum();
    assert_eq!(expression.len(), sum as usize);
    println!("stream_fast: {:#?}", stream_fast);
}
fn tokenize_fast(expression: &str) {
    Tokens::from_string(expression).unwrap();
}
#[bench]
fn bench_tokenizing_fast(b: &mut test::Bencher) {
    let expression = "1/2+1*3^2+45*43231231541.35252";
    b.iter(|| test::black_box(tokenize_fast(expression)))
}

#[bench] 
fn bench_fasteval(b: &mut test::Bencher) {
    let expression = "1/2+1*3^2+45*43231231541.35252";
    b.iter( || test::black_box( {
        let mut slab = Slab::new();
        let parser = Parser::new();
        parser.parse(expression, &mut slab.ps)
    }))
}

#[test]
fn test_node_creation() {
    let expression = "1/2+1*3^2+45*4323123154135252";
    let tokens = Tokens::from_string(expression).unwrap();
    //println!("tokens: {:#?}", tokens);
    let nodes = tokens.try_to_nodes::<Std>().unwrap();
    //println!("nodes: {:#?}", nodes);
}
#[bench]
fn bench_node_creation(b: &mut test::Bencher) {
    let expression = "1/2+1*3^2+45*43231231541.35252";
    let tokens = Tokens::from_string(expression).unwrap();
    //b.iter(|| test::black_box(tokens.try_to_nodes::<Std>().unwrap()));
    b.iter(|| test::black_box(Tokens::from_string(expression).unwrap().try_to_nodes::<Std>().unwrap()));
}
#[test]
fn test_sizes() {
    println!("token:     {}", size_of::<Token>());
    println!("tokens:    {}", size_of::<Tokens>());
    println!("std:       {}", size_of::<Std>());
    println!("node:      {}", size_of::<Node<Std>>());
    println!("nodes:     {}", size_of::<Nodes<Std>>());
    println!("tokenkind: {}", size_of::<TokenKind>());
}