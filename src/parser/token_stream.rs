
use super::{
    partial_token::PartialToken,
    partial_token_stream::PartialTokenStream,
    token::{
        Arithmetic, Bracket, Identifier, Literal, Logical, Operator, Separator, Token, TokenKind,
    },
};

#[derive(Debug)]
pub(crate) struct TokenStream {
    stream: Vec<Token>,
}
impl FromIterator<Token> for TokenStream {
    fn from_iter<T: IntoIterator<Item = Token>>(iter: T) -> Self {
        Self {
            stream: FromIterator::from_iter(iter),
        }
    }
}
impl From<PartialTokenStream> for TokenStream {
    fn from(partial_token_stream: PartialTokenStream) -> Self {
        let mut token_stream =
            TokenStream::with_capacity(partial_token_stream.size_hint().1.unwrap_or(1));
        for partial_token in partial_token_stream {
            token_stream.push(partial_token);
        }
        token_stream
    }
}

impl TokenStream {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            stream: Vec::with_capacity(capacity),
        }
    }
    pub fn push(&mut self, partial_token: PartialToken) {
        let token = self.stream.last_mut();
        if let Some(token) = token {
            match partial_token {
                PartialToken::Digit(_) => match token.kind() {
                    TokenKind::Identifier(_) => token.inc_span(),
                    TokenKind::Literal(_) => token.inc_span(),
                    _ => self.new_token(partial_token),
                },
                PartialToken::Letter(_) => match token.kind() {
                    TokenKind::Identifier(_) => token.inc_span(),
                    TokenKind::Literal(Literal::String) => token.inc_span(),
                    _ => self.new_token(partial_token),
                },
                PartialToken::Delimiter(d) => match (d, token.kind()) {
                    (_, TokenKind::Literal(Literal::String)) => token.inc_span(),
                    (b'.', TokenKind::Literal(Literal::Int)) => {
                        token.push(TokenKind::Literal(Literal::Float))
                    }
                    (b' ', _) => (),
                    _ => self.new_token(partial_token),
                },
                PartialToken::Operator(o) => match (o, token.kind()) {
                    (b'>', TokenKind::Operator(Operator::Logical(Logical::Equal))) => token
                        .push(TokenKind::Operator(Operator::Logical(
                            Logical::GreaterEqual,
                        ))),
                    (b'<', TokenKind::Operator(Operator::Logical(Logical::Equal))) => token
                        .push(TokenKind::Operator(Operator::Logical(
                            Logical::LessEqual,
                        ))),
                    (b'!', TokenKind::Operator(Operator::Logical(Logical::Equal))) => token
                        .push(TokenKind::Operator(Operator::Logical(
                            Logical::NotEqual,
                        ))),
                    _ => self.new_token(partial_token),
                },
                PartialToken::Bracket(_) => (),
            }
        } else {
            self.new_token(partial_token);
        }
    }
    fn new_token(&mut self, partial_token: PartialToken) {
        let token = match partial_token {
            PartialToken::Digit(_) => Token::new(TokenKind::Literal(Literal::Int)),
            PartialToken::Letter(_) => Token::new(TokenKind::Identifier(Identifier::Variable)),
            PartialToken::Operator(o) => Token::new(TokenKind::Operator(match o {
                b'+' => Operator::Arithmetic(Arithmetic::Add),
                b'-' => Operator::Arithmetic(Arithmetic::Sub),
                b'*' => Operator::Arithmetic(Arithmetic::Mul),
                b'/' => Operator::Arithmetic(Arithmetic::Div),
                b'%' => Operator::Arithmetic(Arithmetic::Mod),
                b'^' => Operator::Arithmetic(Arithmetic::Pow),
                b'=' => Operator::Logical(Logical::Equal),
                b'!' => Operator::Logical(Logical::Not),
                b'>' => Operator::Logical(Logical::Greater),
                b'<' => Operator::Logical(Logical::Less),
                b'&' => Operator::Logical(Logical::And),
                b'|' => Operator::Logical(Logical::Or),
                b'#' => Operator::Logical(Logical::Xor),
                _ => unreachable!(),
            })),
            PartialToken::Bracket(b) => Token::new(TokenKind::Separator(match b {
                b'(' => Separator::Bracket(Bracket::Opened),
                b')' => Separator::Bracket(Bracket::Closed),
                b'{' => Separator::WavyBracket(Bracket::Closed),
                b'}' => Separator::WavyBracket(Bracket::Closed),
                b'[' => Separator::SquareBracket(Bracket::Closed),
                b']' => Separator::SquareBracket(Bracket::Closed),
                _ => unreachable!(),
            })),
            PartialToken::Delimiter(d) => Token::new(match d {
                b'.' => TokenKind::Literal(Literal::Float),
                b',' => TokenKind::Separator(Separator::Comma),
                _ => unreachable!(),
            }),
        };
        self.stream.push(token);
    }
}
