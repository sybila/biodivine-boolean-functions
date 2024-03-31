use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Display, Error, Formatter};
use std::hash::Hash;
use std::ops::{BitAnd, BitOr};

use itertools::Itertools;

use crate::parser::{parse_tokens, tokenize, ParseError};
use crate::traits::{Evaluate, GatherLiterals, Parse, PowerSet, SemanticEq};

use super::Expression;
use super::ExpressionNode::{And, Constant, Literal, Not, Or};

impl<TLiteral: Debug + Clone + Eq + Hash> SemanticEq<TLiteral> for Expression<TLiteral> {
    fn semantic_eq(&self, other: &Self) -> bool {
        let self_literals = self.gather_literals();
        let other_literals = other.gather_literals();
        let literals_union = HashSet::from_iter(self_literals.union(&other_literals).cloned());

        let all_options = Self::generate_power_set(literals_union);

        all_options.into_iter().all(|literal_settings| {
            self.evaluate(&literal_settings) == other.evaluate(&literal_settings)
        })
    }
}

impl<TLiteral: Debug + Clone + Eq + Hash> GatherLiterals<TLiteral> for Expression<TLiteral> {
    fn gather_literals_rec(&self, current: &mut HashSet<TLiteral>) {
        match self.node() {
            Literal(l) => {
                current.insert(l.clone());
            }
            Constant(_) => (),
            Not(e) => e.gather_literals_rec(current),
            And(es) | Or(es) => {
                for e in es {
                    e.gather_literals_rec(current);
                }
            }
        }
    }
}

impl<TLiteral: Debug + Clone + Eq + Hash> PowerSet<TLiteral> for Expression<TLiteral> {
    fn generate_power_set_rec(
        mut initial: Vec<TLiteral>,
        mut current: HashMap<TLiteral, bool>,
        result: &mut Vec<HashMap<TLiteral, bool>>,
    ) {
        if let Some(literal) = initial.pop() {
            current.insert(literal.clone(), true);
            Self::generate_power_set_rec(initial.clone(), current.clone(), result);

            current.insert(literal, false);
            Self::generate_power_set_rec(initial, current.clone(), result);
        } else {
            result.push(current);
        }
    }
}

impl<TLiteral: Debug + Clone + Eq + Hash> Evaluate<TLiteral> for Expression<TLiteral> {
    fn evaluate(&self, literal_values: &HashMap<TLiteral, bool>) -> bool {
        match self.node() {
            Literal(t) => *literal_values.get(t).unwrap_or(&false),
            Constant(value) => *value,
            And(values) => values.iter().all(|e| e.evaluate(literal_values)),
            Or(values) => values.iter().any(|e| e.evaluate(literal_values)),
            Not(x) => !x.evaluate(literal_values),
        }
    }

    fn evaluate_with_err(
        &self,
        literal_values: &HashMap<TLiteral, bool>,
    ) -> Result<bool, TLiteral> {
        match self.node() {
            Literal(t) => match literal_values.get(t) {
                None => Err(t.clone()),
                Some(valuation) => Ok(*valuation),
            },
            Constant(value) => Ok(*value),
            Not(inner) => inner.evaluate_with_err(literal_values).map(|value| !value),
            And(expressions) => expressions
                .iter()
                .map(|e| e.evaluate_with_err(literal_values))
                .fold_ok(true, BitAnd::bitand),
            Or(expressions) => expressions
                .iter()
                .map(|e| e.evaluate_with_err(literal_values))
                .fold_ok(false, BitOr::bitor),
        }
    }
}

impl Parse for Expression<String> {
    fn from_str(input: &str) -> Result<Self, ParseError> {
        let tokens = tokenize(input)?;
        let parsed = parse_tokens(&tokens)?;

        Ok(parsed)
    }
}

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

#[cfg(test)]
mod tests {
    use crate::expressions::Expression;
    use crate::expressions::ExpressionNode::{And, Not, Or};
    use std::fmt::Debug;
    use std::hash::Hash;

    /*
       The following traits are only implemented in test builds because it is not yet
       clear whether we should provide them as part of the stable Rust API (overloading
       operators for non-copy types often isn't as useful as it might initially seem).

       However, in tests, we can use them to simplify expression construction.
    */

    impl<T: Debug + Clone + Eq + Hash> std::ops::BitAnd<Expression<T>> for Expression<T> {
        type Output = Expression<T>;

        fn bitand(self, rhs: Expression<T>) -> Self::Output {
            let mut es = Vec::new();
            match (self.node(), rhs.node()) {
                (And(es1), And(es2)) => {
                    es.extend(es1.iter().cloned());
                    es.extend(es2.iter().cloned());
                }
                (And(es1), _other) => {
                    es.extend(es1.iter().cloned());
                    es.push(rhs);
                }
                (_other, And(es2)) => {
                    es.push(self);
                    es.extend(es2.iter().cloned());
                }
                _ => {
                    es.push(self);
                    es.push(rhs);
                }
            }

            And(es).into()
        }
    }

    impl<T: Debug + Clone + Eq + Hash> std::ops::BitOr<Expression<T>> for Expression<T> {
        type Output = Expression<T>;

        fn bitor(self, rhs: Expression<T>) -> Self::Output {
            let mut es = Vec::new();
            match (self.node(), rhs.node()) {
                (Or(es1), Or(es2)) => {
                    es.extend(es1.iter().cloned());
                    es.extend(es2.iter().cloned());
                }
                (Or(es1), _other) => {
                    es.extend(es1.iter().cloned());
                    es.push(rhs);
                }
                (_other, Or(es2)) => {
                    es.push(self);
                    es.extend(es2.iter().cloned());
                }
                _ => {
                    es.push(self);
                    es.push(rhs);
                }
            }

            Or(es).into()
        }
    }

    impl<T: Debug + Clone + Eq + Hash> std::ops::Not for Expression<T> {
        type Output = Expression<T>;

        fn not(self) -> Self::Output {
            Not(self).into()
        }
    }
}
