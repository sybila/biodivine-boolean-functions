use std::ops::{BitOr, Not};

pub trait Implication<Rhs = Self>: BitOr<Rhs> + Not {
    type Output;

    fn imply(self, rhs: Rhs) -> <Self as Implication<Rhs>>::Output;
}
