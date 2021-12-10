use std::str::FromStr;

use crate::lex::tokens::Tokens;

use super::node::{Function, Node};

#[derive(Debug)]
pub(crate) struct Nodes<T>
where
    T: Function<T>,
{
    nodes: Vec<Node<T>>,
}

impl<T: Function<T>> FromIterator<Node<T>> for Nodes<T> {
    fn from_iter<S: IntoIterator<Item = Node<T>>>(iter: S) -> Self {
        Self {
            nodes: Vec::from_iter(iter),
        }
    }
}
impl<T: Function<T>> Nodes<T> {
    pub(crate) fn new(nodes: Vec<Node<T>>) -> Self {
        Self { nodes }
    }
    pub(crate) fn with_capacity(capacity: usize) -> Self {
        Self {
            nodes: Vec::with_capacity(capacity),
        }
    }
    pub(crate) fn push(&mut self, node: Node<T>) {
        self.nodes.push(node);
    }
}
