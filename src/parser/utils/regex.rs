use regex::Regex;

lazy_static::lazy_static! {
    pub static ref SHOULD_END_LITERAL: Regex = Regex::new(r"[^-_a-zA-Z0-9]").unwrap();
    pub static ref LITERAL_IDENTIFIER: Regex = Regex::new(r"[-_a-zA-Z0-9]+").unwrap();
}
