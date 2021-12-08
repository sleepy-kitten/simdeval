#[derive(Debug)]
pub enum SimdevalError {
    UnkownCharacter(char),
    UnexpectedToken,
    NoIdentifierMatch,
}
