use crate::bdd::Bdd;
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
            let (self_lifted, rhs_lifted, common_inputs) = self.union_and_extend(&rhs);

            Bdd {
                inputs: common_inputs,
                bdd: self_lifted.bdd.and(&rhs_lifted.bdd),
            }
        }
    }
}
