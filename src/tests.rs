#[cfg(test)]
extern crate test;

use std::{
    collections::BTreeMap,
    mem::{align_of, size_of},
    time::Instant,
};

use fasteval::Compiler;

use crate::{
    biggest,
    evaluate::{
        expression::Expression,
        function::{std::Std, Function},
        node::Node,
        parse_element::ParseElement,
        token::Token,
        value::single::Value,
    },
    impl_functions, impl_functions_test,
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
    let expression = "a+43*3-a+b^3";
    let mut test = Expression::<Std>::new(expression);
    b.iter(move || {
        test::black_box({
            test.compile().unwrap();
            test.set_expression(expression)
            //.to_nodes()
            //.unwrap()
            //.set_indices()
            //.unwrap();
        })
    });
}

#[bench]
fn bench_compile_to_tokens(b: &mut test::Bencher) {
    let expression = "a+43*3-a+b^3";
    let mut test = Expression::<Std>::new(expression);
    test.compile().unwrap();
    b.iter(|| {
        test.to_tokens().unwrap();
        test.set_expression(expression);
    })
}

#[bench]
fn bench_compile_to_nodes(b: &mut test::Bencher) {
    let expression = "a+43*3-a+b^3";
    let mut test = Expression::<Std>::new(expression);
    test.compile().unwrap();
    b.iter(|| {
        test.to_tokens().unwrap();
        test.to_nodes::<4>().unwrap();
        test.set_expression(expression);
    })
}

#[bench]
fn bench_compile_set_indices(b: &mut test::Bencher) {
    let expression = "a+43*3-a+b^3";
    let mut test = Expression::<Std>::new(expression);
    test.compile().unwrap();
    b.iter(|| {
        test.to_tokens().unwrap();
        test.to_nodes::<4>().unwrap();
        test.set_indices().unwrap();
        test.set_expression(expression);
    })
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
impl_functions!(Foo: foo; [Std: std]; [Root: root, 2, Sqrt: sqrt, 1]);
impl_functions!(Bar: bar; [Std: std, Foo: foo]; [Root: root, 2, Sqrt: sqrt, 1]);
impl_functions_test!(
    Baz: baz;
    [Std: std];
    [test(2) {
        Value::Int(0);
        Value::Int(9)
    }]
);
#[test]
fn test_macro() {
    let mut expression = Expression::<Baz>::new("baz:test(3+1+5+a)");
    expression.compile().unwrap();
    expression.set_variable("a", Value::Int(666)).unwrap();
    println!("{}", Baz::MAX_ARGS)
}
#[test]
fn test_eval() {
    let mut expression = Expression::<Bar>::new("3+1+5+a");
    expression.compile().unwrap();
    expression.set_variable("a", Value::Int(666)).unwrap();
    println!("expression: {:#?}", expression);
    let result = expression.eval().unwrap();
    println!("result {:#?}", result);
    let start = Instant::now();
    for i in 0..=1000000 {
        let _ = expression.eval();
    }
    let end = start.elapsed();
    println!("{}ms", end.as_millis());
}
#[bench]
fn bench_eval(b: &mut test::Bencher) {
    let mut expression = Expression::<Std>::new("2+6^2*4");
    expression.compile().unwrap();
    b.iter(|| test::black_box(expression.eval()))
}

#[bench]
fn bench_eval_fasteval(b: &mut test::Bencher) {
    use fasteval::Evaler;
    let parser = fasteval::Parser::new();
    let mut slab = fasteval::Slab::new();
    let expression = parser
        .parse("2+6^2*4", &mut slab.ps)
        .unwrap()
        .from(&slab.ps);
    let compiled = expression.compile(&slab.ps, &mut slab.cs);
    b.iter(|| test::black_box(compiled.eval(&slab, &mut fasteval::evalns::EmptyNamespace)))
}
#[bench]
fn bench_parse_fasteval(b: &mut test::Bencher) {
    use fasteval::Evaler;
    let parser = fasteval::Parser::new();
    let mut slab = fasteval::Slab::new();
    let expression = parser
        .parse("2+6^2*4", &mut slab.ps)
        .unwrap()
        .from(&slab.ps);
    let compiled = expression.compile(&slab.ps, &mut slab.cs);
    b.iter(|| {
        test::black_box({
            parser
                .parse("2+6^2*4", &mut slab.ps)
                .unwrap()
                .from(&slab.ps)
                .compile(&slab.ps, &mut slab.cs);
        })
    })
}

#[test]
fn test_handle_thing() {
    let mut test = Expression::<Std>::new("a+b+2");
    let _ = test.compile();
    let elements = test.elements();
}

#[test]
fn test_full() {
    let start = Instant::now();
    for i in 0..=1000000 {
        let mut test = Expression::<Std>::new("a+b+2");
        test.set_expression("3+1+5+3");
        let _ = test.compile();
        let _ = test.eval();
    }
    let end = start.elapsed();
    println!("{}ms", end.as_millis());
}

#[test]
fn test_simd() {
    #[cfg(all(
        any(target_arch = "x86", target_arch = "x86_64"),
        target_feature = "avx2"
    ))]
    println!("avx2");

    #[cfg(not(all(
        any(target_arch = "x86", target_arch = "x86_64"),
        target_feature = "avx2"
    )))]
    println!("not");
}
