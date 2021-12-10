use std::{cmp::Ordering, str::FromStr};

use crate::lex::tokens::Tokens;

use super::node::{Function, Node};
use std::fmt::Debug;

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

impl<T> Nodes<T>
where
    T: Function<T> + Debug,
{
    pub(crate) fn set_indices(&mut self) {
        let mut weight = 0;
        let mut iter = self
            .nodes
            .iter_mut()
            .enumerate()
            .filter_map(|(i, n)| match n {
                <Node<T>>::Instruction {
                    operator: o,
                    lhs,
                    rhs,
                } => {
                    *lhs = i - 1;
                    *rhs = i + 1;
                    Some((i, n, weight))
                }
                <Node<T>>::Bracket(b) => {
                    weight += b.weight();
                    None
                }
                _ => None,
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
    }
}
impl<T> Nodes<T>
where
    T: Function<T>,
{
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
