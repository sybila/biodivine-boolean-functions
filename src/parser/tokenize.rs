use itertools::Itertools;

use crate::parser::error::TokenizeError;
use crate::parser::error::TokenizeError::MissingClosingParenthesis;
use crate::parser::structs::{FinalToken, IntermediateToken, PositionTracker};
use crate::parser::utils::SHOULD_END_LITERAL;
use crate::parser::utils::{peek_until_n, pop_n_left, trim_whitespace_left};
use crate::parser::TokenizerInput;

pub fn tokenize(input: &str) -> Result<Vec<FinalToken>, TokenizeError> {
    tokenize_level(&mut PositionTracker::new(input.chars().multipeek()), true)
}

fn tokenize_level(
    input: &mut TokenizerInput,
    is_top_level: bool,
) -> Result<Vec<FinalToken>, TokenizeError> {
    let mut result = vec![];
    let mut buffer = String::new();
    let take_size = IntermediateToken::longest_token_len() + 1;

    // trim whitespace in case of whitespace after opening parenthesis
    trim_whitespace_left(input);

    while peek_until_n(take_size, &mut input.iterator, &mut buffer) || !buffer.is_empty() {
        let intermediate_token = IntermediateToken::try_from(buffer.as_str());

        match intermediate_token {
            None => consume_while_literal(input, &mut result),
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
                    IntermediateToken::ParenthesesStart => handle_parentheses(input, &mut buffer)?,
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
                        consume_until_brace(input, &mut buffer)?
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
        input.iterator.reset_peek();
        buffer.clear();
        trim_whitespace_left(input);
    }

    if is_top_level {
        Ok(result)
    } else {
        Err(MissingClosingParenthesis)
    }
}

fn handle_parentheses(
    input: &mut TokenizerInput,
    buffer: &mut String,
) -> Result<(FinalToken, usize), TokenizeError> {
    // move over from the initial `(`
    pop_n_left(buffer, input, 1);

    let tokens = tokenize_level(input, false)?;
    Ok((FinalToken::Parentheses(tokens), 0))
}

fn consume_until_brace(
    input: &mut TokenizerInput,
    buffer: &mut String,
) -> Result<(FinalToken, usize), TokenizeError> {
    // TODO maybe assert that builder is empty?

    // move over from the initial `{`, resetting peeking
    pop_n_left(buffer, input, 1);
    let mut literal_buffer: String = String::new();
    let mut did_hit_closing_brace = false;
    input.iterator.reset_peek();

    while let Some(c) = input.iterator.peek() {
        if c.to_string() == IntermediateToken::LITERAL_END_PATTERN {
            // move over from the final `}`
            input.next();

            did_hit_closing_brace = true;
            break;
        }

        literal_buffer.push(*c);
        input.next();
    }

    if !did_hit_closing_brace {
        return Err(TokenizeError::MissingClosingCurlyBrace);
    }
    if literal_buffer.is_empty() {
        return Err(TokenizeError::EmptyLiteralName);
    }

    Ok((FinalToken::Literal(literal_buffer), 0))
}

fn consume_while_literal(input: &mut TokenizerInput, result: &mut Vec<FinalToken>) {
    let mut literal_buffer: String = String::new();
    input.iterator.reset_peek();

    while let Some(c) = input.iterator.peek() {
        if SHOULD_END_LITERAL.is_match(&c.to_string()) {
            break;
        }

        literal_buffer.push(*c);
        input.next();
    }

    result.push(FinalToken::Literal(literal_buffer));
}

#[cfg(test)]
mod tests {
    use crate::parser::error::TokenizeError::{
        EmptyLiteralName, MissingClosingCurlyBrace, UnexpectedClosingCurlyBrace,
        UnexpectedClosingParenthesis,
    };
    use crate::parser::structs::FinalToken::*;

    use super::*;

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
        let actual = tokenize("a       &\t\nb")?;
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
    fn test_nospace_parenthesesnotclosed_minimal_nok() -> Result<(), TokenizeError> {
        let actual = tokenize("(");

        assert!(actual.is_err());
        assert_eq!(actual.unwrap_err(), MissingClosingParenthesis);

        Ok(())
    }

    #[test]
    fn test_nospace_parenthesesnotclosed_nested_nok() -> Result<(), TokenizeError> {
        let actual = tokenize("(((()()))");

        assert!(actual.is_err());
        assert_eq!(actual.unwrap_err(), MissingClosingParenthesis);

        Ok(())
    }

    #[test]
    fn test_singlespace_parenthesesnotclosed_minimal_nok() -> Result<(), TokenizeError> {
        let actual = tokenize(" ( ");

        assert!(actual.is_err());
        assert_eq!(actual.unwrap_err(), MissingClosingParenthesis);

        Ok(())
    }

    #[test]
    fn test_singlespace_parenthesesnotclosed_nested_nok() -> Result<(), TokenizeError> {
        let actual = tokenize(" ( ( ( ( ) ( ) ) )");

        assert!(actual.is_err());
        assert_eq!(actual.unwrap_err(), MissingClosingParenthesis);

        Ok(())
    }

    #[test]
    fn test_nospace_parenthesesnotopened_minimal_nok() -> Result<(), TokenizeError> {
        let actual = tokenize(")");

        assert!(actual.is_err());
        assert_eq!(actual.unwrap_err(), UnexpectedClosingParenthesis);

        Ok(())
    }

    #[test]
    fn test_nospace_parenthesesnotopened_nested_nok() -> Result<(), TokenizeError> {
        let actual = tokenize("(((()))))");

        assert!(actual.is_err());
        assert_eq!(actual.unwrap_err(), UnexpectedClosingParenthesis);

        Ok(())
    }

    #[test]
    fn test_singlespace_parenthesesnotopened_minimal_nok() -> Result<(), TokenizeError> {
        let actual = tokenize(" ) ");

        assert!(actual.is_err());
        assert_eq!(actual.unwrap_err(), UnexpectedClosingParenthesis);

        Ok(())
    }

    #[test]
    fn test_singlespace_parenthesesnotopened_nested_nok() -> Result<(), TokenizeError> {
        let actual = tokenize(" ( ( ( ( ) ) ) ) )");

        assert!(actual.is_err());
        assert_eq!(actual.unwrap_err(), UnexpectedClosingParenthesis);

        Ok(())
    }

    #[test]
    fn test_bracenotopened_nok() -> Result<(), TokenizeError> {
        let actual = tokenize("}");

        assert!(actual.is_err());
        assert_eq!(actual.unwrap_err(), UnexpectedClosingCurlyBrace);

        Ok(())
    }

    #[test]
    fn test_bracenotclosed_empty_nok() -> Result<(), TokenizeError> {
        let actual = tokenize("{");

        assert!(actual.is_err());
        assert_eq!(actual.unwrap_err(), MissingClosingCurlyBrace);

        Ok(())
    }

    #[test]
    fn test_bracenotclosed_nonempty_nok() -> Result<(), TokenizeError> {
        let actual = tokenize("{abc&&");

        assert!(actual.is_err());
        assert_eq!(actual.unwrap_err(), MissingClosingCurlyBrace);

        Ok(())
    }

    #[test]
    fn test_brace_ok() -> Result<(), TokenizeError> {
        let actual = tokenize("{abc&&}")?;
        let expected = vec![Literal("abc&&".to_string())];

        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn test_brace_spaces_ok() -> Result<(), TokenizeError> {
        let actual = tokenize("{ abc && }")?;
        let expected = vec![Literal(" abc && ".to_string())];

        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn test_brace_empty_nok() -> Result<(), TokenizeError> {
        let actual = tokenize("{}");

        assert!(actual.is_err());
        assert_eq!(actual.unwrap_err(), EmptyLiteralName);

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
