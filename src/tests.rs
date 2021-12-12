#[cfg(test)]
extern crate test;

use std::mem::{align_of, size_of};

use crate::{
    evaluate::{expression::Expression, function::std::Std, parse_element::ParseElement},
    functions,
};

#[test]
fn test_parse_fast() {
    let expression = "1/2+1*3^2+45*43231231541.35252";
    let mut expression = Expression::<Std>::new(expression);
    expression.compile().unwrap();
    //let temp = expression;
    println!("compiled: {:#?}", expression);
}
#[bench]
fn bench_parse_fast(b: &mut test::Bencher) {
    let expression = "1/2+1*3^2+45*43231231541.35252";
    b.iter(|| {
        test::black_box({
            let mut test = Expression::<Std>::new(expression);
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
    println!("element:   {}", size_of::<ParseElement<Std>>());
    println!();
    println!("element:   {}", align_of::<ParseElement<Std>>());
}

functions!(
    Foo; 
    bar() -> {Value::Int(12)}; baz() -> {Value::Int(23)}, false);

#[test]
fn test_macro() {
    let bar = Foo::bar;
    let baz = Foo::baz;
    let test = baz.is_const();
    let test = Foo::NAMESPACE;
    println!("test: {}", test);
}
