use crate::{error::SimdevalError, stack::Stack};

use super::{
    enums::{Bracket, Identifier, Literal, Operator, Special, TokenKind},
    function::Function,
    node::Node,
    parse_element::ParseElement,
    token::Token,
    value::single::Value,
    variables::Variables,
};
use std::{cmp::Ordering, collections::HashMap, fmt::Debug};

#[derive(Debug)]

/// An `Expression` contains the expression string and the compiled version of that expression.
///
/// # UB
/// Compiling the same expression multiple times is UB
pub(crate) struct Expression<'a, T>
where
    T: Function<T>,
{
    elements: Vec<ParseElement<'a, T>>,
    variables: Variables<'a>,
    expression: &'a str,
    top_node: usize,
}

impl<'a, 'b, T: Function<T>> Expression<'a, T> {
    pub(crate) fn set_indices(&'b mut self) -> Result<(), SimdevalError> {
        struct HighestWeight {
            weight: i16,
            index: usize,
        }
        struct NodeInfo<'a, 'b, T>
        where
            T: Function<T>,
        {
            index: usize,
            node: &'b mut Node<'a, T>,
            weight: i16,
        }
        let mut bracket_weight = 0;
        let mut highest_weight = HighestWeight {
            weight: 0,
            index: 0,
        };
        let mut iter = self
            .elements
            .iter_mut()
            .enumerate()
            .filter_map(|(index, e)| match e {
                ParseElement::<'a, T>::Node(node) => match node {
                    <Node<T>>::Instruction {
                        operator: o,
                        lhs,
                        rhs,
                    } => {
                        *lhs = index - 1;
                        *rhs = index + 1;
                        let weight = o.weight();
                        let info = NodeInfo::<'a, 'b, T> {
                            index,
                            node,
                            weight: weight + bracket_weight,
                        };
                        Some(info)
                    }
                    _ => None,
                },
                ParseElement::Token(t) => match t.kind() {
                    TokenKind::Bracket(b) => {
                        bracket_weight += b.weight();
                        None
                    }
                    _ => None,
                },
            })
            .peekable();
        loop {
            let c = iter.next();
            let n = iter.peek_mut();
            if let Some(next) = n {
                let curr = c.unwrap();
                let ord = curr.weight.cmp(&next.weight);

                match ord {
                    Ordering::Equal => {
                        let (lhs, _) = next.node.as_mut_instruction_indices();
                        *lhs = curr.index;
                        if highest_weight.weight <= curr.weight {
                            highest_weight.weight = curr.weight;
                            highest_weight.index = curr.index;
                        }
                    }
                    Ordering::Greater => {
                        let (lhs, _) = next.node.as_mut_instruction_indices();
                        *lhs = curr.index;
                        if highest_weight.weight <= curr.weight {
                            highest_weight.weight = curr.weight;
                            highest_weight.index = curr.index;
                        }
                    }
                    Ordering::Less => {
                        let (_, rhs) = curr.node.as_mut_instruction_indices();
                        *rhs = next.index;
                        if highest_weight.weight <= next.weight {
                            highest_weight.weight = next.weight;
                            highest_weight.index = next.index;
                        }
                    }
                }
            } else {
                break;
            }
        }
        self.top_node = highest_weight.index;
        Ok(())
    }
    /// Get a reference to the expression's elements.
    pub(crate) fn elements(&self) -> &[ParseElement<T>] {
        self.elements.as_ref()
    }
}
impl<'a, T: Function<T>> Expression<'a, T>
where
    T: Function<T> + Clone + Debug,
{
    #[inline]
    pub fn new(expression: &'a str) -> Self {
        Self {
            elements: Vec::with_capacity(expression.len()),
            variables: Variables::with_capacity(expression.len() / 2),
            expression,
            top_node: 0,
        }
    }
    pub fn clear(&mut self) {
        self.elements.clear();
        self.variables.clear()
    }
    pub fn set_expression(&mut self, expression: &'a str) {
        self.clear();
        self.expression = expression;
    }
    #[inline]
    pub fn compile(&mut self) -> Result<(), SimdevalError> {
        self.to_tokens()?.to_nodes::<4, 16>()?.set_indices()
    }
    pub fn optimize(&mut self) {}
    fn new_token_from_kind(&mut self, token_kind: TokenKind, start: usize) {
        let token = Token::new(token_kind, start);
        self.elements.push(ParseElement::Token(token));
    }
    pub fn set_variable(&mut self, identifier: &'a str, value: Value) -> Result<(), SimdevalError> {
        self.variables.set(identifier, value)
    }
    fn new_token(&mut self, chr: u8, start: usize) -> Result<(), SimdevalError> {
        let token_kind = match chr {
            b'0'..=b'9' => TokenKind::Literal(Literal::Int),

            b'a'..=b'z' | b'A'..=b'Z' => TokenKind::Identifier(Identifier::Variable),

            b'+' => TokenKind::Operator(Operator::Add),
            b'-' => TokenKind::Operator(Operator::Sub),
            b'*' => TokenKind::Operator(Operator::Mul),
            b'/' => TokenKind::Operator(Operator::Div),
            b'%' => TokenKind::Operator(Operator::Mod),
            b'^' => TokenKind::Operator(Operator::Pow),

            b'=' => TokenKind::Operator(Operator::Equal),
            b'!' => TokenKind::Operator(Operator::Not),
            b'>' => TokenKind::Operator(Operator::Greater),
            b'<' => TokenKind::Operator(Operator::Smaller),
            b'&' => TokenKind::Operator(Operator::And),
            b'|' => TokenKind::Operator(Operator::Or),
            b'#' => TokenKind::Operator(Operator::Xor),

            b'(' => TokenKind::Bracket(Bracket::Opened),
            b')' => TokenKind::Bracket(Bracket::Closed),

            b'.' => TokenKind::Literal(Literal::Float),

            _ => return Err(SimdevalError::UnkownCharacter(chr as char)),
        };
        let token = Token::new(token_kind, start);
        self.elements.push(ParseElement::Token(token));
        Ok(())
    }
    fn insert_neg(&mut self, index: usize) {
        self.elements
            .push(ParseElement::Token(Token::new_neg_zero()));
        self.elements.push(ParseElement::Token(Token::new(
            TokenKind::Operator(Operator::Sub),
            index,
        )))
    }
    pub(crate) fn to_tokens(&mut self) -> Result<&mut Self, SimdevalError> {
        for (index, &chr) in self.expression.as_bytes().iter().enumerate() {
            self.push(chr, index)?;
        }
        Ok(self)
    }
    pub(crate) fn to_nodes<const N: usize, const V: usize>(
        &mut self,
    ) -> Result<&mut Self, SimdevalError> {
        let mut namespaces = Stack::<&str, N>::new();
        let string = self.expression;
        for element in &mut self.elements {
            if let ParseElement::Token(token) = element {
                match token.kind() {
                    TokenKind::Literal(l) => {
                        let value = match l {
                            Literal::Bool => Value::Bool(token.slice(string).parse::<bool>()?),
                            Literal::Float => Value::Float(token.slice(string).parse::<f64>()?),
                            Literal::Int => Value::Int(token.slice(string).parse::<i64>()?),
                        };
                        *element = ParseElement::Node(Node::Literal(value));
                    }
                    TokenKind::Operator(o) => {
                        *element = ParseElement::Node(Node::Instruction {
                            operator: o,
                            lhs: 0,
                            rhs: 0,
                        })
                    }
                    TokenKind::Identifier(Identifier::Function) => {
                        let identifier = token.slice(string);
                        let mut namespaces = namespaces.iter();
                        let function =
                            <T as Function<T>>::from_string(&mut namespaces, identifier)?;
                        *element = ParseElement::Node(Node::Function {
                            function,
                            args: None,
                        })
                    }
                    TokenKind::Bracket(_) | TokenKind::Special(Special::Comma) => (),
                    TokenKind::Special(Special::Namespace) => {
                        namespaces.push(token.slice(string));
                    }
                    TokenKind::Special(Special::NegZero) => {
                        *element = ParseElement::Node(Node::Literal(Value::Int(0)));
                    }
                    TokenKind::Identifier(Identifier::Variable) => {
                        let identifier = token.slice(string);
                        let index = self.variables.find_or_set(identifier);
                        *element = ParseElement::Node(Node::Variable { index });
                    }
                };
            }
        }
        Ok(self)
    }
    fn push(&mut self, chr: u8, index: usize) -> Result<(), SimdevalError> {
        if let Some(ParseElement::Token(token)) = self.elements.last_mut() {
            match chr {
                b' ' => (),
                b':' => match token.kind() {
                    TokenKind::Identifier(_) => {
                        token.set_kind(TokenKind::Special(Special::Namespace));
                    }
                    _ => return Err(SimdevalError::UnexpectedToken),
                },
                b'0'..=b'9' => match token.kind() {
                    TokenKind::Identifier(_) | TokenKind::Literal(_) => token.inc_end(),
                    _ => self.new_token_from_kind(TokenKind::Literal(Literal::Int), index),
                },
                b'a'..=b'z' | b'A'..=b'Z' => match token.kind() {
                    TokenKind::Identifier(_) => token.inc_end(),
                    _ => {
                        self.new_token_from_kind(TokenKind::Identifier(Identifier::Variable), index)
                    }
                },
                b'.' => match token.kind() {
                    TokenKind::Literal(Literal::Int) => {
                        token.set_inc(TokenKind::Literal(Literal::Float))
                    }
                    _ => self.new_token_from_kind(TokenKind::Literal(Literal::Float), index),
                },
                b'+' | b'-' | b'*' | b'/' | b'%' | b'^' | b'&' | b'|' | b'!' | b'=' | b'<'
                | b'>' | b'#' => match (chr, token.kind()) {
                    (b'-', TokenKind::Operator(_) | TokenKind::Bracket(_)) => {
                        self.insert_neg(index);
                    }
                    (b'>', TokenKind::Operator(Operator::Equal)) => {
                        token.set_inc(TokenKind::Operator(Operator::GreaterEqual));
                    }
                    (b'<', TokenKind::Operator(Operator::Equal)) => {
                        token.set_inc(TokenKind::Operator(Operator::SmallerEqual));
                    }
                    (b'!', TokenKind::Operator(Operator::Equal)) => {
                        token.set_inc(TokenKind::Operator(Operator::NotEqual));
                    }
                    _ => self.new_token(chr, index)?,
                },
                b'(' | b')' => match (chr, token.kind()) {
                    (b'(', TokenKind::Identifier(_)) => {
                        token.set_kind(TokenKind::Identifier(Identifier::Function));
                        self.new_token_from_kind(TokenKind::Bracket(Bracket::Opened), index)
                    }
                    _ => self.new_token_from_kind(TokenKind::Bracket(Bracket::Opened), index),
                },
                b',' => self.new_token_from_kind(TokenKind::Special(Special::Comma), index),
                _ => return Err(SimdevalError::UnexpectedToken),
            }
        } else {
            if chr == b'-' {
                self.insert_neg(index);
            } else {
                self.new_token(chr, index)?;
            }
        }
        Ok(())
    }
}

impl<'a, T: Function<T>> Expression<'a, T>
where
    T: Function<T> + Clone + Debug,
{
    pub fn eval(&mut self) -> Result<Value, SimdevalError> {
        /*
        let mut current = self.top_node;
        loop {
            if let ParseElement::Token(_) = self.elements[current] {
                panic!("referenced node was a token")
            }
            let children = self.get_operands(current);
            if let Some(children) = children {
                let current = children.0;
                self.eval_stack.push(children.1);
            }
        }
        */
        Ok(self.eval_recursive())
    }
    fn eval_recursive(&self) -> Value {
        self.eval_at_recursive(self.top_node)
    }
    fn eval_at_recursive(&self, index: usize) -> Value {
        if let ParseElement::Node(n) = &self.elements[index] {
            match n {
                Node::Instruction { operator, lhs, rhs } => {
                    operator.eval(self.eval_at_recursive(*lhs), self.eval_at_recursive(*rhs))
                }
                Node::Literal(value) => *value,
                Node::Variable { index } => self.variables[*index],
                Node::Function { function, args } => todo!(),
            }
        } else {
            panic!()
        }
    }
    fn get_operands(&self, index: usize) -> Option<(usize, usize)> {
        self.elements[index].get_operands_indices()
    }
}
