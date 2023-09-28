use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

use Expression::*;

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
        match self {
            Constant(_) => true,
            _ => false,
        }
    }

    pub fn is_not(&self) -> bool {
        match self {
            Not(_) => true,
            _ => false,
        }
    }

    pub fn is_and(&self) -> bool {
        match self {
            And(_) => true,
            _ => false,
        }
    }

    pub fn is_or(&self) -> bool {
        match self {
            Or(_) => true,
            _ => false,
        }
    }

    pub fn not(e: Expression<T>) -> Expression<T> {
        Not(Box::new(e))
    }

    pub fn binary_and(e1: Expression<T>, e2: Expression<T>) -> Expression<T> {
        And(vec![Box::new(e1), Box::new(e2)])
    }

    pub fn n_ary_and(es: Vec<Expression<T>>) -> Expression<T> {
        And(es.into_iter().map(|e| Box::new(e)).collect())
    }

    pub fn binary_or(e1: Expression<T>, e2: Expression<T>) -> Expression<T> {
        Or(vec![Box::new(e1), Box::new(e2)])
    }

    pub fn n_ary_or(es: Vec<Expression<T>>) -> Expression<T> {
        Or(es.into_iter().map(|e| Box::new(e)).collect())
    }

    pub fn evaluate(&self, literal_values: &HashMap<T, bool>) -> bool {
        match self {
            Literal(ref t) => *literal_values.get(t).unwrap_or(&false),
            Constant(ref value) => *value,
            And(ref values) => values
                .iter()
                .fold(true, |acc, e| acc && e.evaluate(literal_values)),
            Or(ref values) => values
                .iter()
                .fold(false, |acc, e| acc || e.evaluate(literal_values)),
            Not(ref x) => !x.evaluate(literal_values),
        }
    }
}
