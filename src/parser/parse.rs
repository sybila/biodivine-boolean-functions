use crate::expressions::Expression;
use crate::expressions::ExpressionNode::{And, Constant, Literal, Not, Or};
use crate::parser::error::ParseTokensError;
use crate::parser::structs::FinalToken;

pub fn parse_tokens(input: &[FinalToken]) -> Result<Expression<String>, ParseTokensError> {
    priority_0_parse_or(input)
}

fn priority_0_parse_or(data: &[FinalToken]) -> Result<Expression<String>, ParseTokensError> {
    let mut es = Vec::new();
    for group in data.split(|t| t == &FinalToken::Or) {
        es.push(priority_1_parse_and(group)?);
    }

    match es.len() {
        0 => Err(ParseTokensError::EmptySideOfOperator),
        1 => Ok(es.remove(0)),
        _ => Ok(Or(es).into()),
    }
}

fn priority_1_parse_and(data: &[FinalToken]) -> Result<Expression<String>, ParseTokensError> {
    let mut es = Vec::new();
    for group in data.split(|t| t == &FinalToken::And) {
        es.push(priority_2_terminal(group)?);
    }

    match es.len() {
        0 => Err(ParseTokensError::EmptySideOfOperator),
        1 => Ok(es.remove(0)),
        _ => Ok(And(es).into()),
    }
}

fn priority_2_terminal(data: &[FinalToken]) -> Result<Expression<String>, ParseTokensError> {
    if data.is_empty() {
        Err(ParseTokensError::EmptySideOfOperator)
    } else if data[0] == FinalToken::Not {
        Ok(Not(priority_2_terminal(&data[1..])?).into())
    } else if data.len() > 1 {
        Err(ParseTokensError::UnexpectedLiteralsGroup)
    } else {
        // data.len() == 1
        match &data[0] {
            FinalToken::ConstantTrue => Ok(Constant(true).into()),
            FinalToken::ConstantFalse => Ok(Constant(false).into()),
            FinalToken::Literal(name) => Ok(Literal(name.clone()).into()),
            FinalToken::Parentheses(inner) => Ok(parse_tokens(inner)?),
            _ => unreachable!(
                "Other tokens are matched by remaining functions, nothing else should remain."
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::expressions::{bool, var};
    use crate::parser::error::ParseTokensError::EmptySideOfOperator;
    use crate::parser::{tokenize, ParseError};
    use crate::traits::SemanticEq;

    use super::*;

    #[test]
    fn test_empty_nok() -> Result<(), ParseError> {
        let input = tokenize("")?;
        let actual = parse_tokens(&input);

        assert!(actual.is_err());
        assert_eq!(actual.unwrap_err(), EmptySideOfOperator);

        Ok(())
    }

    #[test]
    fn test_binaryand_ok() -> Result<(), ParseError> {
        let input = tokenize("a & b")?;
        let actual = parse_tokens(&input)?;
        let expected = var("a") & var("b");

        assert!(actual.semantic_eq(&expected));
        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn test_naryand_ok() -> Result<(), ParseError> {
        let input = tokenize("a & b & c")?;
        let actual = parse_tokens(&input)?;
        let expected = var("a") & var("b") & var("c");

        assert!(actual.semantic_eq(&expected));
        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn test_binaryor_ok() -> Result<(), ParseError> {
        let input = tokenize("a | b")?;
        let actual = parse_tokens(&input)?;
        let expected = var("a") | var("b");

        assert!(actual.semantic_eq(&expected));
        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn test_naryor_ok() -> Result<(), ParseError> {
        let input = tokenize("a | b | c")?;
        let actual = parse_tokens(&input)?;
        let expected = var("a") | var("b") | var("c");

        assert!(actual.semantic_eq(&expected));
        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn test_parentheses_toplevel_ok() -> Result<(), ParseError> {
        let input = tokenize("(a)")?;
        let actual = parse_tokens(&input)?;
        let expected = var("a");

        assert!(actual.semantic_eq(&expected));
        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn test_parentheses_naryor_naryand_ok() -> Result<(), ParseError> {
        let input = tokenize("a | b | (a & b & !c)")?;
        let actual = parse_tokens(&input)?;
        let expected = var("a") | var("b") | (var("a") & var("b") & !var("c"));

        assert!(actual.semantic_eq(&expected));
        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn test_parentheses_naryor_naryand_constants_ok() -> Result<(), ParseError> {
        let input = tokenize("F | 0 | False | (T & 1 & True)")?;
        let actual = parse_tokens(&input)?;
        let expected =
            bool(false) | bool(false) | bool(false) | (bool(true) & bool(true) & bool(true));

        assert!(actual.semantic_eq(&expected));
        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn test_priorities_naryor_naryand_ok() -> Result<(), ParseError> {
        let input = tokenize("a | b | a & b & !c")?;
        let actual = parse_tokens(&input)?;
        let expected = var("a") | var("b") | (var("a") & var("b") & !var("c"));

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
