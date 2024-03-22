use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Display, Error, Formatter};
use std::hash::Hash;
use std::ops::{BitAnd, BitOr};

use itertools::Itertools;

use crate::expressions::Expression::{self, And, Constant, Literal, Not, Or};
use crate::parser::{parse_tokens, tokenize, ParseError};
use crate::traits::{Evaluate, GatherLiterals, Parse, PowerSet, SemanticEq};

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
    fn gather_literals_rec(&self, mut current: HashSet<TLiteral>) -> HashSet<TLiteral> {
        match self {
            Literal(l) => {
                current.insert(l.clone());
                current
            }
            Constant(_) => current,
            Not(e) => e.gather_literals_rec(current),
            And(es) | Or(es) => {
                let v = es
                    .iter()
                    .map(|e| e.gather_literals_rec(HashSet::new()))
                    .reduce(|mut acc, set| {
                        acc.extend(set);
                        acc
                    });
                current.extend(v.unwrap());
                current
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
        match self {
            Literal(ref t) => *literal_values.get(t).unwrap_or(&false),
            Constant(ref value) => *value,
            And(ref values) => values.iter().all(|e| e.evaluate(literal_values)),
            Or(ref values) => values.iter().any(|e| e.evaluate(literal_values)),
            Not(ref x) => !x.evaluate(literal_values),
        }
    }

    fn evaluate_with_err(
        &self,
        literal_values: &HashMap<TLiteral, bool>,
    ) -> Result<bool, TLiteral> {
        match self {
            Literal(ref t) => match literal_values.get(t) {
                None => Err(t.clone()),
                Some(valuation) => Ok(*valuation),
            },
            Constant(ref value) => Ok(*value),
            Not(ref inner) => inner.evaluate_with_err(literal_values).map(|value| !value),
            And(ref expressions) => expressions
                .iter()
                .map(|e| e.evaluate_with_err(literal_values))
                .fold_ok(true, BitAnd::bitand),
            Or(ref expressions) => expressions
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
        match self {
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
