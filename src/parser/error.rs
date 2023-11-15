#[derive(Debug, thiserror::Error)]
pub enum TokenParseError {
    #[error("Unknown symbol encountered {0}")]
    UnknownSymbol(String),
}

#[derive(Debug, thiserror::Error)]
pub enum TokenizeError {
    #[error("Unexpected `)` encountered")]
    UnexpectedClosingParenthesis,
    #[error("Missing `)`")]
    MissingClosingParenthesis,
    #[error("Unexpected `}}` encountered")]
    UnexpectedClosingCurlyBrace,
    #[error("Unexpected whitespace encountered in the middle of operator")]
    UnexpectedWhitespace,
}
