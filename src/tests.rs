#[cfg(test)]
extern crate test;

use std::mem::{align_of, size_of};

use crate::{
    parse_new::{expression::Expression, parse_element::ParseElement},
};

#[test]
fn test_parse_fast() {
    let expression = "1/2+1*3^2+45*43231231541.35252";
    let mut expression = Expression::<crate::parse_new::std::Std>::new(expression);
    expression.compile().unwrap();
    //let temp = expression;
    println!("compiled: {:#?}", expression);
}
#[bench]
fn bench_parse_fast(b: &mut test::Bencher) {
    let expression = "1/2+1*3^2+45*43231231541.35252";
    b.iter(|| {
        test::black_box({
            let mut test = Expression::<crate::parse_new::std::Std>::new(expression);
            test.compile().unwrap();
            //.to_nodes()
            //.unwrap()
            //.set_indices()
            //.unwrap();
        })
    });
}

#[test]
fn test_fast_sizes() {
    println!(
        "element:   {}",
        size_of::<ParseElement<crate::parse_new::std::Std>>()
    );
    println!();
    println!(
        "element:   {}",
        align_of::<ParseElement<crate::parse_new::std::Std>>()
    );
}
