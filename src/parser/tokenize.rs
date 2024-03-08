use std::str::Chars;

use itertools::{Itertools, MultiPeek};
use regex::Regex;

use crate::parser::error::TokenizeError;
use crate::parser::error::TokenizeError::MissingClosingParenthesis;
use crate::parser::token::IntermediateToken;

#[derive(PartialEq, Debug)]
pub enum FinalToken {
    And,
    Or,
    Not,
    ConstantTrue,
    ConstantFalse,
    Literal(String),
    Parentheses(Vec<FinalToken>),
}

pub fn tokenize(input: &str) -> Result<Vec<FinalToken>, TokenizeError> {
    tokenize_level(&mut input.chars().multipeek(), true)
}

fn tokenize_level(
    input: &mut MultiPeek<Chars>,
    is_top_level: bool,
) -> Result<Vec<FinalToken>, TokenizeError> {
    let mut result = vec![];
    let mut buffer = String::new();
    let take_size = IntermediateToken::longest_token_len() + 1;

    // TODO make regex lazy-static
    let should_end_literal = Regex::new(r"[^-_a-zA-Z0-9]").unwrap();

    trim_whitespace_left(input);
    while peek_until_n(take_size, input, &mut buffer) || !buffer.is_empty() {
        let intermediate_token = IntermediateToken::try_from(buffer.as_str());

        match intermediate_token {
            None => {
                let mut literal_buffer: String = String::new();
                input.reset_peek();

                while let Some(c) = input.peek() {
                    if should_end_literal.is_match(&c.to_string()) {
                        break;
                    }

                    literal_buffer.push(*c);
                    input.next();
                }

                result.push(FinalToken::Literal(literal_buffer));
            }
            Some(token) => {
                let (final_token, pattern_length) = match token {
                    IntermediateToken::And { pattern } => {
                        (FinalToken::And, pattern.chars().count())
                    }
                    IntermediateToken::Or { pattern } => (FinalToken::Or, pattern.chars().count()),
                    IntermediateToken::Not { pattern } => {
                        (FinalToken::Not, pattern.chars().count())
                    }
                    IntermediateToken::ConstantTrue { pattern } => {
                        (FinalToken::ConstantTrue, pattern.chars().count())
                    }
                    IntermediateToken::ConstantFalse { pattern } => {
                        (FinalToken::ConstantFalse, pattern.chars().count())
                    }
                    IntermediateToken::ParenthesesStart => {
                        // move over from the initial `(`
                        pop_n_left(&mut buffer, input, 1);

                        let tokens = tokenize_level(input, false)?;
                        (FinalToken::Parentheses(tokens), 0)
                    }
                    IntermediateToken::ParenthesesEnd => {
                        return if is_top_level {
                            Err(TokenizeError::UnexpectedClosingParenthesis)
                        } else {
                            // move over from the final `)`
                            pop_n_left(&mut buffer, input, 1);

                            Ok(result)
                        };
                    }
                    IntermediateToken::LiteralLongNameStart => {
                        // TODO maybe assert that builder is empty?

                        // move over from the initial `{`, resetting peeking
                        pop_n_left(&mut buffer, input, 1);
                        let mut literal_buffer: String = String::new();
                        input.reset_peek();

                        while let Some(c) = input.peek() {
                            if c.to_string() == IntermediateToken::LITERAL_END_PATTERN {
                                // move over from the final `}`
                                input.next();
                                break;
                            }

                            literal_buffer.push(*c);
                            input.next();
                        }

                        // TODO return TokenizeError if builder is empty at the end
                        (FinalToken::Literal(literal_buffer), 0)
                    }
                    IntermediateToken::LiteralLongNameEnd => {
                        return Err(TokenizeError::UnexpectedClosingCurlyBrace);
                    }
                };

                result.push(final_token);
                pop_n_left(&mut buffer, input, pattern_length);
            }
        }

        // TODO try to reconcile this to not require resetting peeking after every iteration,
        // TODO but to use what's in the buffer already
        input.reset_peek();
        buffer.clear();
        trim_whitespace_left(input);
    }

    if is_top_level {
        Ok(result)
    } else {
        Err(MissingClosingParenthesis)
    }
}

fn trim_whitespace_left(input: &mut MultiPeek<Chars>) {
    while let Some(c) = input.peek() {
        if !c.is_whitespace() {
            break;
        }

        input.next();
    }
    input.reset_peek();
}

// https://stackoverflow.com/a/38447886
/// Advances (i.e. calls .next()) `input` by `pop_count` characters, clears `pop_count` characters from the start of the `buffer`.
fn pop_n_left(buffer: &mut String, input: &mut MultiPeek<Chars>, pop_count: usize) {
    for _ in 0..pop_count {
        input.next();
    }

    match buffer.char_indices().nth(pop_count) {
        Some((pos, _)) => {
            buffer.drain(..pos);
        }
        None => {
            buffer.clear();
        }
    }
}

/// Returns `false` if if no characters were peeked from the `input` iterator, `true` otherwise.
fn peek_until_n(n: usize, input: &mut MultiPeek<Chars>, buffer: &mut String) -> bool {
    let mut did_read_anything = false;

    while buffer.chars().count() < n {
        let c = input.peek();

        match c {
            Some(c) => {
                did_read_anything = true;
                buffer.push(*c)
            }
            None => break,
        }
    }

    did_read_anything
}

#[cfg(test)]
mod tests {
    use super::FinalToken::*;
    use super::*;

    #[test]
    fn test_peek_n() {
        for input_size in 0..=10 {
            let input = "a".repeat(input_size);
            let mut buffer = String::new();

            let mut input = input.chars().multipeek();

            let target_peak = 6;
            let did_read_anything = peek_until_n(target_peak, &mut input, &mut buffer);

            assert_eq!(did_read_anything, input_size >= 1, "i: {input_size}");
            assert_eq!(
                buffer.chars().count(),
                usize::min(input_size, target_peak),
                "i: {input_size}"
            );
        }
    }

    #[test]
    fn test_pop_from_start() {
        let mut buffer = String::from("");
        let input = buffer.clone();
        let mut input = input.chars().multipeek();

        pop_n_left(&mut buffer, &mut input, 3);
        assert_eq!(&buffer, "");
        assert_eq!(&input.join(""), "");

        let mut buffer = String::from("a");
        pop_n_left(&mut buffer, &mut input, 3);
        let input = buffer.clone();
        let mut input = input.chars().multipeek();
        assert_eq!(&buffer, "");
        assert_eq!(&input.join(""), "");

        let mut buffer = String::from("ab");
        pop_n_left(&mut buffer, &mut input, 3);
        let input = buffer.clone();
        let mut input = input.chars().multipeek();
        assert_eq!(&buffer, "");
        assert_eq!(&input.join(""), "");

        let mut buffer = String::from("abc");
        pop_n_left(&mut buffer, &mut input, 3);
        let input = buffer.clone();
        let mut input = input.chars().multipeek();
        assert_eq!(&buffer, "");
        assert_eq!(&input.join(""), "");

        let mut buffer = String::from("abcd");
        pop_n_left(&mut buffer, &mut input, 3);
        let input = buffer.clone();
        let mut input = input.chars().multipeek();
        assert_eq!(&buffer, "d");
        assert_eq!(&input.join(""), "d");
    }

    #[test]
    fn test_charvar_ok() -> Result<(), TokenizeError> {
        let actual = tokenize("a")?;
        let expected = vec![Literal("a".to_string())];

        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn test_stringvar_short_ok() -> Result<(), TokenizeError> {
        let actual = tokenize("abcd")?;
        let expected = vec![Literal("abcd".to_string())];

        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn test_stringvar_long_ok() -> Result<(), TokenizeError> {
        let actual = tokenize("abcdefgh")?;
        let expected = vec![Literal("abcdefgh".to_string())];

        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn test_charvar_and_singlespace_ok() -> Result<(), TokenizeError> {
        let actual = tokenize("a & b")?;
        let expected = vec![Literal("a".to_string()), And, Literal("b".to_string())];

        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn test_charvar_and_nospace_ok() -> Result<(), TokenizeError> {
        let actual = tokenize("a&b")?;
        let expected = vec![Literal("a".to_string()), And, Literal("b".to_string())];

        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn test_charvar_and_crazyspace_ok() -> Result<(), TokenizeError> {
        let actual = tokenize("a       &\nb")?;
        let expected = vec![Literal("a".to_string()), And, Literal("b".to_string())];

        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn test_stringvar_and_singlespace_ok() -> Result<(), TokenizeError> {
        let actual = tokenize("{a} & {b}")?;
        let expected = vec![Literal("a".to_string()), And, Literal("b".to_string())];

        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn test_stringvar_and_nospace_ok() -> Result<(), TokenizeError> {
        let actual = tokenize("{a}&{b}")?;
        let expected = vec![Literal("a".to_string()), And, Literal("b".to_string())];

        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn test_stringvar_and_crazyspace_ok() -> Result<(), TokenizeError> {
        let actual = tokenize("{a}       &\n\t{b}")?;
        let expected = vec![Literal("a".to_string()), And, Literal("b".to_string())];

        assert_eq!(actual, expected);

        Ok(())
    }

    fn all_tokens() -> String {
        IntermediateToken::all_token_patterns().join("")
    }

    fn all_tokens_without_literal_delimiters() -> String {
        all_tokens()
            .replace(IntermediateToken::LITERAL_START_PATTERN, "")
            .replace(IntermediateToken::LITERAL_END_PATTERN, "")
    }

    #[test]
    fn test_nonalphastringvar_and_singlespace_ok() -> Result<(), TokenizeError> {
        // do not contain curly braces in check
        let name = format!("{{{0}}} & {{{0}}}", all_tokens_without_literal_delimiters());
        let actual = tokenize(name.as_str())?;
        let expected = vec![
            Literal(all_tokens_without_literal_delimiters()),
            And,
            Literal(all_tokens_without_literal_delimiters()),
        ];

        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn test_nonalphastringvar_and_nospace_ok() -> Result<(), TokenizeError> {
        let name = format!("{{{0}}}&{{{0}}}", all_tokens_without_literal_delimiters());
        let actual = tokenize(name.as_str())?;
        let expected = vec![
            Literal(all_tokens_without_literal_delimiters()),
            And,
            Literal(all_tokens_without_literal_delimiters()),
        ];

        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn test_nonalphastringvar_and_crazyspace_ok() -> Result<(), TokenizeError> {
        let name = format!(
            "{{{0}}}       &\n\t{{{0}}}",
            all_tokens_without_literal_delimiters()
        );
        let actual = tokenize(name.as_str())?;
        let expected = vec![
            Literal(all_tokens_without_literal_delimiters()),
            And,
            Literal(all_tokens_without_literal_delimiters()),
        ];

        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn test_charvar_andor_simplespace_singleparentheses_ok() -> Result<(), TokenizeError> {
        let actual = tokenize("(a & b)")?;
        let expected = vec![Parentheses(vec![
            Literal("a".to_string()),
            And,
            Literal("b".to_string()),
        ])];

        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn test_charvar_andor_simplespace_mediumparentheses_ok() -> Result<(), TokenizeError> {
        let actual = tokenize("(a & b) | (c & d)")?;
        let expected = vec![
            Parentheses(vec![
                Literal("a".to_string()),
                And,
                Literal("b".to_string()),
            ]),
            Or,
            Parentheses(vec![
                Literal("c".to_string()),
                And,
                Literal("d".to_string()),
            ]),
        ];

        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn test_charvar_and_simplespace_varparentheses_ok() -> Result<(), TokenizeError> {
        let actual = tokenize("( a ) & b")?;
        let expected = vec![
            Parentheses(vec![Literal("a".to_string())]),
            And,
            Literal("b".to_string()),
        ];

        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn test_charvar_and_nospace_varparentheses_ok() -> Result<(), TokenizeError> {
        let actual = tokenize("(a)&b")?;
        let expected = vec![
            Parentheses(vec![Literal("a".to_string())]),
            And,
            Literal("b".to_string()),
        ];

        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn test_charvar_and_nospace_simplearentheses_ok() -> Result<(), TokenizeError> {
        let actual = tokenize("(a&b)")?;
        let expected = vec![Parentheses(vec![
            Literal("a".to_string()),
            And,
            Literal("b".to_string()),
        ])];

        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn test_charvar_andor_nospace_mediumparentheses_ok() -> Result<(), TokenizeError> {
        let actual = tokenize("(a&b)|(c&d)")?;
        let expected = vec![
            Parentheses(vec![
                Literal("a".to_string()),
                And,
                Literal("b".to_string()),
            ]),
            Or,
            Parentheses(vec![
                Literal("c".to_string()),
                And,
                Literal("d".to_string()),
            ]),
        ];

        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn test_charvar_alloperators_simplespace_crazyparenthesesright_ok() -> Result<(), TokenizeError>
    {
        let actual = tokenize("( ! a & ( b | ( c | ( 0 & 1 ) ) ) )")?;
        let expected = vec![Parentheses(vec![
            Not,
            Literal("a".to_string()),
            And,
            Parentheses(vec![
                Literal("b".to_string()),
                Or,
                Parentheses(vec![
                    Literal("c".to_string()),
                    Or,
                    Parentheses(vec![ConstantFalse, And, ConstantTrue]),
                ]),
            ]),
        ])];

        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn test_charvar_alloperators_nospace_crazyparenthesesright_ok() -> Result<(), TokenizeError> {
        let actual = tokenize("(!a&(b|(c|(0&1))))")?;
        let expected = vec![Parentheses(vec![
            Not,
            Literal("a".to_string()),
            And,
            Parentheses(vec![
                Literal("b".to_string()),
                Or,
                Parentheses(vec![
                    Literal("c".to_string()),
                    Or,
                    Parentheses(vec![ConstantFalse, And, ConstantTrue]),
                ]),
            ]),
        ])];

        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn test_charvar_alloperators_simplespace_crazyparenthesesleft_ok() -> Result<(), TokenizeError>
    {
        let actual = tokenize("( ( ( ( ( 0 & 1 ) | c ) | b ) & ! a ) )")?;
        let expected = vec![Parentheses(vec![Parentheses(vec![
            Parentheses(vec![
                Parentheses(vec![
                    Parentheses(vec![ConstantFalse, And, ConstantTrue]),
                    Or,
                    Literal("c".to_string()),
                ]),
                Or,
                Literal("b".to_string()),
            ]),
            And,
            Not,
            Literal("a".to_string()),
        ])])];

        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn test_charvar_alloperators_nospace_crazyparenthesesleft_ok() -> Result<(), TokenizeError> {
        let actual = tokenize("(((((0&1)|c)|b)&!a))")?;
        let expected = vec![Parentheses(vec![Parentheses(vec![
            Parentheses(vec![
                Parentheses(vec![
                    Parentheses(vec![ConstantFalse, And, ConstantTrue]),
                    Or,
                    Literal("c".to_string()),
                ]),
                Or,
                Literal("b".to_string()),
            ]),
            And,
            Not,
            Literal("a".to_string()),
        ])])];

        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn test_operator_boundary_andword_space_charvar_ok() -> Result<(), TokenizeError> {
        let actual = tokenize("a and b")?;
        let expected = vec![Literal("a".to_string()), And, Literal("b".to_string())];

        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn test_operator_boundary_andword_nospace_charvar_ok() -> Result<(), TokenizeError> {
        let actual = tokenize("a andB")?;
        let expected = vec![Literal("a".to_string()), Literal("andB".to_string())];

        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn test_operator_boundary_andword_nospace_operatorvar_ok() -> Result<(), TokenizeError> {
        let actual = tokenize("a andBand b")?;
        let expected = vec![
            Literal("a".to_string()),
            Literal("andBand".to_string()),
            Literal("b".to_string()),
        ];

        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn test_operator_boundary_andlogic_space_charvar_ok() -> Result<(), TokenizeError> {
        let actual = tokenize("a && b")?;
        let expected = vec![Literal("a".to_string()), And, Literal("b".to_string())];

        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn test_operator_boundary_andlogic_nospace_charvar_ok() -> Result<(), TokenizeError> {
        let actual = tokenize("a &&b")?;
        let expected = vec![Literal("a".to_string()), And, Literal("b".to_string())];

        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn test_operator_boundary_andlogic_nospace_operatorvar_ok() -> Result<(), TokenizeError> {
        let actual = tokenize("a &&B&& b")?;
        let expected = vec![
            Literal("a".to_string()),
            And,
            Literal("B".to_string()),
            And,
            Literal("b".to_string()),
        ];

        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn test_operator_boundary_andword_space_underscorevar_ok() -> Result<(), TokenizeError> {
        let actual = tokenize("a and _B")?;
        let expected = vec![Literal("a".to_string()), And, Literal("_B".to_string())];

        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn test_operator_boundary_andword_nospace_underscorevar_ok() -> Result<(), TokenizeError> {
        let actual = tokenize("a and_B")?;
        let expected = vec![Literal("a".to_string()), Literal("and_B".to_string())];

        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn test_operator_boundary_andlogic_space_underscorevar_ok() -> Result<(), TokenizeError> {
        let actual = tokenize("a && _B")?;
        let expected = vec![Literal("a".to_string()), And, Literal("_B".to_string())];

        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn test_operator_boundary_andlogic_nospace_underscorevar_ok() -> Result<(), TokenizeError> {
        let actual = tokenize("a &&_B")?;
        let expected = vec![Literal("a".to_string()), And, Literal("_B".to_string())];

        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn test_operator_boundary_andword_space_dashvar_ok() -> Result<(), TokenizeError> {
        let actual = tokenize("a and -B")?;
        let expected = vec![Literal("a".to_string()), And, Literal("-B".to_string())];

        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn test_operator_boundary_andword_nospace_dashvar_ok() -> Result<(), TokenizeError> {
        let actual = tokenize("a and-B")?;
        let expected = vec![Literal("a".to_string()), Literal("and-B".to_string())];

        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn test_operator_boundary_andlogic_space_dashvar_ok() -> Result<(), TokenizeError> {
        let actual = tokenize("a && -B")?;
        let expected = vec![Literal("a".to_string()), And, Literal("-B".to_string())];

        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn test_operator_boundary_andlogic_nospace_dashvar_ok() -> Result<(), TokenizeError> {
        let actual = tokenize("a &&-B")?;
        let expected = vec![Literal("a".to_string()), And, Literal("-B".to_string())];

        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn test_operator_boundary_false() -> Result<(), TokenizeError> {
        let actual = tokenize("F and andF && False && andFALSE &&FALSE and FALSEand")?;
        let expected = vec![
            ConstantFalse,
            And,
            Literal("andF".to_string()),
            And,
            ConstantFalse,
            And,
            Literal("andFALSE".to_string()),
            And,
            ConstantFalse,
            And,
            Literal("FALSEand".to_string()),
        ];

        assert_eq!(actual, expected);

        Ok(())
    }
}
