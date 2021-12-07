#[cfg(test)]
extern crate test;

use crate::parser::token_stream::TokenStream;

#[test]
fn test_tokenizing_fast() {
    let expression = "1000000000000000000000000000000000000000000000000000000";
    let stream_fast = expression.parse::<TokenStream>().unwrap();
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
