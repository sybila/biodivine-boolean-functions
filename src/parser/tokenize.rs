use std::str::Chars;

use itertools::{Itertools, MultiPeek};

use crate::parser::error::TokenizeError;
use crate::parser::error::TokenizeError::{MissingClosingParenthesis, UnexpectedWhitespace};
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

    let mut builder = String::new();

    while let Some(c) = input.peek() {
        // started parsing the next token

        // -1 for the char already there

        // if c.is_whitespace() {
        //     if builder.is_empty() {
        //         input.next();
        //     } else {
        //         // What is in builder isn't any token
        //         for builder_c in builder.chars() {
        //             result.push(Token::MaybeLiteral(builder_c));
        //             input.next();
        //         }
        //         builder.clear();
        //     }
        //
        //     continue;
        // }

        builder.push(*c);

        let final_token = match IntermediateToken::try_from(builder.as_str()) {
            None if c.is_whitespace() => {
                if builder.is_empty() || builder.chars().all(char::is_whitespace) {
                    advance_all_build_and_clear(input, &mut builder);
                    continue;
                } else {
                    return Err(UnexpectedWhitespace);
                }
            }
            // None => FinalToken::Literal(std::mem::take(&mut builder)),
            None => FinalToken::Literal(builder.clone()),
            Some(token) => {
                match token {
                    IntermediateToken::And { .. } => FinalToken::And,
                    IntermediateToken::Or { .. } => FinalToken::Or,
                    IntermediateToken::Not { .. } => FinalToken::Not,
                    IntermediateToken::ConstantTrue { .. } => FinalToken::ConstantTrue,
                    IntermediateToken::ConstantFalse { .. } => FinalToken::ConstantFalse,
                    IntermediateToken::ParenthesesStart => {
                        // move over from the initial `(`
                        advance_one_and_pop(input, &mut builder);

                        let tokens = tokenize_level(input, false)?;
                        FinalToken::Parentheses(tokens)
                    }
                    IntermediateToken::ParenthesesEnd => {
                        return if is_top_level {
                            Err(TokenizeError::UnexpectedClosingParenthesis)
                        } else {
                            // move over from the final `)`
                            input.next();

                            Ok(result)
                        };
                    }
                    IntermediateToken::LiteralLongNameStart => {
                        // maybe assert that builder is empty?

                        // move over from the initial `{`
                        advance_one_and_pop(input, &mut builder);

                        while let Some(c) = input.peek() {
                            if c.to_string() == IntermediateToken::LITERAL_END_PATTERN {
                                // move over from the final `}`
                                input.next();
                                break;
                            }

                            builder.push(*c);
                        }

                        // FinalToken::Literal(std::mem::take(&mut builder))
                        FinalToken::Literal(builder.clone())
                    }
                    IntermediateToken::LiteralLongNameEnd => {
                        return Err(TokenizeError::UnexpectedClosingCurlyBrace);
                    }
                }
            }
        };

        result.push(final_token);
        advance_all_build_and_clear(input, &mut builder);
    }

    if is_top_level {
        Ok(result)
    } else {
        Err(MissingClosingParenthesis)
    }
}

fn advance_all_build_and_clear(input: &mut MultiPeek<Chars>, builder: &mut String) {
    for _ in 0..builder.chars().count() {
        input.next();
    }
    builder.clear();
}

fn advance_one_and_pop(input: &mut MultiPeek<Chars>, builder: &mut String) {
    input.next();
    builder.pop();
}

#[cfg(test)]
mod tests {
    use super::FinalToken::*;
    use super::*;

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
}
