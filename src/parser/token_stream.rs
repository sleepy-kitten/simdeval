use std::str::FromStr;

use crate::error::SimdevalError;

use super::token::{
    Arithmetic, Bracket, Identifier, Literal, Logical, Operator, Separator, Token, TokenKind,
};
#[derive(Debug, PartialEq, Eq)]
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

impl FromStr for TokenStream {
    type Err = SimdevalError;
    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let mut token_stream = TokenStream::with_capacity(string.len());
        for &chr in string.as_bytes() {
            token_stream.push(chr)?
        }
        Ok(token_stream)
    }
}

impl TokenStream {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            stream: Vec::with_capacity(capacity),
        }
    }
    pub fn push(&mut self, chr: u8) -> Result<(), SimdevalError> {
        let token = self.stream.last_mut();
        if let Some(token) = token {
            match chr {
                b'0'..=b'9' => match token.kind() {
                    TokenKind::Identifier(_) => token.inc_span(),
                    TokenKind::Literal(_) => token.inc_span(),
                    _ => self.new_token(chr)?,
                },

                b'a'..=b'z' | b'A'..=b'Z' => match token.kind() {
                    TokenKind::Identifier(_) => token.inc_span(),
                    TokenKind::Literal(Literal::String) => token.inc_span(),
                    _ => self.new_token(chr)?,
                },

                b'.' | b',' | b'_' | b' ' | b'"' => match (chr, token.kind()) {
                    (_, TokenKind::Literal(Literal::String)) => token.inc_span(),
                    (b'.', TokenKind::Literal(Literal::Int)) => {
                        token.push(TokenKind::Literal(Literal::Float))
                    }
                    (b' ', _) => (),
                    _ => self.new_token(chr)?,
                },

                b'+' | b'-' | b'*' | b'/' | b'%' | b'^' | b'&' | b'|' | b'!' | b'=' | b'<'
                | b'>' | b'#' => match (chr, token.kind()) {
                    (b'>', TokenKind::Operator(Operator::Logical(Logical::Equal))) => token.push(
                        TokenKind::Operator(Operator::Logical(Logical::GreaterEqual)),
                    ),
                    (b'<', TokenKind::Operator(Operator::Logical(Logical::Equal))) => {
                        token.push(TokenKind::Operator(Operator::Logical(Logical::LessEqual)))
                    }
                    (b'!', TokenKind::Operator(Operator::Logical(Logical::Equal))) => {
                        token.push(TokenKind::Operator(Operator::Logical(Logical::NotEqual)))
                    }
                    _ => self.new_token(chr)?,
                },
                _ => (),
            }
        } else {
            self.new_token(chr)?;
        }
        Ok(())
    }
    fn new_token(&mut self, chr: u8) -> Result<(), SimdevalError> {
        let token = Token::new(match chr {
            b'0'..=b'9' => TokenKind::Literal(Literal::Int),

            b'a'..=b'z' | b'A'..=b'Z' => TokenKind::Identifier(Identifier::Variable),

            b'+' => TokenKind::Operator(Operator::Arithmetic(Arithmetic::Add)),
            b'-' => TokenKind::Operator(Operator::Arithmetic(Arithmetic::Sub)),
            b'*' => TokenKind::Operator(Operator::Arithmetic(Arithmetic::Mul)),
            b'/' => TokenKind::Operator(Operator::Arithmetic(Arithmetic::Div)),
            b'%' => TokenKind::Operator(Operator::Arithmetic(Arithmetic::Mod)),
            b'^' => TokenKind::Operator(Operator::Arithmetic(Arithmetic::Pow)),

            b'=' => TokenKind::Operator(Operator::Logical(Logical::Equal)),
            b'!' => TokenKind::Operator(Operator::Logical(Logical::Not)),
            b'>' => TokenKind::Operator(Operator::Logical(Logical::Greater)),
            b'<' => TokenKind::Operator(Operator::Logical(Logical::Less)),
            b'&' => TokenKind::Operator(Operator::Logical(Logical::And)),
            b'|' => TokenKind::Operator(Operator::Logical(Logical::Or)),
            b'#' => TokenKind::Operator(Operator::Logical(Logical::Xor)),

            b'(' => TokenKind::Separator(Separator::Bracket(Bracket::Opened)),
            b')' => TokenKind::Separator(Separator::Bracket(Bracket::Closed)),
            b'{' => TokenKind::Separator(Separator::WavyBracket(Bracket::Closed)),
            b'}' => TokenKind::Separator(Separator::WavyBracket(Bracket::Closed)),
            b'[' => TokenKind::Separator(Separator::SquareBracket(Bracket::Closed)),
            b']' => TokenKind::Separator(Separator::SquareBracket(Bracket::Closed)),

            b'.' => TokenKind::Literal(Literal::Float),
            b',' => TokenKind::Separator(Separator::Comma),

            _ => return Err(SimdevalError::UnkownCharacter),
        });
        self.stream.push(token);
        Ok(())
    }
}

/*
fn try_from(chr: u8) -> Result<Self, Self::Error> {
        Ok(match chr {
            b'0'..=b'9' => PartialToken::Digit(chr),
            b'a'..=b'z' | b'A'..=b'Z' => PartialToken::Letter(chr),
            b'.' | b',' | b'_' | b' ' | b'"' => PartialToken::Delimiter(chr),
            b'{' | b'}' | b'(' | b')' | b'[' | b']' => PartialToken::Bracket(chr),
            b'+' | b'-' | b'*' | b'/' | b'%' | b'^' => PartialToken::Operator(chr),
            b'&' | b'|' | b'!' | b'=' | b'<' | b'>' | b'#' => PartialToken::Operator(chr),
            _ => return Err(SimdevalError::UnkownCharacter)
        })
    }
*/
