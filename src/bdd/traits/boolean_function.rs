use crate::bdd::iterators::{ImageIterator, SupportIterator};
use crate::bdd::Bdd;
use crate::iterators::DomainIterator;
use crate::traits::{BooleanFunction, BooleanPoint, BooleanValuation};
use biodivine_lib_bdd::Bdd as InnerBdd;
use biodivine_lib_bdd::BddVariable;
use num_bigint::BigUint;
use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::fmt::Debug;
use std::iter::{zip, Zip};

impl<T: Debug + Clone + Ord> BooleanFunction<T> for Bdd<T> {
    type DomainIterator = DomainIterator;
    type RangeIterator = ImageIterator;
    type RelationIterator = Zip<DomainIterator, ImageIterator>;
    type SupportIterator = SupportIterator;

    fn inputs(&self) -> BTreeSet<T> {
        self.inputs.iter().cloned().collect()
    }

    fn essential_inputs(&self) -> BTreeSet<T> {
        self.bdd
            .support_set()
            .into_iter()
            .map(|var| {
                // This unwrap is safe unless the BDD has variables that we don't know about.
                self.map_var_inner_to_outer(var).unwrap()
            })
            .collect()
    }

    fn degree(&self) -> usize {
        self.inputs.len()
    }

    fn essential_degree(&self) -> usize {
        self.bdd.support_set().len()
    }

    fn domain(&self) -> Self::DomainIterator {
        DomainIterator::from_count(self.inputs.len())
    }

    fn image(&self) -> Self::RangeIterator {
        // evaluate for each domain point
        // DomainIterator::new(self).map(|it| self.bdd.eval_in(&BddValuation::new(it)));
        ImageIterator::new(self.inputs.len(), &self.bdd)
    }

    fn relation(&self) -> Self::RelationIterator {
        // zip domain/range
        zip(self.domain(), self.image())
    }

    fn support(&self) -> Self::SupportIterator {
        SupportIterator::new(&self.bdd)
    }

    fn weight(&self) -> BigUint {
        self.bdd.exact_cardinality().to_biguint().unwrap()
    }

    fn restrict(&self, valuation: &BooleanValuation<T>) -> Self {
        let lib_bdd_valuation: Vec<(BddVariable, bool)> = valuation
            .iter()
            .filter_map(|(a, b)| self.map_var_outer_to_inner(a).map(|var| (var, *b)))
            .collect::<Vec<_>>();
        let new_bdd = Bdd::new(self.bdd.restrict(&lib_bdd_valuation), self.inputs.clone());

        self.restrict_and_prune_map(valuation, &new_bdd)
    }

    fn substitute(&self, mapping: &BTreeMap<T, Self>) -> Self {
        // Bdd.substitute exists, but assumes all BDDs share input variables (we need to extend)
        // and does not eliminate the substituted variable from inputs afterward (we need to prune).

        // Bdd.substitute currently assumes that the substituted functions does not depend on the
        // substituted variables. This will be solved in lib-bdd, we can just panic for now.
        if mapping
            .iter()
            .any(|(key, value_bdd)| value_bdd.inputs.contains(key))
        {
            panic!("Currently not allowed to have the substituted variable also appear in the substituting BDD value");
        }

        let mut extended_mapping = mapping.clone();
        let (mut self_lifted, _common_inputs) = self.union_and_extend_n_ary(&mut extended_mapping);

        for (k, v) in extended_mapping.iter() {
            self_lifted.bdd = self_lifted
                .bdd
                .substitute(self_lifted.map_var_outer_to_inner(k).unwrap(), &v.bdd)
        }

        self_lifted.restrict_and_prune_map(&extended_mapping, &self_lifted)
    }

    fn sat_point(&self) -> Option<BooleanPoint> {
        self.bdd.sat_witness().map(|it| it.vector())
    }

    fn existential_quantification(&self, variables: BTreeSet<T>) -> Self {
        let lib_bdd_variables = variables
            .iter()
            .filter_map(|it| self.map_var_outer_to_inner(it))
            .collect::<Vec<_>>();
        let new_bdd = Bdd::new(self.bdd.exists(&lib_bdd_variables), self.inputs.clone());

        self.restrict_and_prune_set(&variables, &new_bdd)
    }

    fn universal_quantification(&self, variables: BTreeSet<T>) -> Self {
        let lib_bdd_variables = variables
            .iter()
            .filter_map(|it| self.map_var_outer_to_inner(it))
            .collect::<Vec<_>>();
        let new_bdd = Bdd::new(self.bdd.for_all(&lib_bdd_variables), self.inputs.clone());

        self.restrict_and_prune_set(&variables, &new_bdd)
    }

    fn derivative(&self, variables: BTreeSet<T>) -> Self {
        let lib_bdd_variables = variables
            .iter()
            .filter_map(|it| self.map_var_outer_to_inner(it))
            .collect::<HashSet<_>>();
        let trigger = |var: BddVariable| lib_bdd_variables.contains(&var);

        let new_bdd = Bdd::new(
            InnerBdd::binary_op_nested(
                &self.bdd,
                &self.bdd,
                trigger,
                biodivine_lib_bdd::op_function::and,
                biodivine_lib_bdd::op_function::xor,
            ),
            self.inputs.clone(),
        );

        self.restrict_and_prune_set(&variables, &new_bdd)
    }

    fn is_equivalent(&self, other: &Self) -> bool {
        let (self_lifted, other_lifted, _common_inputs) = self.union_and_extend(other);

        self_lifted.bdd == other_lifted.bdd
    }

    fn is_implied_by(&self, other: &Self) -> bool {
        let (self_lifted, other_lifted, _common_inputs) = self.union_and_extend(other);

        other_lifted.bdd.imp(&self_lifted.bdd).is_true()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expressions::{bool, var};

    #[test]
    fn test_restrict_ok() {
        let input = Bdd::try_from((var("a") | var("b")) & var("c")).expect("Should not panic here");
        let valuation = BTreeMap::from_iter([("a".to_string(), false), ("c".to_string(), true)]);

        let expected =
            Bdd::try_from((bool(false) | var("b")) & bool(true)).expect("Should not panic here");
        let actual = input.restrict(&valuation);

        assert_eq!(expected, actual);
        assert!(expected.is_equivalent(&actual));

        assert_eq!(actual.degree(), 1);
    }

    #[test]
    fn test_restrict_too_many_variables_ok() {
        let input = Bdd::try_from((var("a") | var("b")) & var("c")).expect("Should not panic here");
        let valuation = BTreeMap::from_iter([
            ("a".to_string(), false),
            ("c".to_string(), true),
            ("notinthere".to_string(), true),
        ]);

        let expected =
            Bdd::try_from((bool(false) | var("b")) & bool(true)).expect("Should not panic here");
        let actual = input.restrict(&valuation);

        assert_eq!(expected, actual);
        assert!(expected.is_equivalent(&actual));
        assert_eq!(actual.degree(), 1);
    }

    #[test]
    fn test_restrict_no_variables_ok() {
        let input = Bdd::try_from((var("a") | var("b")) & var("c")).expect("Should not panic here");
        let valuation = BTreeMap::new();

        let expected = input.clone();
        let actual = input.restrict(&valuation);

        assert_eq!(expected, actual);
        assert!(expected.is_equivalent(&actual));
        assert_eq!(actual.degree(), expected.degree());
    }
}
