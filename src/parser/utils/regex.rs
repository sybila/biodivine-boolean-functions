use crate::parser::structs::IntermediateToken;
use regex::{Regex, RegexSet};

lazy_static::lazy_static! {
    pub static ref SHOULD_END_LITERAL: Regex = Regex::new(r"[^-_a-zA-Z0-9]").unwrap();
    static ref LITERAL_IDENTIFIER: Regex = Regex::new(r"[-_a-zA-Z0-9]+").unwrap();

    pub static ref PATTERN_SET: RegexSet = RegexSet::new(IntermediateToken::ALL_TOKEN_PATTERNS_FROM_LONGEST
        .iter()
        .map(|pattern| {
            format!(
                r"(?i)^{}{}",
                // escape the pattern so that e.g. "^" is not treated as regex, but as a literal character for the And operation
                regex::escape(pattern),
                if LITERAL_IDENTIFIER.is_match(pattern) {
                    "([^-_a-zA-Z0-9]|$)"
                } else {
                    ""
                }
            )
        }))
        .unwrap();
}
