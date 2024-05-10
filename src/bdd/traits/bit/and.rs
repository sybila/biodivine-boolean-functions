use crate::bdd::{extend_bdd_variables, Bdd};
use std::fmt::Debug;
use std::ops::BitAnd;

impl<TLiteral: Debug + Clone + Eq + Ord + 'static> BitAnd for Bdd<TLiteral> {
    type Output = Bdd<TLiteral>;

    fn bitand(self, rhs: Self) -> Self::Output {
        if self.inputs == rhs.inputs {
            Bdd {
                inputs: self.inputs.clone(),
                bdd: self.bdd.and(&rhs.bdd),
            }
        } else {
            let mut common_inputs = self.inputs.clone();
            for other in &rhs.inputs {
                if !common_inputs.contains(other) {
                    common_inputs.push(other.clone());
                }
            }
            common_inputs.sort();

            let self_lifted = extend_bdd_variables(&self, &common_inputs);
            let rhs_lifted = extend_bdd_variables(&rhs, &common_inputs);

            Bdd {
                inputs: common_inputs,
                bdd: self_lifted.bdd.and(&rhs_lifted.bdd),
            }
        }
    }
}
