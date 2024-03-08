use crate::parser::ParseError;

pub trait Parse {
    fn from_str(input: &str) -> Result<Self, ParseError>
    where
        Self: Sized;

    fn from_string(input: String) -> Result<Self, ParseError>
    where
        Self: Sized,
    {
        Self::from_str(input.as_str())
    }
}
