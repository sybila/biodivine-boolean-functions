use crate::expressions::Expression;
use crate::parser::{parse_tokens, tokenize, ParseError};
use crate::traits::Parse;

impl Parse for Expression<String> {
    fn from_str(input: &str) -> Result<Self, ParseError> {
        let tokens = tokenize(input)?;
        let parsed = parse_tokens(&tokens)?;

        Ok(parsed)
    }
}
