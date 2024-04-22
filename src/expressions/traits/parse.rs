use crate::expressions::Expression;
use crate::parser::{parse_tokens, tokenize, ParseError};
use std::str::FromStr;

impl FromStr for Expression<String> {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let tokens = tokenize(input)?;
        let parsed = parse_tokens(&tokens)?;

        Ok(parsed)
    }
}
