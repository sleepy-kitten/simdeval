use super::{partial_token_stream::PartialTokenStream, token::Token};

pub(crate) struct TokenStream {
    stream: Vec<Token>,
}

impl TryFrom<PartialTokenStream> for TokenStream {
    fn try_from(value: PartialTokenStream) -> Result<Self, Self::Error> {

    }
}