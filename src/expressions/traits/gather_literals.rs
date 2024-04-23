use crate::expressions::Expression;
use crate::expressions::ExpressionNode::{And, Constant, Literal, Not, Or};
use crate::traits::GatherLiterals;
use std::collections::BTreeSet;
use std::fmt::Debug;

impl<TLiteral: Debug + Clone + Eq + Ord> GatherLiterals<TLiteral> for Expression<TLiteral> {
    fn gather_literals_rec(&self, current: &mut BTreeSet<TLiteral>) {
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
