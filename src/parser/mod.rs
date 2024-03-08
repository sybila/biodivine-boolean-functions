use crate::parser::structs::PositionTracker;
pub use error::ParseError;
use itertools::MultiPeek;
pub use parse::parse_tokens;
use std::str::Chars;
pub use tokenize::tokenize;

mod error;
mod parse;
mod structs;
mod tokenize;
mod utils;

type TokenizerInput<'a> = PositionTracker<MultiPeek<Chars<'a>>>;
