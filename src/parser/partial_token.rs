
pub(crate) enum PartialToken {
    Digit(u8),
    Letter(u8),
    Delimiter(u8),
    Operator(u8),
    Bracket(u8),
}
pub(crate) struct ParsePartialTokenError;
impl TryFrom<u8> for PartialToken {
    type Error = ParsePartialTokenError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            b'0'..=b'9' => PartialToken::Digit(value),
            b'a'..=b'z' | b'A'..=b'Z' => PartialToken::Letter(value),
            b'.' | b',' | b'_' | b' ' => PartialToken::Delimiter(value),
            b'+' | b'-' | b'*' | b'/' | b'%' | b'^' => PartialToken::Operator(value),
            b'&' | b'|' | b'!' | b'=' | b'<' | b'>' => PartialToken::Operator(value),
            b'{' | b'}' | b'(' | b')' | b'[' | b']' => PartialToken::Bracket(value),
            _ => return Err(ParsePartialTokenError)
        })
    }
}

