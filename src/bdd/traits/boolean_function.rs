use crate::bdd::iterators::{ImageIterator, SupportIterator};
use crate::bdd::utils::{extend_bdd_variables, prune_bdd_variables};
use crate::bdd::Bdd;
use crate::iterators::DomainIterator;
use crate::traits::{BooleanFunction, BooleanPoint, BooleanValuation};
use biodivine_lib_bdd::Bdd as InnerBdd;
use biodivine_lib_bdd::BddVariable;
use num_bigint::BigUint;
use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::fmt::Debug;
use std::iter::{zip, Zip};

impl<T: Debug + Clone + Ord + 'static> BooleanFunction<T> for Bdd<T> {
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
        let new_bdd = Bdd {
            inputs: self.inputs.clone(),
            bdd: self.bdd.restrict(&lib_bdd_valuation),
        };
        let restricted_inputs = self
            .inputs
            .iter()
            .filter(|var| !valuation.contains_key(var))
            .cloned()
            .collect::<Vec<_>>();
        prune_bdd_variables(&new_bdd, &restricted_inputs)
    }

    fn substitute(&self, _mapping: &BTreeMap<T, Self>) -> Self {
        // Bdd.substitute exists, but assumes all BDDs share input variables (we need to extend)
        // and does not eliminate the substituted variable from inputs afterward (we need to prune).

        // Bdd.substitute currently assumes that the substituted functions does not depend on the
        // substituted variables. This will be solved in lib-bdd, we can just panic for now.
        todo!()
    }

    fn sat_point(&self) -> Option<BooleanPoint> {
        self.bdd.sat_witness().map(|it| it.vector())
    }

    fn existential_quantification(&self, variables: BTreeSet<T>) -> Self {
        let lib_bdd_variables = variables
            .iter()
            .filter_map(|it| self.map_var_outer_to_inner(it))
            .collect::<Vec<_>>();
        let new_bdd = Bdd {
            inputs: self.inputs.clone(),
            bdd: self.bdd.exists(&lib_bdd_variables),
        };
        let restricted_inputs = self
            .inputs
            .iter()
            .filter(|var| !variables.contains(var))
            .cloned()
            .collect::<Vec<_>>();
        prune_bdd_variables(&new_bdd, &restricted_inputs)
    }

    fn universal_quantification(&self, variables: BTreeSet<T>) -> Self {
        let lib_bdd_variables = variables
            .iter()
            .filter_map(|it| self.map_var_outer_to_inner(it))
            .collect::<Vec<_>>();
        let new_bdd = Bdd {
            inputs: self.inputs.clone(),
            bdd: self.bdd.for_all(&lib_bdd_variables),
        };
        let restricted_inputs = self
            .inputs
            .iter()
            .filter(|var| !variables.contains(var))
            .cloned()
            .collect::<Vec<_>>();
        prune_bdd_variables(&new_bdd, &restricted_inputs)
    }

    fn derivative(&self, variables: BTreeSet<T>) -> Self {
        let lib_bdd_variables = variables
            .iter()
            .filter_map(|it| self.map_var_outer_to_inner(it))
            .collect::<HashSet<_>>();
        let trigger = |var: BddVariable| lib_bdd_variables.contains(&var);

        let new_bdd = Bdd {
            inputs: self.inputs.clone(),
            bdd: InnerBdd::binary_op_nested(
                &self.bdd,
                &self.bdd,
                trigger,
                biodivine_lib_bdd::op_function::and,
                biodivine_lib_bdd::op_function::xor,
            ),
        };

        let restricted_inputs = self
            .inputs
            .iter()
            .filter(|var| !variables.contains(var))
            .cloned()
            .collect::<Vec<_>>();
        prune_bdd_variables(&new_bdd, &restricted_inputs)
    }

    fn is_equivalent(&self, other: &Self) -> bool {
        let mut common_inputs = self.inputs.clone();
        for other in &other.inputs {
            if !common_inputs.contains(other) {
                common_inputs.push(other.clone());
            }
        }
        common_inputs.sort();

        let self_lifted = extend_bdd_variables(self, &common_inputs);
        let other_lifted = extend_bdd_variables(other, &common_inputs);
        self_lifted.bdd == other_lifted.bdd
    }

    fn is_implied_by(&self, other: &Self) -> bool {
        let mut common_inputs = self.inputs.clone();
        for other in &other.inputs {
            if !common_inputs.contains(other) {
                common_inputs.push(other.clone());
            }
        }
        common_inputs.sort();

        let self_lifted = extend_bdd_variables(self, &common_inputs);
        let other_lifted = extend_bdd_variables(other, &common_inputs);
        other_lifted.bdd.imp(&self_lifted.bdd).is_true()
    }
}
