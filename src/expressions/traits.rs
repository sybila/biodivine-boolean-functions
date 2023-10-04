use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;

use crate::expressions::Expression;
use crate::traits::{GatherLiterals, PowerSet, SemanticEq};

impl<TLiteral: Debug + Clone + Eq + Hash> SemanticEq<TLiteral> for Expression<TLiteral> {
    fn semantic_eq(&self, other: &Self) -> bool {
        let self_literals = self.gather_literals();
        let other_literals = other.gather_literals();

        if self_literals != other_literals {
            return false;
        }

        let all_options = Self::generate_power_set(self_literals);

        all_options.into_iter().all(|literal_settings| {
            self.evaluate(&literal_settings) == other.evaluate(&literal_settings)
        })
    }
}

impl<TLiteral: Debug + Clone + Eq + Hash> GatherLiterals<TLiteral> for Expression<TLiteral> {
    fn gather_literals_rec(&self, mut current: HashSet<TLiteral>) -> HashSet<TLiteral> {
        use Expression::{And, Constant, Literal, Not, Or};

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
