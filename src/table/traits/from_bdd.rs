use crate::bdd::Bdd;
use crate::table::utils::boolean_point_to_row_index;
use crate::table::TruthTable;
use crate::traits::BooleanFunction;
use std::fmt::Debug;

impl<T: Debug + Clone + Ord> From<Bdd<T>> for TruthTable<T> {
    fn from(value: Bdd<T>) -> Self {
        let inputs = value.inputs();
        let mut outputs = vec![false; 2usize.pow(value.inputs().len() as u32)];

        value
            .support()
            .map(|point| boolean_point_to_row_index(&point))
            .for_each(|index| outputs[index] = true);

        TruthTable::new(inputs.into_iter().collect(), outputs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expressions::Expression;
    use biodivine_lib_bdd::BddVariableSet;
    use std::str::FromStr;

    #[test]
    fn test_table_from_bdd_standard() {
        let exp_string = "(b | a & c) & !a | false".to_string();
        let inputs = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        let var_set = BddVariableSet::from(inputs.clone());
        let inner_bdd = var_set.eval_expression_string(&exp_string);
        let bdd = Bdd::new(inner_bdd, inputs);

        let expected = TruthTable::from(Expression::from_str(&exp_string).unwrap());
        let actual = TruthTable::from(bdd);

        assert!(
            actual.is_equivalent(&expected),
            "expected: `{expected}`,\nactual: `{actual}`"
        );
    }

    #[test]
    fn test_table_from_bdd_n_ary() {
        let exp_string = "(a | b | c) | (!a & !b & !c)".to_string();
        let inputs = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        let var_set = BddVariableSet::from(inputs.clone());
        let inner_bdd = var_set.eval_expression_string(&exp_string);
        let bdd = Bdd::new(inner_bdd, inputs);

        let expected = TruthTable::from(Expression::from_str(&exp_string).unwrap());
        let actual = TruthTable::from(bdd);

        assert!(
            actual.is_equivalent(&expected),
            "expected: `{expected}`,\nactual: `{actual}`"
        );
    }
}
