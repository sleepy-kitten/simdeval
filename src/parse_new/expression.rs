use crate::{error::SimdevalError, stack::Stack};

use super::{
    enums::{Bracket, Identifier, Literal, Operator, Special, TokenKind, Value},
    function::Function,
    node::Node,
    parse_element::ParseElement,
    token::Token,
};
use std::{cmp::Ordering, fmt::Debug};

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
    expression: &'a str,
}

impl<'a, T: Function<T>> Expression<'a, T>
where
    T: Function<T> + Clone,
{
    #[inline]
    pub fn new(expression: &'a str) -> Self {
        Self {
            elements: Vec::with_capacity(expression.len()),
            expression,
        }
    }
    #[inline]
    pub fn compile(&mut self) -> Result<(), SimdevalError> {
        self.to_tokens()?.to_nodes()?.set_indices()
    }
    pub fn eval(&self) -> Result<Value, SimdevalError> {
        
    }
    fn new_token_from_kind(&mut self, token_kind: TokenKind, start: usize) {
        let token = Token::new(token_kind, start);
        self.elements.push(ParseElement::Token(token));
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
    pub(crate) fn to_nodes(&mut self) -> Result<&mut Self, SimdevalError> {
        let mut namespaces = Stack::<&str, 4>::new();
        let string = self.expression;
        for element in &mut self.elements {
            if let ParseElement::Token(token) = element {
                match token.kind() {
                    TokenKind::Literal(_) | TokenKind::Operator(_) | TokenKind::Identifier(_) => {
                        element.to_node_clone(&mut namespaces.iter(), string)?
                    }
                    TokenKind::Bracket(b) => (),
                    TokenKind::Special(Special::Namespace) => {
                        namespaces.push(token.slice(string));
                    }
                    TokenKind::Special(Special::NegZero) => {
                        *element = ParseElement::Node(Node::Literal(Value::Int(0)));
                    }
                };
            }
        }
        Ok(self)
    }
    pub(crate) fn set_indices(&mut self) -> Result<(), SimdevalError> {
        let mut weight = 0;
        let mut iter = self
            .elements
            .iter_mut()
            .enumerate()
            .filter_map(|(i, e)| match e {
                ParseElement::Node(n) => match n {
                    <Node<T>>::Instruction {
                        operator: o,
                        lhs,
                        rhs,
                    } => {
                        *lhs = i - 1;
                        *rhs = i + 1;
                        Some((i, n, weight))
                    }
                    _ => None,
                },
                ParseElement::Token(t) => match t.kind() {
                    TokenKind::Bracket(b) => {
                        weight += b.weight();
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
                let curr_weight = curr.1.weight() + curr.2;
                let next_weight = next.1.weight() + next.2;
                let ord = curr_weight.cmp(&next_weight);

                match ord {
                    Ordering::Equal | Ordering::Greater => {
                        let (lhs, _) = next.1.as_mut_instruction_indices().unwrap();
                        *lhs = curr.0;
                    }
                    Ordering::Less => {
                        let (_, rhs) = curr.1.as_mut_instruction_indices().unwrap();
                        *rhs = next.0;
                    }
                }
            } else {
                break;
            }
        }
        Ok(())
    }
    fn push(&mut self, chr: u8, index: usize) -> Result<(), SimdevalError> {
        if let Some(ParseElement::Token(token)) = self.elements.last_mut() {
            match chr {
                b' ' => (),
                b':' => match token.kind() {
                    TokenKind::Identifier(_) => {
                        token.set_kind(TokenKind::Special(Special::Namespace));
                    }
                    //_ => self.new_token(chr, index)?,
                    _ => return Err(SimdevalError::UnexpectedToken),
                },
                b'0'..=b'9' => match token.kind() {
                    TokenKind::Identifier(_) | TokenKind::Literal(_) => token.inc_end(),
                    //_ => self.new_token(chr, index)?,
                    _ => self.new_token_from_kind(TokenKind::Literal(Literal::Int), index),
                },
                b'a'..=b'z' | b'A'..=b'Z' => match token.kind() {
                    TokenKind::Identifier(_) => token.inc_end(),
                    _ => {
                        self.new_token_from_kind(TokenKind::Identifier(Identifier::Variable), index)
                    } //_ => self.new_token(chr, index)?,
                },
                b'.' => match token.kind() {
                    TokenKind::Literal(Literal::Int) => {
                        token.set_inc(TokenKind::Literal(Literal::Float))
                    }
                    _ => self.new_token_from_kind(TokenKind::Literal(Literal::Float), index),
                    //_ => return Err(SimdevalError::UnexpectedToken)
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
                        //self.new_token(chr, index)?;
                        self.new_token_from_kind(TokenKind::Bracket(Bracket::Opened), index)
                    }
                    //_ => self.new_token(chr, index)?,
                    _ => self.new_token_from_kind(TokenKind::Bracket(Bracket::Opened), index),
                },
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
