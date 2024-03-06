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

#[derive(Debug, thiserror::Error)]
pub enum ParseTokensError {
    #[error("Operator had nothing as its operand")]
    EmptySideOfOperator,
    #[error("Unexpected multiple consecutive literals, maybe you are missing an operator?")]
    UnexpectedLiteralsGroup,
}

#[derive(thiserror::Error, Debug)]
pub enum ParseError {
    #[error(transparent)]
    TokenizingError(#[from] TokenizeError),
    #[error(transparent)]
    ParsingError(#[from] ParseTokensError),
}