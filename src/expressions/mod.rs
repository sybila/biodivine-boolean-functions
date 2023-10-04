use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

use Expression::{And, Constant, Literal, Not, Or};

pub mod traits;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Expression<T>
where
    T: Debug + Clone + Eq + Hash,
{
    Literal(T),
    Constant(bool),
    Not(Box<Expression<T>>),
    And(Vec<Box<Expression<T>>>),
    Or(Vec<Box<Expression<T>>>),
}

impl<T: Debug + Clone + Eq + Hash> Expression<T> {
    pub fn is_literal(&self) -> bool {
        match self {
            &Literal(_) => true,
            Not(maybe_literal) => maybe_literal.is_literal(),
            _ => false,
        }
    }

    pub fn is_constant(&self) -> bool {
        matches!(self, Constant(_))
    }

    pub fn is_not(&self) -> bool {
        matches!(self, Not(_))
    }

    pub fn is_and(&self) -> bool {
        matches!(self, And(_))
    }

    pub fn is_or(&self) -> bool {
        matches!(self, Or(_))
    }

    pub fn negate(e: Expression<T>) -> Expression<T> {
        Not(Box::new(e))
    }

    pub fn binary_and(e1: Expression<T>, e2: Expression<T>) -> Expression<T> {
        And(vec![Box::new(e1), Box::new(e2)])
    }

    pub fn n_ary_and(es: Vec<Expression<T>>) -> Expression<T> {
        And(es.into_iter().map(Box::new).collect())
    }

    pub fn binary_or(e1: Expression<T>, e2: Expression<T>) -> Expression<T> {
        Or(vec![Box::new(e1), Box::new(e2)])
    }

    pub fn n_ary_or(es: Vec<Expression<T>>) -> Expression<T> {
        Or(es.into_iter().map(Box::new).collect())
    }

    // toNNF (Not (Bin And     l r)) = Bin Or  (toNNF (Not l)) (toNNF (Not r))  -- ¬(ϕ ∧ ψ) = ¬ϕ ∨ ¬ψ
    // toNNF (Not (Bin Or      l r)) = Bin And (toNNF (Not l)) (toNNF (Not r))  -- ¬(ϕ ∨ ψ) = ¬ϕ ∧ ¬ψ
    // toNNF (Bin op      l r)       = Bin op  (toNNF l)       (toNNF r)
    // toNNF (Not (Not exp))         = toNNF exp
    // toNNF (Not exp)               = Not (toNNF exp)
    // toNNF leaf                    = leaf
    pub fn to_nnf(self) -> Self {
        match self {
            Not(inner) => match *inner {
                And(expressions) => Or(expressions
                    .into_iter()
                    .map(|e| Box::new(Not(e).to_nnf()))
                    .collect()),
                Or(expressions) => And(expressions
                    .into_iter()
                    .map(|e| Box::new(Not(e).to_nnf()))
                    .collect()),
                Not(expression) => expression.to_nnf(),
                expression => Expression::negate(expression.to_nnf()),
            },
            And(expressions) => And(expressions
                .into_iter()
                .map(|e| Box::new(e.to_nnf()))
                .collect()),
            Or(expressions) => Or(expressions
                .into_iter()
                .map(|e| Box::new(e.to_nnf()))
                .collect()),
            leaf => leaf,
        }
    }

    // let rec cnfc (phi: formula_wi) : formula_wi
    // = match phi with
    // | FOr_wi phi1 phi2 → distr (cnfc phi1) (cnfc phi2)
    // | FAnd_wi phi1 phi2 → FAnd_wi (cnfc phi1) (cnfc phi2)
    // | phi → phi
    // end
    pub fn to_cnf(self) -> Self {
        let nnf = self.to_nnf();

        match nnf {
            Or(expressions) => expressions
                .into_iter()
                .map(|e| e.to_cnf())
                // .rev()
                .reduce(|acc, e| Expression::distribute(acc, e))
                .unwrap(),
            And(expressions) => And(expressions
                .into_iter()
                .map(|e| Box::new(e.to_cnf()))
                .collect()),
            expression => expression,
        }
    }

    // let rec distr (phi1 phi2: formula_wi) : formula_wi
    // = match phi1, phi2 with
    // | FAnd_wi and1 and2, phi2 → FAnd_wi (distr and1 phi2) (distr and2 phi2)
    // | phi1, FAnd_wi and1 and2 → FAnd_wi (distr phi1 and1) (distr phi1 and2)
    // | phi1,phi2 → FOr_wi phi1 phi2
    // end
    fn distribute(first: Self, other: Self) -> Self {
        match (first, other) {
            (And(expressions), other) => And(expressions
                .into_iter()
                .map(|e| Box::new(Expression::distribute(*e, other.clone())))
                .collect()),
            (other, And(expressions)) => And(expressions
                .into_iter()
                .map(|e| Box::new(Expression::distribute(other.clone(), *e)))
                .collect()),
            (expression1, expression2) => Expression::binary_or(expression1, expression2),
        }
    }
}
