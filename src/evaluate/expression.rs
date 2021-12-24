use crate::{error::Error, stack::Stack};

use super::{
    enums::{Bracket, Identifier, Literal, Operator, Special, TokenKind},
    function::Function,
    node::Node,
    parse_element::ParseElement,
    token::Token,
    value::{single::Single, Value},
    variables::Variables,
};
use core::fmt;
use std::{
    cmp::Ordering,
    collections::HashMap,
    fmt::Debug,
    simd::{LaneCount, SupportedLaneCount},
};

struct IndexedWeight {
    weight: i16,
    index: usize,
}

/// An `Expression` contains the expression string and the compiled version of that expression.
///
/// # UB
/// Compiling the same expression multiple times is UB
#[derive(Debug)]
pub struct Expression<T, const LANES: usize>
where
    T: Function<T, LANES>,
    LaneCount<LANES>: SupportedLaneCount,
    [(); T::MAX_ARGS]:,
{
    elements: Vec<ParseElement<T, LANES>>,
    variables: Variables<LANES>,
    expression: String,
    top_node: Option<usize>,
}
impl<T, const LANES: usize> fmt::Display for Expression<T, LANES>
where
    T: Function<T, LANES>,
    T: Clone + Debug,
    LaneCount<LANES>: SupportedLaneCount,
    [(); T::MAX_ARGS]:,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.expression)
    }
}
impl<'b, T, const LANES: usize> Expression<T, LANES>
where
    T: Function<T, LANES>,
    T: Clone + Debug,
    LaneCount<LANES>: SupportedLaneCount,
    [(); T::MAX_ARGS]:,
{
    pub(crate) fn to_tokens(&mut self) -> Result<&mut Self, Error> {
        // WHY DO I HAVE TO CLONE NOW!?
        let cloned = self.expression.clone();
        for (index, &chr) in cloned.as_bytes().iter().enumerate() {
            self.push(chr, index)?;
        }
        Ok(self)
    }
    pub(crate) fn to_nodes<const N: usize>(&mut self) -> Result<&mut Self, Error> {
        {
            let mut namespaces = Stack::<&str, N>::new();
            for element in &mut self.elements {
                if let ParseElement::Token(token) = element {
                    match token.kind() {
                        TokenKind::Literal(l) => {
                            let value = Value::Single(match l {
                                Literal::Bool => {
                                    Single::Bool(token.slice(&self.expression).parse::<bool>()?)
                                }
                                Literal::Float => {
                                    Single::Float(token.slice(&self.expression).parse::<f64>()?)
                                }
                                Literal::Int => {
                                    Single::Int(token.slice(&self.expression).parse::<i64>()?)
                                }
                            });
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
                            let identifier = token.slice(&self.expression);
                            let mut namespaces = namespaces.iter();
                            let function = <T as Function<T, LANES>>::from_string(
                                &mut namespaces,
                                identifier,
                            )?;
                            *element = ParseElement::Node(Node::Function {
                                function,
                                args: Stack::new(),
                            })
                        }
                        TokenKind::Bracket(_) | TokenKind::Special(Special::Comma) => (),
                        TokenKind::Special(Special::Namespace) => {
                            namespaces.push(token.slice(&self.expression));
                        }
                        TokenKind::Special(Special::NegZero) => {
                            *element =
                                ParseElement::Node(Node::Literal(Value::Single(Single::Int(0))));
                        }
                        TokenKind::Identifier(Identifier::Variable) => {
                            let identifier = token.slice(&self.expression);
                            let index = self.variables.find_or_set(identifier);
                            *element = ParseElement::Node(Node::Variable { index });
                        }
                    };
                }
            }
        }
        Ok(self)
    }
    pub(crate) fn set_indices(&'b mut self) -> Result<&mut Self, Error> {
        struct NodeInfo<'b, T, const LANES: usize>
        where
            T: Function<T, LANES>,
            LaneCount<LANES>: SupportedLaneCount,
            [(); T::MAX_ARGS]:,
        {
            index: usize,
            node: &'b mut Node<T, LANES>,
            weight: i16,
        }
        let mut bracket_weight = 0;
        let mut lowest_weight = IndexedWeight {
            weight: i16::MAX,
            index: 0,
        };
        let mut iter = self
            .elements
            .iter_mut()
            .enumerate()
            .filter_map(|(index, e)| match e {
                ParseElement::<T, LANES>::Node(node) => match node {
                    <Node<T, LANES>>::Instruction {
                        operator: o,
                        lhs,
                        rhs,
                    } => {
                        *lhs = index - 1;
                        *rhs = index + 1;
                        let weight = o.weight();
                        let info = NodeInfo::<'b, T, LANES> {
                            index,
                            node,
                            weight: weight + bracket_weight,
                        };
                        Some(info)
                    }
                    <Node<T, LANES>>::Function { function, args } => {
                        let info = NodeInfo::<'b, T, LANES> {
                            index,
                            node,
                            weight: bracket_weight,
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
            match (c, n) {
                (Some(curr), Some(next)) => {
                    let ord = curr.weight.cmp(&next.weight);

                    match ord {
                        Ordering::Equal => {
                            if let Node::Instruction { lhs, .. } = next.node {
                                *lhs = curr.index;
                            } else {
                            }
                            if lowest_weight.weight >= next.weight {
                                lowest_weight.weight = next.weight;
                                lowest_weight.index = next.index;
                            }
                        }
                        Ordering::Greater => {
                            if let Node::Instruction { lhs, .. } = next.node {
                                *lhs = curr.index;
                            } else {
                            }
                            if lowest_weight.weight >= next.weight {
                                lowest_weight.weight = next.weight;
                                lowest_weight.index = next.index;
                            }
                        }
                        Ordering::Less => {
                            if let Node::Instruction { rhs, .. } = curr.node {
                                *rhs = next.index;
                            } else {
                            }
                            if lowest_weight.weight >= curr.weight {
                                lowest_weight.weight = curr.weight;
                                lowest_weight.index = curr.index;
                            }
                        }
                    }
                }
                (Some(curr), None) => {
                    if lowest_weight.weight >= curr.weight {
                        lowest_weight.weight = curr.weight;
                        lowest_weight.index = curr.index
                    }
                }
                _ => break,
            }
        }
        self.top_node = Some(lowest_weight.index);
        Ok(self)
    }
    pub(crate) fn set_function_args(&mut self) -> Result<(), Error> {
        let mut index = 0;
        while index < self.elements.len() {
            let element = &self.elements[index];
            // skip until element is a function
            if let ParseElement::Node(Node::Function { .. }) = element {
                let mut lowest_weight = IndexedWeight {
                    weight: i16::MAX,
                    index: 0,
                };
                let function_index = index;
                while index < self.elements.len() {
                    let element = &self.elements[index];
                    match element {
                        ParseElement::Node(
                            Node::Literal(_) | Node::Variable { .. } | Node::Function { .. },
                        ) if lowest_weight.weight == i16::MAX
                            && match &self.elements[index + 1] {
                                ParseElement::Token(token) => match token.kind() {
                                    TokenKind::Bracket(Bracket::Closed) => true,
                                    TokenKind::Special(Special::Comma) => true,
                                    _ => false,
                                },
                                _ => false,
                            } =>
                        {
                            if let ParseElement::Node(Node::Function { args, .. }) =
                                &mut self.elements[function_index]
                            {
                                lowest_weight.index = index;
                            }
                        }
                        // if element is instruction update weight and index
                        ParseElement::Node(Node::Instruction { operator, .. }) => {
                            if lowest_weight.weight >= operator.weight() {
                                lowest_weight.weight = operator.weight();
                                lowest_weight.index = index;
                            }
                        }
                        ParseElement::Token(token) => match token.kind() {
                            // if element is comma add last lowest index to args and reset weight
                            TokenKind::Special(Special::Comma) => {
                                if let ParseElement::Node(Node::Function { args, .. }) =
                                    &mut self.elements[function_index]
                                {
                                    args.push(lowest_weight.index);
                                    lowest_weight.weight = i16::MAX;
                                }
                            }
                            // if element is bracket add last lowest index to args and break
                            TokenKind::Bracket(Bracket::Closed) => {
                                if let ParseElement::Node(Node::Function { args, .. }) =
                                    &mut self.elements[function_index]
                                {
                                    args.push(lowest_weight.index);
                                    break;
                                }
                            }
                            _ => {}
                        },
                        _ => (),
                    }
                    index += 1;
                }
            }
            index += 1;
        }
        Ok(())
    }
    pub fn to_simd(&mut self) {
        for element in self.elements.iter_mut() {
            if let ParseElement::Node(node) = element {
                match node {
                    Node::Literal(v) => {
                        v.to_simd();
                    }
                    Node::Variable { index } => {
                        self.variables[*index].to_simd();
                    }
                    _ => (),
                }
            }
        }
    }
    pub fn string(&self) -> &str {
        &self.expression
    }
    pub fn variables_iter(&self) -> impl Iterator<Item = &str> {
        self.variables.identifiers()
    }
}
impl<'a, T: Function<T, LANES>, const LANES: usize> Expression<T, LANES>
where
    T: Function<T, LANES> + Clone + Debug,
    [(); T::MAX_ARGS]:,
    LaneCount<LANES>: SupportedLaneCount,
{
    /// creates a new `Expression` from a string
    pub fn new(expression: String) -> Self {
        Self {
            elements: Vec::with_capacity(expression.len()),
            variables: Variables::with_capacity(expression.len() / 2),
            expression,
            top_node: None,
        }
    }
    /// Get a reference to the expression's elements.
    pub(crate) fn elements(&self) -> &[ParseElement<T, LANES>] {
        self.elements.as_ref()
    }
    fn clear(&mut self) {
        self.elements.clear();
        self.variables.clear();
        self.top_node = None
    }
    /// set a new expression
    pub fn set_expression(&mut self, expression: String) {
        self.clear();
        self.expression = expression;
    }
    /// compiles the expression into a usable form
    #[inline]
    pub fn compile(&mut self) -> Result<(), Error> {
        if self.top_node.is_none() {
            if self.expression.is_empty() {
                Err(Error::EmptyExpression)
            } else {
                self.to_tokens()?
                    .to_nodes::<4>()?
                    .set_indices()?
                    .set_function_args()
            }
        } else {
            Err(Error::AlreadyCompiled)
        }
    }

    pub fn optimize(&mut self) -> Result<(), Error> {
        if let Some(top_node) = self.top_node {
            self.optimize_recursive(top_node)?;
            Ok(())
        } else {
            Err(Error::NotCompiled)
        }
    }
    fn optimize_recursive(&mut self, index: usize) -> Result<Option<Value<LANES>>, Error> {
        if let ParseElement::Node(node) = &self.elements[index] {
            match node {
                Node::Instruction { operator, lhs, rhs } => {
                    let operator = *operator;
                    let rhs = *rhs;
                    let lhs = *lhs;
                    if let (Ok(Some(lhs)), Ok(Some(rhs))) =
                        (self.optimize_recursive(lhs), self.optimize_recursive(rhs))
                    {
                        let value = operator.eval(lhs, rhs);
                        self.elements[index] = ParseElement::Node(Node::Literal(value));
                        Ok(Some(value))
                    } else {
                        Ok(None)
                    }
                }
                Node::Literal(value) => Ok(Some(*value)),
                Node::Function { function, args } => {
                    let function = function.clone();
                    let args = args.clone();
                    let mut args_eval = Stack::<Value<LANES>, { T::MAX_ARGS }>::new();
                    for arg in args.iter() {
                        let arg_eval = self.optimize_recursive(*arg)?;
                        if let Some(arg) = None {
                            args_eval.push(arg)
                        } else {
                            return Ok(None);
                        }
                    }
                    if function.is_const() {
                        let value = function.call(args_eval.slice())?;
                        self.elements[index] = ParseElement::Node(Node::Literal(value));
                        Ok(Some(value))
                    } else {
                        Ok(None)
                    }
                }
                Node::Variable { .. } => Ok(None),
            }
        } else {
            Ok(None)
        }
    }
    fn new_token_from_kind(&mut self, token_kind: TokenKind, start: usize) {
        let token = Token::new(token_kind, start);
        self.elements.push(ParseElement::Token(token));
    }

    pub fn set_variable(&mut self, identifier: &'a str, value: Value<LANES>) -> Result<(), Error> {
        self.variables.set(identifier, value)
    }

    pub fn set_variable_by_index(
        &mut self,
        index: usize,
        value: Value<LANES>,
    ) -> Result<(), Error> {
        self.variables.set_by_index(index, value)
    }
    fn new_token(&mut self, chr: u8, start: usize) -> Result<(), Error> {
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

            _ => return Err(Error::UnkownCharacter(chr as char)),
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
    fn push(&mut self, chr: u8, index: usize) -> Result<(), Error> {
        if let Some(ParseElement::Token(token)) = self.elements.last_mut() {
            match chr {
                b' ' => (),
                b':' => match token.kind() {
                    TokenKind::Identifier(_) => {
                        token.set_kind(TokenKind::Special(Special::Namespace));
                    }
                    _ => return Err(Error::UnexpectedToken),
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
                    (b'(', _) => {
                        self.new_token_from_kind(TokenKind::Bracket(Bracket::Opened), index)
                    }
                    (b')', _) => {
                        self.new_token_from_kind(TokenKind::Bracket(Bracket::Closed), index)
                    }
                    _ => unreachable!(),
                },
                b',' => self.new_token_from_kind(TokenKind::Special(Special::Comma), index),
                _ => return Err(Error::UnexpectedToken),
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

impl<'a, T: Function<T, LANES>, const LANES: usize> Expression<T, LANES>
where
    T: Function<T, LANES> + Clone + Debug,
    [(); T::MAX_ARGS]:,
    LaneCount<LANES>: SupportedLaneCount,
{
    #[inline]
    pub fn eval(&self) -> Result<Value<LANES>, Error> {
        if let Some(top_node) = self.top_node {
            self.eval_recursive(top_node)
        } else {
            Err(Error::NotCompiled)
        }
    }
    fn eval_recursive(&self, index: usize) -> Result<Value<LANES>, Error> {
        if let ParseElement::Node(n) = &self.elements[index] {
            Ok(match n {
                Node::Instruction { operator, lhs, rhs } => {
                    operator.eval(self.eval_recursive(*lhs)?, self.eval_recursive(*rhs)?)
                }
                Node::Literal(value) => *value,
                Node::Variable { index } => self.variables[*index],
                Node::Function { function, args } => {
                    let mut args_eval = Stack::<Value<LANES>, { T::MAX_ARGS }>::new();
                    for arg in args.iter() {
                        args_eval.push(self.eval_recursive(*arg)?);
                    }
                    function.call(args_eval.slice())?
                }
            })
        } else {
            Err(Error::InvalidIndex)
        }
    }
    fn get_operands(&self, index: usize) -> Option<(usize, usize)> {
        self.elements[index].get_operands_indices()
    }

}

impl<'a, T, const LANES: usize> Default for Expression<T, LANES>
where
    LaneCount<LANES>: SupportedLaneCount,
    T: Function<T, LANES>,
    [(); T::MAX_ARGS]:,
{
    fn default() -> Self {
        Self {
            elements: Default::default(),
            variables: Default::default(),
            expression: Default::default(),
            top_node: Default::default(),
        }
    }
}
