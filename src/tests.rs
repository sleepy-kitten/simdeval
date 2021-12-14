#[cfg(test)]
extern crate test;

use std::{mem::{align_of, size_of}, collections::BTreeMap};

use fasteval::Compiler;

use crate::{
    evaluate::{
        enums::Value,
        expression::Expression,
        function::{std::Std, Function},
        node::Node,
        parse_element::ParseElement,
        token::Token,
    },
    impl_functions,
};

#[test]
fn test_parse_fast() {
    let expression = "a+43*3-a+b^3";
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
    println!("node:      {}", size_of::<Node<Std>>());
    println!("token:     {}", size_of::<Token>());
    println!();
    println!("element:   {}", align_of::<ParseElement<Std>>());
}
fn root(value: &[Value; 2]) -> Value {
    todo!()
}
fn sqrt(value: &[Value; 1]) -> Value {
    todo!()
}
impl_functions!(Foo: foo; [Std: std]; [Root: root, Sqrt: sqrt]);
impl_functions!(Bar: bar; [Std: std, Foo: foo]; [Root: root, Sqrt: sqrt]);
#[test]
fn test_eval() {
    let mut expression = Expression::<Bar>::new("3+1+5");
    expression.compile().unwrap();
    println!("expression: {:#?}", expression);
    let result = expression.eval().unwrap();
    println!("result {:#?}", result);
}
#[bench]
fn bench_eval(b: &mut test::Bencher) {
    let mut expression = Expression::<Std>::new("2+6^2*4");
    expression.compile().unwrap();
    b.iter(||test::black_box(expression.eval()))
}

#[bench]
fn bench_eval_fasteval(b: &mut test::Bencher) {
    use fasteval::Evaler;
    let parser = fasteval::Parser::new();
    let mut slab = fasteval::Slab::new();
    let expression = parser.parse("2+6^2*4", &mut slab.ps).unwrap().from(&slab.ps);
    let compiled = expression.compile(&slab.ps, &mut slab.cs);
    b.iter(|| test::black_box(compiled.eval(&slab, &mut fasteval::evalns::EmptyNamespace)))
}

#[test]
fn test_handle_thing() {
    let mut test = Expression::<Std>::new("a+b+2");
    let _ = test.compile();
    let elements = test.elements();
}
