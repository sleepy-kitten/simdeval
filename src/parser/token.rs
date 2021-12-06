/// a `Token` type
/// stores the token kind and the lenght of that token in the source code

#[derive(Debug)]
pub(crate) struct Token {
    kind: TokenKind,
    span: u16,
}

impl Token {
    /// Get the token's span.
    pub(crate) fn span(&self) -> u16 {
        self.span
    }
    /// constructs a new token
    pub(crate) fn new(kind: TokenKind) -> Self {
        Self { kind, span: 1 }
    }

    /// Get a reference to the token's kind.
    pub(crate) fn kind(&self) -> &TokenKind {
        &self.kind
    }
    /// increments the span by 1
    pub(crate) fn inc_span(&mut self) {
        self.span += 1
    }

    /// set the kind and increments the span
    pub(crate) fn push(&mut self, kind: TokenKind) {
        self.kind = kind;
        self.inc_span()
    }
}

#[derive(Debug)]
pub(crate) enum TokenKind {
    Literal(Literal),
    Operator(Operator),
    Identifier(Identifier),
    Separator(Separator),
}

#[derive(Debug)]
pub(crate) enum Literal {
    String,
    Int,
    Float,
}

impl Literal {
}
#[derive(Debug)]
pub(crate) enum Separator {
    Comma,
    WavyBracket(Bracket),
    Bracket(Bracket),
    SquareBracket(Bracket),
}
#[derive(Debug)]
pub(crate) enum Bracket {
    Opened,
    Closed,
}

#[derive(Debug)]
pub(crate) enum Identifier {
    Function,
    Variable,
}
#[derive(Debug)]
pub(crate) enum Operator {
    Arithmetic(Arithmetic),
    Logical(Logical),
}

#[derive(Debug)]
pub(crate) enum Arithmetic {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
}
#[derive(Debug)]
pub(crate) enum Logical {
    And,
    Or,
    Xor,
    Not,
    Equal,
    NotEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
}
