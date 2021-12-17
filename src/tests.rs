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
        value::{simd::Simd, single::Single, Value},
    },
    impl_functions, impl_functions_test,
    stack::Stack,
};

#[test]
fn test_parse_fast() {
    let expression = "print(2+6^2*4)";
    let mut expression = Expression::<Std<8>, 8>::new(expression);
    expression.compile().unwrap();
    expression.optimize().unwrap();

    //let temp = expression;
    println!("compiled: {:#?}", expression);
    expression.eval().unwrap();
}
#[bench]
fn bench_parse_fast(b: &mut test::Bencher) {
    let expression = "a+43*3-a+b^3";
    let mut test = Expression::<Std<8>, 8>::new(expression);
    b.iter(|| {
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
    let mut test = Expression::<Std<8>, 8>::new(expression);
    test.compile().unwrap();
    b.iter(|| {
        test.to_tokens().unwrap();
        test.set_expression(expression);
    })
}

#[bench]
fn bench_compile_to_nodes(b: &mut test::Bencher) {
    let expression = "a+43*3-a+b^3";
    let mut test = Expression::<Std<8>, 8>::new(expression);
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
    let mut test = Expression::<Std<8>, 8>::new(expression);
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
    println!("element:   {}", size_of::<ParseElement<Std<8>, 8>>());
    println!("node:      {}", size_of::<Node<Std<8>, 8>>());
    println!("token:     {}", size_of::<Token>());
}

#[test]
fn test_macro() {
    let mut expression = Expression::<Std<8>, 8>::new("std:sqrt(3+1+5+a)");
    expression.compile().unwrap();
    expression
        .set_variable("a", Value::Single(Single::Int(666)))
        .unwrap();
    // println!("{}", <Std::<8>>::MAX_ARGS);
    //let test = Std::Sqrt;
    //println!("{}", test.is_const());
}
#[test]
fn test_eval() {
    let mut expression = Expression::<Std<8>, 8>::new("sqrt(a)");
    expression.compile().unwrap();
    expression.optimize().unwrap();
    println!("expression: {:#?}", expression);

    expression
        .set_variable_by_index(0, Value::Single(Single::Float(4.0)))
        .unwrap();
    let result = expression.eval().unwrap();
    println!("result {:#?}", result);

    let start = Instant::now();
    for i in 0..=1000000 {
        expression
            .set_variable_by_index(0, Value::Single(Single::Float(i as f64)))
            .unwrap();
        //expression.set_variable("a", Value::Float(i as f64)).unwrap();
        let result = expression.eval();
    }
    let end = start.elapsed();

    println!("{}ms", end.as_millis());
}
#[bench]
fn bench_eval(b: &mut test::Bencher) {
    let mut expression = Expression::<Std<8>, 8>::new("2+6^a*4");
    expression.compile().unwrap();
    expression.optimize().unwrap();
    b.iter(|| test::black_box(expression.eval()))
}

#[bench]
fn bench_eval_fasteval(b: &mut test::Bencher) {
    use fasteval::Evaler;
    let parser = fasteval::Parser::new();
    let mut slab = fasteval::Slab::new();
    let expression = parser
        .parse("2+6^a*4", &mut slab.ps)
        .unwrap()
        .from(&slab.ps);
    let compiled = expression.compile(&slab.ps, &mut slab.cs);
    println!("{:#?}", compiled);
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
    let mut test = Expression::<Std<8>, 8>::new("a+b+2");
    let _ = test.compile();
    let elements = test.elements();
}

#[test]
fn test_full_speed() {
    let start = Instant::now();
    for i in 0..=1000000 {
        let mut test = Expression::<Std<8>, 8>::new("a+b+2");
        test.set_expression("3+1+5+3");
        test.compile().unwrap();
        test.optimize().unwrap();
        test.eval().unwrap();
    }
    let end = start.elapsed();
    println!("{}ms", end.as_millis());
}

#[test]
fn test_eval_simd_stack() {
    let expression = "1.0+a+43.0*3.0-b^3.0";
    let mut expression = Expression::<Std<8>, 8>::new(expression);
    expression.compile().unwrap();
    expression.optimize().unwrap();
    expression
        .set_variable_by_index(1, Value::Single(Single::Float(2.2)))
        .unwrap();
    expression.to_simd();
    let mut stack = Stack::<f64, 8_usize>::new();
    println!("{:#?}", expression);
    let start = Instant::now();
    for i in 0..=1000000 {
        stack.push(i as f64);
        if let Some(array) = stack.full_array() {
            expression
                .set_variable_by_index(0, Value::Simd(Simd::Float(array.into())))
                .unwrap();
            stack.clear();
            expression.eval().unwrap();
        }
    }
    let end = start.elapsed();
    println!("{}ms", end.as_millis())
}
#[test]
fn test_eval_simd() {
    let expression = "1.0+a+43.0*3.0-b^3.0";
    let mut expression = Expression::<Std<8>, 8>::new(expression);
    expression.compile().unwrap();
    expression.optimize().unwrap();
    expression
        .set_variable_by_index(1, Value::Single(Single::Float(2.2)))
        .unwrap();
    expression.to_simd();
    println!("{:#?}", expression);
    let start = Instant::now();
    for i in 0..=1000000 / 8 {
        let e = (i * 8) as f64;
        let array = [
            e,
            e + 1.0,
            e + 2.0,
            e + 3.0,
            e + 4.0,
            e + 5.0,
            e + 6.0,
            e + 7.0,
        ];
        expression
            .set_variable_by_index(0, Value::Simd(Simd::Float(array.into())))
            .unwrap();
        expression.eval().unwrap();
    }
    let end = start.elapsed();
    println!("{}ms", end.as_millis())
}

#[test]
fn test_eval_normal() {
    let expression = "1.0+a+43.0*3.0-b^3.0";
    let mut expression = Expression::<Std<8>, 8>::new(expression);
    expression.compile().unwrap();
    expression.optimize().unwrap();
    expression
        .set_variable_by_index(1, Value::Single(Single::Float(2.2)))
        .unwrap();
    println!("{:#?}", expression);
    let start = Instant::now();
    for i in 0..=1000000 {
        expression
            .set_variable_by_index(0, Value::Single(Single::Int(i)))
            .unwrap();
        expression.eval().unwrap();
    }
    let end = start.elapsed();
    println!("{}ms", end.as_millis())
}
