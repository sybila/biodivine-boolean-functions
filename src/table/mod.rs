use std::fmt::{Debug, Display};
use std::hash::Hash;

use crate::expressions::Expression;
use crate::table::utils::row_index_to_valuation;

#[cfg(feature = "csv")]
pub mod csv;
pub mod display_formatted;
pub mod traits;
mod utils;

#[derive(Debug, PartialEq, Eq)]
pub struct TruthTable<TLiteral>
where
    TLiteral: Debug + Display + Clone + Eq + Hash,
{
    inputs: Vec<TLiteral>,
    outputs: Vec<bool>,
}

impl<TLiteral: Debug + Clone + Display + Eq + Hash> TruthTable<TLiteral> {
    fn new(inputs: Vec<TLiteral>, outputs: Vec<bool>) -> Self {
        Self { inputs, outputs }
    }

    pub fn row_count(&self) -> usize {
        2_usize.pow(self.variable_count() as u32)
    }

    pub fn variable_count(&self) -> usize {
        self.inputs.len()
    }

    pub fn is_empty(&self) -> bool {
        self.inputs.is_empty() && self.outputs.is_empty()
    }

    pub fn to_expression_trivial(&self) -> Expression<TLiteral> {
        let truth_row_indexes = self
            .outputs
            .iter()
            .enumerate()
            .filter(|(_index, is_row_true)| **is_row_true)
            .map(|(index, _value)| index);

        let and_expressions = truth_row_indexes
            .into_iter()
            .map(|row_index| self.row(row_index))
            .map(|row_values| {
                Expression::n_ary_and(
                    row_values
                        .into_iter()
                        .enumerate()
                        .map(|(index, literal_value)| self.cell_to_expression(index, literal_value))
                        .collect(),
                )
            })
            .collect();

        Expression::n_ary_or(and_expressions)
    }

    fn cell_to_expression(&self, cell_index: usize, literal_value: bool) -> Expression<TLiteral> {
        let literal = self
            .inputs
            .get(cell_index)
            .expect("Number of variables is different from number of values");
        if literal_value {
            Expression::Literal(literal.clone())
        } else {
            Expression::negate(Expression::Literal(literal.clone()))
        }
    }

    pub fn row_with_output(&self, row_index: usize) -> (Vec<bool>, bool) {
        (self.row(row_index), self.outputs[row_index])
    }

    pub fn row(&self, row_index: usize) -> Vec<bool> {
        row_index_to_valuation(row_index, self.variable_count())
    }
}

#[cfg(test)]
mod tests {
    use crate::expressions::Expression;
    use crate::expressions::Expression::Literal;
    use crate::table::TruthTable;
    use crate::traits::SemanticEq;

    #[test]
    fn test_to_expression_and_ok() {
        let input_expression = Expression::binary_and(Literal("x0"), Literal("x1"));
        let input_table = TruthTable::from(input_expression.clone());

        let actual = input_table.to_expression_trivial();
        assert!(actual.semantic_eq(&input_expression));

        // this inner is equal to input
        assert_eq!(actual, Expression::n_ary_or(vec![input_expression]));
    }

    #[test]
    fn test_to_expression_or_ok() {
        let input_expression = Expression::binary_or(Literal("x0"), Literal("x1"));
        let input_table = TruthTable::from(input_expression.clone());

        let actual = input_table.to_expression_trivial();
        assert!(actual.semantic_eq(&input_expression));

        assert_eq!(
            actual,
            Expression::n_ary_or(vec![
                Expression::binary_and(Expression::negate(Literal("x0")), Literal("x1")),
                Expression::binary_and(Literal("x0"), Expression::negate(Literal("x1"))),
                Expression::binary_and(Literal("x0"), Literal("x1")),
            ])
        );
    }
}
