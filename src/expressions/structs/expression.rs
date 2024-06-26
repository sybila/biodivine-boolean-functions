use crate::expressions::structs::ExpressionNode;
use crate::expressions::structs::ExpressionNode::{And, Constant, Literal, Not, Or};
use std::collections::BTreeMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Expression<T>(Arc<ExpressionNode<T>>)
where
    T: Debug + Clone + Eq + Ord;

impl<T: Debug + Clone + Eq + Ord> From<ExpressionNode<T>> for Expression<T> {
    fn from(value: ExpressionNode<T>) -> Self {
        Expression(Arc::new(value))
    }
}

impl<T: Debug + Clone + Eq + Ord> Expression<T> {
    pub fn node(&self) -> &ExpressionNode<T> {
        self.0.as_ref()
    }

    pub fn is_literal(&self) -> bool {
        match self.node() {
            Literal(_) => true,
            Not(maybe_literal) => {
                matches!(maybe_literal.node(), Literal(..))
            }
            _ => false,
        }
    }

    pub fn is_constant(&self) -> bool {
        matches!(self.node(), Constant(_))
    }

    pub fn is_not(&self) -> bool {
        matches!(self.node(), Not(_))
    }

    pub fn is_and(&self) -> bool {
        matches!(self.node(), And(_))
    }

    pub fn is_or(&self) -> bool {
        matches!(self.node(), Or(_))
    }

    pub fn negate(e: &Expression<T>) -> Expression<T> {
        Not(e.clone()).into()
    }

    pub fn binary_and(e1: &Expression<T>, e2: &Expression<T>) -> Expression<T> {
        And(vec![e1.clone(), e2.clone()]).into()
    }

    pub fn n_ary_and(es: &[Expression<T>]) -> Expression<T> {
        And(es.to_vec()).into()
    }

    pub fn binary_or(e1: &Expression<T>, e2: &Expression<T>) -> Expression<T> {
        Or(vec![e1.clone(), e2.clone()]).into()
    }

    pub fn n_ary_or(es: &[Expression<T>]) -> Expression<T> {
        Or(es.to_vec()).into()
    }

    // toNNF (Not (Bin And     l r)) = Bin Or  (toNNF (Not l)) (toNNF (Not r))  -- ¬(ϕ ∧ ψ) = ¬ϕ ∨ ¬ψ
    // toNNF (Not (Bin Or      l r)) = Bin And (toNNF (Not l)) (toNNF (Not r))  -- ¬(ϕ ∨ ψ) = ¬ϕ ∧ ¬ψ
    // toNNF (Bin op      l r)       = Bin op  (toNNF l)       (toNNF r)
    // toNNF (Not (Not exp))         = toNNF exp
    // toNNF (Not exp)               = Not (toNNF exp)
    // toNNF leaf                    = leaf
    pub fn to_nnf(&self) -> Self {
        match self.node() {
            Not(inner) => match inner.node() {
                And(es) => Or(es.iter().map(|e| Expression::negate(e).to_nnf()).collect()).into(),
                Or(es) => And(es.iter().map(|e| Expression::negate(e).to_nnf()).collect()).into(),
                Not(e) => e.to_nnf(),
                _leaf => self.clone(), // TODO: Should we propagate negation to constants?
            },
            And(es) => And(es.iter().map(|e| e.to_nnf()).collect()).into(),
            Or(es) => Or(es.iter().map(|e| e.to_nnf()).collect()).into(),
            _leaf => self.clone(),
        }
    }

    pub fn is_nnf(&self) -> bool {
        match self.node() {
            Literal(_) => true,
            Constant(_) => false,
            Not(e) => matches!(e.node(), Literal(..)),
            And(es) | Or(es) => es.iter().all(|e| e.is_nnf()),
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

        match nnf.node() {
            Or(es) => es
                .iter()
                .map(|e| e.to_cnf())
                .reduce(|acc, e| Expression::distribute_cnf(&acc, &e))
                .unwrap(),
            And(es) => And(es.iter().map(|e| e.to_cnf()).collect()).into(),
            _other => nnf,
        }
    }

    fn distribute_cnf(first: &Self, second: &Self) -> Self {
        match (first.node(), second.node()) {
            (And(es), _) => {
                let es = es
                    .iter()
                    .map(|e| Expression::distribute_cnf(e, second))
                    .collect();
                And(es).into()
            }
            (_, And(es)) => {
                let es = es
                    .iter()
                    .map(|e| Expression::distribute_cnf(first, e))
                    .collect();
                And(es).into()
            }
            (_e1, _e2) => Expression::binary_or(first, second),
        }
    }

    pub fn is_cnf(&self) -> bool {
        match self.node() {
            Literal(_) => true,
            Constant(_) => false,
            Not(inner) => matches!(inner.node(), Literal(_)),
            And(es) => es.iter().all(|e| e.is_cnf()),
            Or(es) => !es.iter().any(|e| e.is_and()) && es.iter().all(|e| e.is_cnf()),
        }
    }

    pub fn to_dnf(&self) -> Self {
        let nnf = self.to_nnf();

        match nnf.node() {
            And(es) => es
                .iter()
                .map(|e| e.to_dnf())
                .reduce(|acc, e| Expression::distribute_dnf(&acc, &e))
                .unwrap(),
            Or(es) => Or(es.iter().map(|e| e.to_dnf()).collect()).into(),
            _other => nnf,
        }
    }

    fn distribute_dnf(first: &Self, second: &Self) -> Self {
        match (first.node(), second.node()) {
            (Or(es), _) => {
                let es = es
                    .iter()
                    .map(|e| Expression::distribute_dnf(e, second))
                    .collect();
                Or(es).into()
            }
            (_, Or(es)) => {
                let es = es
                    .iter()
                    .map(|e| Expression::distribute_dnf(first, e))
                    .collect();
                Or(es).into()
            }
            (_e1, _e2) => Expression::binary_and(first, second),
        }
    }

    pub fn is_dnf(&self) -> bool {
        match self.node() {
            Literal(_) => true,
            Constant(_) => false,
            Not(inner) => matches!(inner.node(), Literal(_)),
            Or(es) => es.iter().all(|e| e.is_dnf()),
            And(es) => !es.iter().any(|e| e.is_or()) && es.iter().all(|e| e.is_dnf()),
        }
    }

    pub fn rename_literals(&self, mapping: &BTreeMap<T, T>) -> Self {
        match self.node() {
            Literal(name) => Literal(mapping.get(name).unwrap_or(name).clone()),
            Constant(value) => Constant(*value),
            Not(inner) => Not(inner.rename_literals(mapping)),
            And(expressions) => And(expressions
                .iter()
                .map(|e| e.rename_literals(mapping))
                .collect()),
            Or(expressions) => Or(expressions
                .iter()
                .map(|e| e.rename_literals(mapping))
                .collect()),
        }
        .into()
    }
}

#[cfg(test)]
pub mod tests {
    use std::collections::BTreeMap;

    use crate::traits::SemanticEq;

    use crate::expressions::structs::expression::Expression;
    use crate::expressions::{bool, var, vars};

    #[test]
    fn test_literals() {
        let x = var("a");
        let y = Expression::negate(&x);
        let z = Expression::negate(&y);
        let not_it = Expression::binary_and(&x, &x);
        assert!(x.is_literal());
        assert!(y.is_literal());
        assert!(!z.is_literal());
        assert!(!not_it.is_literal());
    }

    #[test]
    fn test_to_nnf_1() {
        // (Not notA) ∨ (Not (notB ∨ vC)), vA ∨ (vB ∧ notC)
        let input = !!var("a") | !(!var("b") | var("c"));
        let expected = var("a") | (var("b") & !var("c"));
        let actual = input.to_nnf();

        assert!(expected.semantic_eq(&actual));
        assert!(actual.is_nnf());
    }

    #[test]
    fn test_to_nnf_2() {
        // Not (vA ∨ (notD ∨ Not (notA ∨ Not notB))), notA ∧ (vD ∧ (notA ∨ vB))
        let input = !(var("a") | !var("d") | !(!var("a") | !!var("b")));
        let expected = !var("a") & var("d") & (!var("a") | var("b"));
        let actual = input.to_nnf();

        assert!(expected.semantic_eq(&actual));
        assert!(actual.is_nnf());
    }

    #[test]
    fn test_to_nnf_3() {
        // Not (notA ∨ vB) ∨ Not (vB ∧ notC), (vA ∧ notB) ∨ (notB ∨ vC)
        let input = !(!var("a") | var("b")) | !(var("b") & !var("c"));
        let expected = (var("a") & !var("b")) | !var("b") | var("c");
        let actual = input.to_nnf();

        assert!(expected.semantic_eq(&actual));
        assert!(actual.is_nnf());
    }

    #[test]
    fn distribute_basic_and_right() {
        let input_left = var("a");
        let input_right = var("b") & var("c");

        let expected = (var("a") | var("b")) & (var("a") | var("c"));
        let actual = Expression::distribute_cnf(&input_left, &input_right);

        assert!(expected.semantic_eq(&actual));
    }

    #[test]
    fn distribute_basic_and_left() {
        let input_left = var("b") & var("c");
        let input_right = var("a");

        let expected = (var("b") | var("a")) & (var("c") | var("a"));
        let actual = Expression::distribute_cnf(&input_left, &input_right);

        assert!(expected.semantic_eq(&actual));
    }

    #[test]
    fn to_cnf_basic() {
        let input = var("a") | (var("b") & var("c"));
        let expected = (var("a") | var("b")) & (var("a") | var("c"));
        let actual = input.to_cnf();

        assert!(expected.semantic_eq(&actual));
        assert!(actual.is_cnf());
    }

    #[test]
    fn to_cnf_n_ary() {
        let input = var("a") | var("b") | (var("c") & var("d"));
        let expected = (var("a") | var("b") | var("c")) & (var("a") | var("b") | var("d"));
        let actual = input.to_cnf();

        assert!(expected.semantic_eq(&actual));
        assert!(actual.is_cnf());
    }

    #[test]
    fn to_cnf_n_ary_2() {
        let c = Expression::n_ary_and(&vars(["c1", "c2", "c3", "c4", "c5"]));
        let input = var("e1") | var("e2") | var("e3") | var("e4") | var("e5") | c;

        let expected = Expression::n_ary_and(&[
            Expression::n_ary_or(&vars(["e1", "e2", "e3", "e4", "e5", "c1"])),
            Expression::n_ary_or(&vars(["e1", "e2", "e3", "e4", "e5", "c2"])),
            Expression::n_ary_or(&vars(["e1", "e2", "e3", "e4", "e5", "c3"])),
            Expression::n_ary_or(&vars(["e1", "e2", "e3", "e4", "e5", "c4"])),
            Expression::n_ary_or(&vars(["e1", "e2", "e3", "e4", "e5", "c5"])),
        ]);
        let actual = input.to_cnf();

        assert!(expected.semantic_eq(&actual));
        assert!(actual.is_cnf());
    }

    #[test]
    fn to_dnf_basic() {
        let input = (var("a") | var("b")) & (var("b") | var("c")) & (var("a") | var("c"));

        let actual = input.to_dnf();
        let expected = (var("a") & var("b")) | (var("b") & var("c")) | (var("a") & var("c"));

        assert!(expected.semantic_eq(&actual));
        assert!(actual.is_dnf());
        println!("{actual}")
    }

    #[test]
    fn to_dnf_advanced() {
        let input =
            (var("A") | (var("B") & (var("C") | var("D")))) & (var("E") | (var("F") & !var("G")));

        let actual = input.to_dnf();
        let expected = (var("E") & var("A"))
            | (var("E") & var("B") & var("C"))
            | (var("E") & var("B") & var("D"))
            | (var("F") & !var("G") & var("A"))
            | (var("F") & !var("G") & var("B") & var("C"))
            | (var("F") & !var("G") & var("B") & var("D"));

        assert!(expected.semantic_eq(&actual));
        assert!(actual.is_dnf());
    }

    #[test]
    fn is_cnf_levels() {
        // We intentionally don't use the built-in operators because they would "level" the expression.
        let x = var("x");
        let y = var("y");
        let nested = Expression::binary_and(
            &Expression::binary_and(
                &Expression::binary_or(&x, &y),
                &Expression::binary_or(&x, &y),
            ),
            &Expression::binary_or(&x, &Expression::negate(&y)),
        );

        let leveled = Expression::n_ary_and(&[
            Expression::binary_or(&x, &y),
            Expression::binary_or(&x, &y),
            Expression::binary_or(&x, &Expression::negate(&y)),
        ]);

        assert!(nested.semantic_eq(&leveled));

        assert!(nested.is_cnf());
        assert!(leveled.is_cnf());
    }

    #[test]
    fn is_not_cnf() {
        assert!(!bool(true).is_cnf());

        // We intentionally don't use the built-in operators because they would "level" the expression.
        let x = var("x");
        let y = var("y");

        let nested = Expression::binary_or(
            &Expression::binary_or(
                &Expression::binary_and(&x, &y),
                &Expression::binary_and(&x, &y),
            ),
            &Expression::binary_and(&x, &Expression::negate(&y)),
        );
        assert!(!nested.is_cnf());

        let leveled = Expression::n_ary_or(&[
            Expression::binary_and(&x, &y),
            Expression::binary_and(&x, &y),
            Expression::binary_and(&x, &Expression::negate(&y)),
        ]);
        assert!(!leveled.is_cnf());
    }

    #[test]
    fn test_rename_literals_ok() {
        let pairs = [("a", "1"), ("b", "2"), ("c", "3"), ("d", "4"), ("e", "5")];
        let mapping =
            BTreeMap::from_iter(pairs.iter().map(|(x, y)| (x.to_string(), y.to_string())));

        let input = var("a") | var("b") | (var("c") & var("d")) | bool(true) | !var("a");
        let expected = var("1") | var("2") | (var("3") & var("4")) | bool(true) | !var("1");
        let actual = input.rename_literals(&mapping);

        assert_eq!(actual, expected)
    }
}
