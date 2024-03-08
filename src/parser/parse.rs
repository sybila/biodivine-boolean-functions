use itertools::Itertools;

use crate::expressions::Expression;
use crate::parser::error::ParseTokensError;
use crate::parser::tokenize::FinalToken;

pub fn parse_tokens(input: &[FinalToken]) -> Result<Expression<String>, ParseTokensError> {
    priority_0_parse_or(input)
}

fn priority_0_parse_or(data: &[FinalToken]) -> Result<Expression<String>, ParseTokensError> {
    data.split(|t| t == &FinalToken::Or)
        .map(priority_1_parse_and)
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

fn priority_1_parse_and(data: &[FinalToken]) -> Result<Expression<String>, ParseTokensError> {
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
    use crate::expressions::Expression::{Constant, Literal};
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
    fn test_parentheses_naryor_naryand_constants_ok() -> Result<(), ParseError> {
        let input = tokenize("F | 0 | False | (T & 1 & True)")?;
        let actual = parse_tokens(&input)?;
        let expected = Expression::n_ary_or(vec![
            Constant(false),
            Constant(false),
            Constant(false),
            Expression::n_ary_and(vec![Constant(true), Constant(true), Constant(true)]),
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
    fn test_terminal_and_emptyside_nok() -> Result<(), ParseError> {
        let input = tokenize("a & ")?;
        let actual = parse_tokens(&input);

        assert!(actual.is_err());
        assert_eq!(actual.unwrap_err(), ParseTokensError::EmptySideOfOperator);

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
