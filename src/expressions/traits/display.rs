use crate::expressions::Expression;
use crate::expressions::ExpressionNode::{And, Constant, Literal, Not, Or};
use std::fmt::{Debug, Display, Error, Formatter};
use std::hash::Hash;

impl<TLiteral: Debug + Clone + Eq + Hash + Display> Display for Expression<TLiteral> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self.node() {
            Constant(value) => write!(f, "{}", value),
            Literal(name) => write!(f, "{}", name),
            Not(inner) => write!(f, "!{}", inner),
            And(expressions) | Or(expressions) => write!(
                f,
                "({})",
                expressions
                    .iter()
                    .fold(String::new(), |acc, elem| format!("{acc} & {elem}"))
            ),
        }
    }
}
