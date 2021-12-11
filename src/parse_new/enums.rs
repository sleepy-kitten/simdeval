#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum TokenKind {
    Literal(Literal),
    Bracket(Bracket),
    Identifier(Identifier),
    Operator(Operator),
    Special(Special),
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Literal {
    Int,
    Float,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Bracket {
    Opened,
    Closed,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Identifier {
    Function,
    Variable,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
    Not,

    Equal,
    And,
    Or,
    Xor,
    NotEqual,
    Greater,
    GreaterEqual,
    Smaller,
    SmallerEqual,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Special {
    Namespace,
    NegZero,
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum Value {
    Int(i64),
    Float(f64),
    Bool(bool),
}

impl Bracket {
    pub fn weight(&self) -> i16 {
        match self {
            Self::Opened => 100,
            Self::Closed => -100,
        }
    }
}
impl Operator {
    pub fn weight(&self) -> i16 {
        match self {
            Self::Add => 20,
            Self::Sub => 20,
            Self::Mul => 21,
            Self::Div => 21,
            Self::Mod => 22,
            Self::Pow => 22,

            Self::Not => 1,
            Self::Equal => 2,
            Self::And => 3,
            Self::Or => 4,
            Self::Xor => 5,
            Self::NotEqual => 7,
            Self::Greater => 8,
            Self::GreaterEqual => 9,
            Self::Smaller => 10,
            Self::SmallerEqual => 11,
        }
    }
}
