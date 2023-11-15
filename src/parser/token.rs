#[derive(PartialEq, Debug)]
pub enum IntermediateToken {
    And,
    Or,
    Not,
    ConstantTrue,
    ConstantFalse,
    ParenthesesStart,
    ParenthesesEnd,
    LiteralLongNameStart,
    LiteralLongNameEnd,
}

impl IntermediateToken {
    const AND_PATTERN_BIT: &'static str = "&";
    // const AND_PATTERN_LOGIC: &'static str = "&&";
    // const AND_PATTERN_WORD: &'static str = "and";
    const AND_PATTERN_MATH: &'static str = "∧";
    const AND_PATTERN_MATH_2: &'static str = "^";
    const AND_PATTERN_BOOL: &'static str = "*";

    const AND_PATTERNS: [&'static str; 4] = [
        Self::AND_PATTERN_BIT,
        // Self::AND_PATTERN_LOGIC,
        // Self::AND_PATTERN_WORD,
        Self::AND_PATTERN_MATH,
        Self::AND_PATTERN_MATH_2,
        Self::AND_PATTERN_BOOL,
    ];

    const OR_PATTERN_BIT: &'static str = "|";
    // const OR_PATTERN_LOGIC: &'static str = "||";
    // const OR_PATTERN_WORD: &'static str = "or";
    const OR_PATTERN_MATH: &'static str = "∨";
    // const OR_PATTERN_MATH_2: &'static str = "v";
    const OR_PATTERN_BOOL: &'static str = "+";
    const OR_PATTERNS: [&'static str; 3] = [
        Self::OR_PATTERN_BIT,
        // Self::OR_PATTERN_LOGIC,
        // Self::OR_PATTERN_WORD,
        Self::OR_PATTERN_MATH,
        // Self::OR_PATTERN_MATH_2,
        Self::OR_PATTERN_BOOL,
    ];

    const NOT_PATTERN_TILDE: &'static str = "~";
    const NOT_PATTERN_MARK: &'static str = "!";
    // const NOT_PATTERN_WORD: &'static str = "not";
    const NOT_PATTERN_MATH: &'static str = "¬";
    const NOT_PATTERNS: [&'static str; 3] = [
        Self::NOT_PATTERN_TILDE,
        Self::NOT_PATTERN_MARK,
        // Self::NOT_PATTERN_WORD,
        Self::NOT_PATTERN_MATH,
    ];

    // const TRUE_PATTERN_CHAR: &'static str = "t";
    // const TRUE_PATTERN_WORD: &'static str = "true";
    const TRUE_PATTERN_NUM: &'static str = "1";

    const TRUE_PATTERNS: [&'static str; 1] = [
        // Self::TRUE_PATTERN_CHAR,
        // Self::TRUE_PATTERN_WORD,
        Self::TRUE_PATTERN_NUM,
    ];

    // const FALSE_PATTERN_CHAR: &'static str = "f";
    // const FALSE_PATTERN_WORD: &'static str = "false";
    const FALSE_PATTERN_NUM: &'static str = "0";

    const FALSE_PATTERNS: [&'static str; 1] = [
        // Self::FALSE_PATTERN_CHAR,
        // Self::FALSE_PATTERN_WORD,
        Self::FALSE_PATTERN_NUM,
    ];

    const LITERAL_START_PATTERN: &'static str = "{";
    pub const LITERAL_END_PATTERN: &'static str = "}";

    pub fn all_token_patterns<'a>() -> Vec<&'a str> {
        [
            Self::AND_PATTERNS.as_slice(),
            Self::OR_PATTERNS.as_slice(),
            Self::NOT_PATTERNS.as_slice(),
            Self::FALSE_PATTERNS.as_slice(),
            Self::TRUE_PATTERNS.as_slice(),
        ]
        .concat()
    }

    pub fn longest_token_len() -> usize {
        Self::all_token_patterns()
            .iter()
            .max_by(|a, b| a.chars().count().cmp(&b.chars().count()))
            .expect("No patterns defined in the library")
            .chars()
            .count()
    }

    pub fn try_from(value: &str) -> Option<IntermediateToken> {
        use IntermediateToken::*;

        match value.to_lowercase().as_str() {
            Self::AND_PATTERN_BIT
            // | Self::AND_PATTERN_LOGIC
            // | Self::AND_PATTERN_WORD
            | Self::AND_PATTERN_MATH
            | Self::AND_PATTERN_MATH_2
            | Self::AND_PATTERN_BOOL => Some(And),

            Self::OR_PATTERN_BIT
            // | Self::OR_PATTERN_LOGIC
            // | Self::OR_PATTERN_WORD
            | Self::OR_PATTERN_MATH
            // | Self::OR_PATTERN_MATH_2
            | Self::OR_PATTERN_BOOL => Some(Or),

            Self::NOT_PATTERN_TILDE |
            Self::NOT_PATTERN_MARK |
            // Self::NOT_PATTERN_WORD |
            Self::NOT_PATTERN_MATH
            => Some(Not),

            // | Self::FALSE_PATTERN_CHAR
            // | Self::FALSE_PATTERN_WORD
            Self::FALSE_PATTERN_NUM => {
                Some(ConstantFalse)
            }
            // | Self::TRUE_PATTERN_CHAR
            // | Self::TRUE_PATTERN_WORD
            Self::TRUE_PATTERN_NUM  => {
                Some(ConstantTrue)
            }
            "(" => Some(ParenthesesStart),
            ")" => Some(ParenthesesEnd),
            Self::LITERAL_START_PATTERN => Some(LiteralLongNameStart),
            Self::LITERAL_END_PATTERN => Some(LiteralLongNameEnd),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest() {
        let actual = IntermediateToken::longest_token_len();
        // let expected = Token::FALSE_PATTERN_WORD.len();
        let expected = 1;

        assert_eq!(actual, expected);
    }
}
