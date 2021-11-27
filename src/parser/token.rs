pub(crate) enum Token {
    Literal(Literal),
    Operator(Operator),
    Identifier(Identifier),
    Separator(Separator),
}

pub(crate) enum Literal {
    String(String),
    Int(i64),
    Float(f64),
}
pub(crate) enum Separator {
    WavyBracket(Bracket),
    Bracket(Bracket),
    SquareBracket(Bracket),
}
pub(crate) enum Bracket {
    Opened,
    Closed,
}

pub(crate) enum Identifier {
    Function(String),
    Variable(String),
}
pub(crate) enum Operator {
    Arithmetic(Arithmetic),
    Logical(Logical),
}

pub(crate) enum Arithmetic {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
}

pub(crate) enum Logical {
    And,
    Or,
    Xor,
    Not,
    Equal,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
}
