use itertools::Itertools;

use crate::expressions::Expression;
use crate::parser::error::ParseTokensError;
use crate::parser::tokenize::FinalToken;

pub fn parse_tokens(input: &[FinalToken]) -> Result<Expression<String>, ParseTokensError> {
    priority_0_parse_or_alt(input)
    // priority_0_parse_or(input)
}

fn index_of_first(data: &[FinalToken], token: FinalToken) -> Option<usize> {
    data.iter().position(|t| *t == token)
}

fn priority_0_parse_or_alt(data: &[FinalToken]) -> Result<Expression<String>, ParseTokensError> {
    data.split(|t| t == &FinalToken::Or)
        .map(priority_1_parse_and_alt)
        .fold_ok(None::<Expression<String>>, |acc, item| match acc {
            None => Some(item),
            Some(Expression::Or(mut es)) => {
                es.push(item);
                Some(Expression::n_ary_or(es))
            }
            Some(previous) => Some(Expression::n_ary_or(vec![previous, item])),
        })?
        .ok_or(ParseTokensError::EmptySideOfOperator)
}

#[allow(dead_code)]
fn priority_0_parse_or(data: &[FinalToken]) -> Result<Expression<String>, ParseTokensError> {
    let maybe_or_position = index_of_first(data, FinalToken::Or);

    let result = if let Some(or_position) = maybe_or_position {
        Expression::binary_or(
            priority_1_parse_and(&data[..or_position])?,
            priority_0_parse_or(&data[(or_position + 1)..])?,
        )
    } else {
        priority_1_parse_and(data)?
    };

    Ok(result)
}

fn priority_1_parse_and_alt(data: &[FinalToken]) -> Result<Expression<String>, ParseTokensError> {
    data.split(|t| t == &FinalToken::And)
        .map(priority_2_terminal)
        .fold_ok(None::<Expression<String>>, |acc, item| match acc {
            None => Some(item),
            Some(Expression::And(mut es)) => {
                es.push(item);
                Some(Expression::n_ary_and(es))
            }
            Some(previous) => Some(Expression::n_ary_and(vec![previous, item])),
        })?
        .ok_or(ParseTokensError::EmptySideOfOperator)
}

#[allow(dead_code)]
fn priority_1_parse_and(data: &[FinalToken]) -> Result<Expression<String>, ParseTokensError> {
    let maybe_and_position = index_of_first(data, FinalToken::And);

    let result = if let Some(and_position) = maybe_and_position {
        Expression::binary_and(
            priority_2_terminal(&data[..and_position])?,
            priority_1_parse_and(&data[(and_position + 1)..])?,
        )
    } else {
        priority_2_terminal(data)?
    };

    Ok(result)
}

fn priority_2_terminal(data: &[FinalToken]) -> Result<Expression<String>, ParseTokensError> {
    if data.is_empty() {
        Err(ParseTokensError::EmptySideOfOperator)
    } else if data[0] == FinalToken::Not {
        Ok(Expression::negate(priority_2_terminal(&data[1..])?))
    } else if data.len() > 1 {
        Err(ParseTokensError::UnexpectedLiteralsGroup)
    } else {
        match &data[0] {
            FinalToken::ConstantTrue => Ok(Expression::Constant(true)),
            FinalToken::ConstantFalse => Ok(Expression::Constant(false)),
            FinalToken::Literal(name) => Ok(Expression::Literal(name.clone())),
            FinalToken::Parentheses(inner) => Ok(parse_tokens(inner)?),
            _ => unreachable!(
                "Other tokens are matched by remaining functions, nothing else should remain."
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::expressions::Expression::Literal;
    use crate::parser::{tokenize, ParseError};
    use crate::traits::SemanticEq;

    use super::*;

    #[test]
    fn test_binaryand_ok() -> Result<(), ParseError> {
        let input = tokenize("a & b")?;
        let actual = parse_tokens(&input)?;
        let expected = Expression::binary_and(Literal("a".to_string()), Literal("b".to_string()));

        assert!(actual.semantic_eq(&expected));
        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn test_naryand_ok() -> Result<(), ParseError> {
        let input = tokenize("a & b & c")?;
        let actual = parse_tokens(&input)?;
        let expected = Expression::n_ary_and(vec![
            Literal("a".to_string()),
            Literal("b".to_string()),
            Literal("c".to_string()),
        ]);

        assert!(actual.semantic_eq(&expected));
        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn test_binaryor_ok() -> Result<(), ParseError> {
        let input = tokenize("a | b")?;
        let actual = parse_tokens(&input)?;
        let expected = Expression::binary_or(Literal("a".to_string()), Literal("b".to_string()));

        assert!(actual.semantic_eq(&expected));
        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn test_naryor_ok() -> Result<(), ParseError> {
        let input = tokenize("a | b | c")?;
        let actual = parse_tokens(&input)?;
        let expected = Expression::n_ary_or(vec![
            Literal("a".to_string()),
            Literal("b".to_string()),
            Literal("c".to_string()),
        ]);

        assert!(actual.semantic_eq(&expected));
        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn test_parentheses_toplevel_ok() -> Result<(), ParseError> {
        let input = tokenize("(a)")?;
        let actual = parse_tokens(&input)?;
        let expected = Literal("a".to_string());

        assert!(actual.semantic_eq(&expected));
        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn test_parentheses_naryor_naryand_ok() -> Result<(), ParseError> {
        let input = tokenize("a | b | (a & b & !c)")?;
        let actual = parse_tokens(&input)?;
        let expected = Expression::n_ary_or(vec![
            Literal("a".to_string()),
            Literal("b".to_string()),
            Expression::n_ary_and(vec![
                Literal("a".to_string()),
                Literal("b".to_string()),
                Expression::negate(Literal("c".to_string())),
            ]),
        ]);

        assert!(actual.semantic_eq(&expected));
        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn test_priorities_naryor_naryand_ok() -> Result<(), ParseError> {
        let input = tokenize("a | b | a & b & !c")?;
        let actual = parse_tokens(&input)?;
        let expected = Expression::n_ary_or(vec![
            Literal("a".to_string()),
            Literal("b".to_string()),
            Expression::n_ary_and(vec![
                Literal("a".to_string()),
                Literal("b".to_string()),
                Expression::negate(Literal("c".to_string())),
            ]),
        ]);

        assert!(actual.semantic_eq(&expected));
        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn test_terminal_and_nok() -> Result<(), ParseError> {
        let input = tokenize("a & b c")?;
        let actual = parse_tokens(&input);

        assert!(actual.is_err());
        assert_eq!(
            actual.unwrap_err(),
            ParseTokensError::UnexpectedLiteralsGroup
        );

        Ok(())
    }
}
