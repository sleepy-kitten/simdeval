#[cfg(test)]
extern crate test;

use fasteval::{Expression, Parser, Slab};

use crate::lex::tokens::Tokens;

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
    let expression = "1/2+a*t^2+45*gwagwagwa";
    b.iter(|| test::black_box(tokenize_fast(expression)))
}

#[bench] 
fn bench_fasteval(b: &mut test::Bencher) {
    let expression = "1/2+a*t^2+45*gwagwagwa";
    b.iter( || test::black_box( {
        let mut slab = Slab::new();
        let parser = Parser::new();
        parser.parse(expression, &mut slab.ps)
    }))
}