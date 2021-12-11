#[cfg(test)]
extern crate test;

use std::mem::{align_of, size_of};

use fasteval::{Parser, Slab};

use crate::{
    lex::{
        token::{Token, TokenKind},
        tokens::Tokens,
    },
    parse::{
        node::{Node, Std, Test},
        nodes::Nodes,
    },
    parse_new::{parse_element::ParseElement, expression::Expression},
};

#[test]
fn test_tokenizing() {
    let expression = "std:sqrt(2)";
    let stream_fast = Tokens::try_from_string(expression).unwrap();
    let sum: usize = stream_fast.tokens().iter().map(|t| t.span()).sum();
    assert_eq!(expression.len(), sum as usize);
    println!("stream_fast: {:#?}", stream_fast);
}
fn tokenize(expression: &str) {
    Tokens::try_from_string(expression).unwrap();
}
#[bench]
fn bench_tokenizing(b: &mut test::Bencher) {
    let expression = "1/2+1*3^2+45*43231231541.35252";
    b.iter(|| test::black_box(tokenize(expression)))
}

#[bench]
fn bench_fasteval(b: &mut test::Bencher) {
    let expression = "1/2+1*3^2+45*43231231541.35252";
    b.iter(|| {
        test::black_box({
            let mut slab = Slab::new();
            let parser = Parser::new();
            parser.parse(expression, &mut slab.ps)
        })
    })
}

#[bench]
fn bench_evalexpr(b: &mut test::Bencher) {
    let expression = "1/2+1*3^2+45*43231231541.35252";
    b.iter(|| test::black_box(evalexpr::build_operator_tree(expression)))
}

#[test]
fn test_node_creation() {
    //let expression = "a/2+1*3^2+45*4323123154135252";
    let expression = "1*(3+2)";
    //                     0123456
    let tokens = Tokens::try_from_string(expression).unwrap();
    println!("tokens: {:#?}", tokens);
    let mut nodes = tokens.try_to_nodes::<Test>().unwrap();
    println!("nodes: {:#?}", nodes);
    nodes.set_indices();
    println!("nodes: {:#?}", nodes);
}
#[bench]
fn bench_node_creation(b: &mut test::Bencher) {
    let expression = "1/2+1*3^2+45*43231231541.35252";
    //let tokens = Tokens::try_from_string(expression).unwrap();
    //b.iter(|| test::black_box(tokens.try_to_nodes::<Std>().unwrap()));
    b.iter(|| {
        test::black_box(
            Tokens::try_from_string(expression)
                .unwrap()
                .try_to_nodes::<Std>()
                .unwrap()
                .set_indices(),
        )
    });
}

#[test]
fn test_parse_fast() {
    let expression = "!rer";
    let mut expression = Expression::<crate::parse_new::std::Std>::new(expression);
    expression.compile().unwrap();
    println!("compiled: {:#?}", expression);
    
}
#[bench]
fn bench_parse_fast(b: &mut test::Bencher) {
    let expression = "1/2+1*3^2+45*43231231541.35252";
    b.iter(|| {
        test::black_box({
            let mut test = Expression::<crate::parse_new::std::Std>::new(expression);
            test.compile()
                .unwrap();
                //.to_nodes()
                //.unwrap()
                //.set_indices()
                //.unwrap();
        })
    });
}

#[test]
fn test_sizes() {
    println!("token:     {}", size_of::<Token>());
    println!("tokens:    {}", size_of::<Tokens>());
    println!("std:       {}", size_of::<Std>());
    println!("node:      {}", size_of::<Node<Std>>());
    println!("nodes:     {}", size_of::<Nodes<Std>>());
    println!("tokenkind: {}", size_of::<TokenKind>());
    println!("str:       {}", size_of::<&str>());
    println!();
    println!("token:     {}", align_of::<Token>());
    println!("tokens:    {}", align_of::<Tokens>());
    println!("std:       {}", align_of::<Std>());
    println!("node:      {}", align_of::<Node<Std>>());
    println!("nodes:     {}", align_of::<Nodes<Std>>());
    println!("tokenkind: {}", align_of::<TokenKind>());
}

#[test]
fn test_fast_sizes() {
    println!("element:   {}", size_of::<ParseElement<crate::parse_new::std::Std>>());
    println!();
    println!("element:   {}", align_of::<ParseElement<crate::parse_new::std::Std>>());
}
