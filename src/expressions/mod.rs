use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::Deref;
use std::sync::Arc;

use Expression::{And, Constant, Literal, Not, Or};

pub mod traits;

/// Immutable struct representing a boolean expression.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Expression<T>
where
    T: Debug + Clone + Eq + Hash,
{
    Literal(T),
    Constant(bool),
    Not(Arc<Expression<T>>),
    And(Vec<Arc<Expression<T>>>),
    Or(Vec<Arc<Expression<T>>>),
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
        Not(Arc::new(e))
    }

    pub fn binary_and(e1: Expression<T>, e2: Expression<T>) -> Expression<T> {
        And(vec![Arc::new(e1), Arc::new(e2)])
    }

    pub fn n_ary_and(es: Vec<Expression<T>>) -> Expression<T> {
        And(es.into_iter().map(Arc::new).collect())
    }

    pub fn binary_or(e1: Expression<T>, e2: Expression<T>) -> Expression<T> {
        Or(vec![Arc::new(e1), Arc::new(e2)])
    }

    pub fn n_ary_or(es: Vec<Expression<T>>) -> Expression<T> {
        Or(es.into_iter().map(Arc::new).collect())
    }

    // toNNF (Not (Bin And     l r)) = Bin Or  (toNNF (Not l)) (toNNF (Not r))  -- ¬(ϕ ∧ ψ) = ¬ϕ ∨ ¬ψ
    // toNNF (Not (Bin Or      l r)) = Bin And (toNNF (Not l)) (toNNF (Not r))  -- ¬(ϕ ∨ ψ) = ¬ϕ ∧ ¬ψ
    // toNNF (Bin op      l r)       = Bin op  (toNNF l)       (toNNF r)
    // toNNF (Not (Not exp))         = toNNF exp
    // toNNF (Not exp)               = Not (toNNF exp)
    // toNNF leaf                    = leaf
    pub fn to_nnf(&self) -> Self {
        match self {
            Not(inner) => match inner.as_ref() {
                And(expressions) => Or(expressions
                    .iter()
                    .map(|e| Arc::new(Not(e.clone()).to_nnf()))
                    .collect()),
                Or(expressions) => And(expressions
                    .iter()
                    .map(|e| Arc::new(Not(e.clone()).to_nnf()))
                    .collect()),
                Not(expression) => expression.to_nnf(),
                expression => Expression::negate(expression.to_nnf()),
            },
            And(expressions) => And(expressions.iter().map(|e| Arc::new(e.to_nnf())).collect()),
            Or(expressions) => Or(expressions.iter().map(|e| Arc::new(e.to_nnf())).collect()),
            leaf => leaf.clone(),
        }
    }

    // let rec cnfc (phi: formula_wi) : formula_wi
    // = match phi with
    // | FOr_wi phi1 phi2 → distr (cnfc phi1) (cnfc phi2)
    // | FAnd_wi phi1 phi2 → FAnd_wi (cnfc phi1) (cnfc phi2)
    // | phi → phi
    // end
    pub fn to_cnf(&self) -> Self {
        let nnf = self.to_nnf();

        match nnf {
            Or(expressions) => expressions
                .into_iter()
                .map(|e| e.to_cnf())
                .reduce(|acc, e| Expression::distribute(acc, e))
                .unwrap(),
            And(expressions) => And(expressions
                .into_iter()
                .map(|e| Arc::new(e.to_cnf()))
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
                .map(|e| Arc::new(Expression::distribute(e.as_ref().clone(), other.clone())))
                .collect()),
            (other, And(expressions)) => And(expressions
                .into_iter()
                .map(|e| Arc::new(Expression::distribute(other.clone(), e.as_ref().clone())))
                .collect()),
            (expression1, expression2) => Expression::binary_or(expression1, expression2),
        }
    }

    pub fn is_cnf(&self) -> bool {
        match self {
            Literal(_) => true,
            Constant(_) => false,
            Not(ref inner) => matches!(inner.deref(), Literal(_)),
            And(es) => es.iter().all(|e| e.is_cnf()),
            Or(es) => !es.iter().any(|e| e.is_and()) && es.iter().all(|e| e.is_cnf()),
        }
    }

    pub fn rename_literals(&self, mapping: &HashMap<T, T>) -> Self {
        match self {
            Literal(name) => {
                let new = mapping.get(name).unwrap_or(name);
                Literal(new.clone())
            }
            Constant(value) => Constant(*value),
            Not(inner) => Expression::negate(inner.as_ref().rename_literals(mapping)),
            And(expressions) => And(expressions
                .iter()
                .map(|e| Arc::new(e.as_ref().rename_literals(mapping)))
                .collect()),
            Or(expressions) => Or(expressions
                .iter()
                .map(|e| Arc::new(e.as_ref().rename_literals(mapping)))
                .collect()),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    #[allow(unused_imports)] // false positive, no idea why
    use crate::expressions::Expression::{self, Constant, Literal};
    #[allow(unused_imports)] // false positive, probably because of usage in macros?
    use crate::traits::SemanticEq;

    #[test]
    fn test_to_nnf_1() {
        // (Not notA) ∨ (Not (notB ∨ vC)), vA ∨ (vB ∧ notC)
        let input = Expression::binary_or(
            Expression::negate(Expression::negate(Literal("a"))),
            Expression::negate(Expression::binary_or(
                Expression::negate(Literal("b")),
                Literal("c"),
            )),
        );

        let expected = Expression::binary_or(
            Literal("a"),
            Expression::binary_and(Literal("b"), Expression::negate(Literal("c"))),
        );
        let actual = input.to_nnf();

        assert!(expected.semantic_eq(&actual));
    }

    #[test]
    fn test_to_nnf_2() {
        // Not (vA ∨ (notD ∨ Not (notA ∨ Not notB))), notA ∧ (vD ∧ (notA ∨ vB))
        let input = Expression::negate(Expression::binary_or(
            Literal("a"),
            Expression::binary_or(
                Expression::negate(Literal("d")),
                Expression::negate(Expression::binary_or(
                    Expression::negate(Literal("a")),
                    Expression::negate(Expression::negate(Literal("b"))),
                )),
            ),
        ));

        let expected = Expression::binary_and(
            Expression::negate(Literal("a")),
            Expression::binary_and(
                Literal("d"),
                Expression::binary_or(Expression::negate(Literal("a")), Literal("b")),
            ),
        );
        let actual = input.to_nnf();

        assert!(expected.semantic_eq(&actual));
    }

    #[test]
    fn test_to_nnn_3() {
        // Not (notA ∨ vB) ∨ Not (vB ∧ notC), (vA ∧ notB) ∨ (notB ∨ vC)
        let input = Expression::binary_or(
            Expression::negate(Expression::binary_or(
                Expression::negate(Literal("a")),
                Literal("b"),
            )),
            Expression::negate(Expression::binary_and(
                Literal("b"),
                Expression::negate(Literal("c")),
            )),
        );

        let expected = Expression::binary_or(
            Expression::binary_and(Literal("a"), Expression::negate(Literal("b"))),
            Expression::binary_or(Expression::negate(Literal("b")), Literal("c")),
        );
        let actual = input.to_nnf();

        assert!(expected.semantic_eq(&actual));
    }

    #[test]
    fn distribute_basic() {
        let input_left = Literal("a");
        let input_right = Expression::binary_and(Literal("b"), Literal("c"));

        let expected = Expression::binary_and(
            Expression::binary_or(Literal("a"), Literal("b")),
            Expression::binary_or(Literal("a"), Literal("c")),
        );
        let actual = Expression::distribute(input_left, input_right);

        assert!(expected.semantic_eq(&actual));
    }

    #[test]
    fn to_cnf_basic() {
        let input = Expression::binary_or(
            Literal("a"),
            Expression::binary_and(Literal("b"), Literal("c")),
        );

        let expected = Expression::binary_and(
            Expression::binary_or(Literal("a"), Literal("b")),
            Expression::binary_or(Literal("a"), Literal("c")),
        );
        let actual = input.to_cnf();

        assert!(expected.semantic_eq(&actual));
        assert!(actual.is_cnf());
    }

    #[test]
    fn to_cnf_n_ary() {
        let input = Expression::n_ary_or(vec![
            Literal("a"),
            Literal("b"),
            Expression::binary_and(Literal("c"), Literal("d")),
        ]);

        let expected = Expression::binary_and(
            Expression::n_ary_or(vec![Literal("a"), Literal("b"), Literal("c")]),
            Expression::n_ary_or(vec![Literal("a"), Literal("b"), Literal("d")]),
        );
        let actual = input.to_cnf();

        assert!(expected.semantic_eq(&actual));
        assert!(actual.is_cnf());
    }

    #[test]
    fn to_cnf_n_ary_2() {
        let input = Expression::n_ary_or(vec![
            Literal("e1"),
            Literal("e2"),
            Literal("e3"),
            Literal("e4"),
            Literal("e5"),
            Expression::n_ary_and(vec![
                Literal("c1"),
                Literal("c2"),
                Literal("c3"),
                Literal("c4"),
                Literal("c5"),
            ]),
        ]);

        let expected = Expression::n_ary_and(vec![
            Expression::n_ary_or(vec![
                Literal("e1"),
                Literal("e2"),
                Literal("e3"),
                Literal("e4"),
                Literal("e5"),
                Literal("c1"),
            ]),
            Expression::n_ary_or(vec![
                Literal("e1"),
                Literal("e2"),
                Literal("e3"),
                Literal("e4"),
                Literal("e5"),
                Literal("c2"),
            ]),
            Expression::n_ary_or(vec![
                Literal("e1"),
                Literal("e2"),
                Literal("e3"),
                Literal("e4"),
                Literal("e5"),
                Literal("c3"),
            ]),
            Expression::n_ary_or(vec![
                Literal("e1"),
                Literal("e2"),
                Literal("e3"),
                Literal("e4"),
                Literal("e5"),
                Literal("c4"),
            ]),
            Expression::n_ary_or(vec![
                Literal("e1"),
                Literal("e2"),
                Literal("e3"),
                Literal("e4"),
                Literal("e5"),
                Literal("c5"),
            ]),
        ]);
        let actual = input.to_cnf();

        assert!(expected.semantic_eq(&actual));
        assert!(actual.is_cnf());
    }

    #[test]
    fn is_cnf_levels() {
        let nested = Expression::binary_and(
            Expression::binary_and(
                Expression::binary_or(Literal("x"), Literal("y")),
                Expression::binary_or(Literal("x"), Literal("y")),
            ),
            Expression::binary_or(Literal("x"), Literal("y")),
        );

        let leveled = Expression::n_ary_and(vec![
            Expression::binary_or(Literal("x"), Literal("y")),
            Expression::binary_or(Literal("x"), Literal("y")),
            Expression::binary_or(Literal("x"), Literal("y")),
        ]);

        assert!(nested.semantic_eq(&leveled));

        assert!(nested.is_cnf());
        assert!(leveled.is_cnf());
    }

    #[test]
    fn test_rename_literals_ok() {
        let mut mapping = HashMap::new();
        mapping.insert("a", "1");
        mapping.insert("b", "2");
        mapping.insert("c", "3");
        mapping.insert("d", "4");
        mapping.insert("e", "5");

        let input = Expression::n_ary_or(vec![
            Literal("a"),
            Literal("b"),
            Expression::binary_and(Literal("c"), Literal("d")),
            Constant(true),
            Expression::negate(Literal("a")),
        ]);

        let actual = input.rename_literals(&mapping);
        let expected = Expression::n_ary_or(vec![
            Literal("1"),
            Literal("2"),
            Expression::binary_and(Literal("3"), Literal("4")),
            Constant(true),
            Expression::negate(Literal("1")),
        ]);

        assert_eq!(actual, expected)
    }
}
