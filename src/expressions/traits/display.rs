use crate::expressions::Expression;
use crate::expressions::ExpressionNode::{And, Constant, Literal, Not, Or};
use itertools::Itertools;
use std::fmt::{Debug, Display, Error, Formatter};
use std::hash::Hash;

impl<TLiteral: Debug + Clone + Eq + Hash + Display> Display for Expression<TLiteral> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self.node() {
            Constant(value) => write!(f, "{}", value),
            Literal(name) => write!(f, "{}", name),
            Not(inner) => write!(f, "!({})", inner),
            And(expressions) => Self::fmt_nary_expression(f, expressions, '&'),
            Or(expressions) => Self::fmt_nary_expression(f, expressions, '|'),
        }
    }
}

impl<TLiteral: Debug + Clone + Eq + Hash + Display> Expression<TLiteral> {
    fn fmt_nary_expression(
        f: &mut Formatter,
        expressions: &[Expression<TLiteral>],
        operator: char,
    ) -> Result<(), Error> {
        write!(f, "({})", expressions.iter().join(&format!(" {operator} ")))
    }
}

#[cfg(test)]
mod tests {
    use crate::expressions::{bool, var};

    #[test]
    fn test_display_ok() {
        let input = var("a") & !var("b") | bool(true);

        let actual = input.to_string();
        let expected = "((a & !(b)) | true)";

        assert_eq!(actual, expected)
    }
}
