#[cfg(test)]
extern crate test;

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
    #[allow(unused_must_use)] {
        TokenStream::from(expression.parse::<PartialTokenStream>().unwrap());
    }
}

#[bench]
fn bench_tokenizing(b: &mut test::Bencher) {
    let expression = "1000000000000000000000000000000000000000000000000000000";
    b.iter(|| test::black_box(tokenize(expression)))
}

#[test]
fn test_tokenizing_fast() {
    let expression = "1000000000000000000000000000000000000000000000000000000";
    let stream_fast = expression.parse::<TokenStream>().unwrap();
    let stream = TokenStream::from(expression.parse::<PartialTokenStream>().unwrap());
    assert_eq!(stream_fast, stream);
    println!("stream_fast: {:#?}", stream_fast);
}
fn tokenize_fast(expression: &str) {
    expression.parse::<TokenStream>().unwrap();
}
#[bench]
fn bench_tokenizing_fast(b: &mut test::Bencher) {
    let expression = "1000000000000000000000000000000000000000000000000000000";
    b.iter(|| test::black_box(tokenize_fast(expression)))
}
