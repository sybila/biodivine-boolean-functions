use crate::parser::utils::PATTERN_SET;

#[derive(PartialEq, Debug)]
pub enum IntermediateToken<'a> {
    And { pattern: &'a str },
    Or { pattern: &'a str },
    Not { pattern: &'a str },
    ConstantTrue { pattern: &'a str },
    ConstantFalse { pattern: &'a str },
    ParenthesesStart,
    ParenthesesEnd,
    LiteralLongNameStart,
    LiteralLongNameEnd,
}

#[allow(dead_code)] // false positive on the const arrays
impl<'a> IntermediateToken<'a> {
    const AND_PATTERN_BIT: &'static str = "&";
    const AND_PATTERN_LOGIC: &'static str = "&&";
    const AND_PATTERN_WORD: &'static str = "and";
    const AND_PATTERN_MATH: &'static str = "∧";
    const AND_PATTERN_MATH_2: &'static str = "^";
    const AND_PATTERN_BOOL: &'static str = "*";

    const AND_PATTERNS: [&'static str; 6] = [
        Self::AND_PATTERN_BIT,
        Self::AND_PATTERN_LOGIC,
        Self::AND_PATTERN_WORD,
        Self::AND_PATTERN_MATH,
        Self::AND_PATTERN_MATH_2,
        Self::AND_PATTERN_BOOL,
    ];

    const OR_PATTERN_BIT: &'static str = "|";
    const OR_PATTERN_LOGIC: &'static str = "||";
    const OR_PATTERN_WORD: &'static str = "or";
    const OR_PATTERN_MATH: &'static str = "∨";
    const OR_PATTERN_MATH_2: &'static str = "v";
    const OR_PATTERN_BOOL: &'static str = "+";
    const OR_PATTERNS: [&'static str; 6] = [
        Self::OR_PATTERN_BIT,
        Self::OR_PATTERN_LOGIC,
        Self::OR_PATTERN_WORD,
        Self::OR_PATTERN_MATH,
        Self::OR_PATTERN_MATH_2,
        Self::OR_PATTERN_BOOL,
    ];

    const NOT_PATTERN_TILDE: &'static str = "~";
    const NOT_PATTERN_MARK: &'static str = "!";
    const NOT_PATTERN_WORD: &'static str = "not";
    const NOT_PATTERN_MATH: &'static str = "¬";
    const NOT_PATTERNS: [&'static str; 4] = [
        Self::NOT_PATTERN_TILDE,
        Self::NOT_PATTERN_MARK,
        Self::NOT_PATTERN_WORD,
        Self::NOT_PATTERN_MATH,
    ];

    const TRUE_PATTERN_CHAR: &'static str = "t";
    const TRUE_PATTERN_WORD: &'static str = "true";
    const TRUE_PATTERN_NUM: &'static str = "1";

    const TRUE_PATTERNS: [&'static str; 3] = [
        Self::TRUE_PATTERN_CHAR,
        Self::TRUE_PATTERN_WORD,
        Self::TRUE_PATTERN_NUM,
    ];

    const FALSE_PATTERN_CHAR: &'static str = "f";
    const FALSE_PATTERN_WORD: &'static str = "false";
    const FALSE_PATTERN_NUM: &'static str = "0";

    const FALSE_PATTERNS: [&'static str; 3] = [
        Self::FALSE_PATTERN_CHAR,
        Self::FALSE_PATTERN_WORD,
        Self::FALSE_PATTERN_NUM,
    ];

    pub const LITERAL_START_PATTERN: &'static str = "{";
    pub const LITERAL_END_PATTERN: &'static str = "}";
    const PARENTHESIS_START_PATTERN: &'static str = "(";
    const PARENTHESIS_END_PATTERN: &'static str = ")";

    pub fn all_token_patterns() -> Vec<&'a str> {
        [
            Self::AND_PATTERNS.as_slice(),
            Self::OR_PATTERNS.as_slice(),
            Self::NOT_PATTERNS.as_slice(),
            Self::FALSE_PATTERNS.as_slice(),
            Self::TRUE_PATTERNS.as_slice(),
            [
                Self::LITERAL_START_PATTERN,
                Self::LITERAL_END_PATTERN,
                Self::PARENTHESIS_START_PATTERN,
                Self::PARENTHESIS_END_PATTERN,
            ]
            .as_slice(),
        ]
        .concat()
    }

    pub const ALL_TOKEN_PATTERNS_FROM_LONGEST: [&'static str; 26] = [
        Self::FALSE_PATTERN_WORD,
        Self::TRUE_PATTERN_WORD,
        Self::AND_PATTERN_WORD,
        Self::NOT_PATTERN_WORD,
        Self::AND_PATTERN_LOGIC,
        Self::OR_PATTERN_LOGIC,
        Self::OR_PATTERN_WORD,
        Self::AND_PATTERN_BIT,
        Self::AND_PATTERN_MATH,
        Self::AND_PATTERN_MATH_2,
        Self::AND_PATTERN_BOOL,
        Self::OR_PATTERN_BIT,
        Self::OR_PATTERN_MATH,
        Self::OR_PATTERN_MATH_2,
        Self::OR_PATTERN_BOOL,
        Self::NOT_PATTERN_TILDE,
        Self::NOT_PATTERN_MARK,
        Self::NOT_PATTERN_MATH,
        Self::FALSE_PATTERN_CHAR,
        Self::FALSE_PATTERN_NUM,
        Self::TRUE_PATTERN_CHAR,
        Self::TRUE_PATTERN_NUM,
        Self::LITERAL_START_PATTERN,
        Self::LITERAL_END_PATTERN,
        Self::PARENTHESIS_START_PATTERN,
        Self::PARENTHESIS_END_PATTERN,
    ];

    // FALSE_PATTERN_WORD == "false"
    pub const LONGEST_TOKEN_LEN: usize = 5;

    // TODO make a trait method
    pub fn try_from(value: &'a str) -> Option<IntermediateToken> {
        let patterns = Self::ALL_TOKEN_PATTERNS_FROM_LONGEST;

        let pattern_or_no_match = PATTERN_SET
            .matches(value)
            .into_iter()
            .map(|index| &patterns[index])
            .next();

        pattern_or_no_match.map(|value| Self::from(value))
    }

    // TODO make a trait method
    fn from(pattern: &str) -> IntermediateToken {
        use IntermediateToken::*;

        match pattern.to_lowercase().as_str() {
            Self::AND_PATTERN_BIT
            | Self::AND_PATTERN_LOGIC
            | Self::AND_PATTERN_WORD
            | Self::AND_PATTERN_MATH
            | Self::AND_PATTERN_MATH_2
            | Self::AND_PATTERN_BOOL => And { pattern },

            Self::OR_PATTERN_BIT
            | Self::OR_PATTERN_LOGIC
            | Self::OR_PATTERN_WORD
            | Self::OR_PATTERN_MATH
            | Self::OR_PATTERN_MATH_2
            | Self::OR_PATTERN_BOOL => Or { pattern },

            Self::NOT_PATTERN_TILDE
            | Self::NOT_PATTERN_MARK
            | Self::NOT_PATTERN_WORD
            | Self::NOT_PATTERN_MATH => Not { pattern },

            Self::FALSE_PATTERN_CHAR | Self::FALSE_PATTERN_WORD | Self::FALSE_PATTERN_NUM => {
                ConstantFalse { pattern }
            }
            Self::TRUE_PATTERN_CHAR | Self::TRUE_PATTERN_WORD | Self::TRUE_PATTERN_NUM => {
                ConstantTrue { pattern }
            }
            Self::PARENTHESIS_START_PATTERN => ParenthesesStart,
            Self::PARENTHESIS_END_PATTERN => ParenthesesEnd,
            Self::LITERAL_START_PATTERN => LiteralLongNameStart,
            Self::LITERAL_END_PATTERN => LiteralLongNameEnd,
            _ => panic!(
                "Invalid value passed to IntermediateToken::from: {}",
                pattern
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use regex::Regex;

    use super::*;

    #[test]
    fn test_longest() {
        let actual = IntermediateToken::LONGEST_TOKEN_LEN;
        let expected = IntermediateToken::FALSE_PATTERN_WORD.len();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_ordered_patterns() {
        let tokens = IntermediateToken::ALL_TOKEN_PATTERNS_FROM_LONGEST;

        assert!(tokens
            .iter()
            .zip(tokens.iter().skip(1))
            .all(|(previous, current)| previous.chars().count() >= current.chars().count()))
    }

    #[test]
    fn test_regex_line_start_char_escaped_ok() {
        let and_str_pattern = "^";
        let pattern = Regex::new(&format!(r"(?i)^{}", regex::escape(and_str_pattern))).unwrap();

        let builder = "a&b".to_string();

        assert!(!pattern.is_match(&builder))
    }

    #[test]
    #[should_panic]
    fn test_from_panics() {
        let input = "abcdefgh";

        // test sanity
        assert!(!IntermediateToken::all_token_patterns().contains(&input));

        let _ = IntermediateToken::from(input);
    }
}
