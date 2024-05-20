use std::fmt::Debug;
use std::ops::Not;

use crate::bdd::Bdd;

impl<T: Debug + Clone + Ord> Not for &Bdd<T> {
    type Output = Bdd<T>;

    fn not(self) -> Self::Output {
        Bdd::new(self.bdd.not(), self.inputs.clone())
    }
}

impl<T: Debug + Clone + Ord> Not for Bdd<T> {
    type Output = Bdd<T>;

    fn not(self) -> Self::Output {
        (&self).not()
    }
}
