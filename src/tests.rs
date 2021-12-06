#[cfg(test)]
extern crate test;
use test::Bencher;

use crate::parser::{partial_token_stream::PartialTokenStream, token_stream::TokenStream};
#[test]
fn test_tokenizing() {
    let expression = "a*b+c = 8434.3.3";
    let stream = expression.parse::<PartialTokenStream>().unwrap();
    let stream = TokenStream::from(stream);
    println!("stream: {:#?}", stream);
    println!("{:#?}", stream);
}
fn tokenize(expression: &str) {
    TokenStream::from(expression.parse::<PartialTokenStream>().unwrap());
}

#[bench]
fn bench_tokenizing(b: &mut Bencher) {
    let expression = "m*x+b*10000-52+waw";
    b.iter(|| {
            test::black_box(tokenize(expression))
    })
}
