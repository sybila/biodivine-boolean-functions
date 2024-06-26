use std::fmt::{Debug, Display};

use std::iter::once;

use crate::expressions::{Expression, ExpressionNode};
use crate::table::display_formatted::TableBooleanFormatting;
use crate::utils::row_index_to_bool_point;

#[cfg(feature = "csv")]
pub mod csv;
pub mod display_formatted;
pub mod iterators;
pub mod traits;
mod utils;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TruthTable<TLiteral>
where
    TLiteral: Debug + Clone + Eq + Ord,
{
    inputs: Vec<TLiteral>,
    outputs: Vec<bool>,
}

impl<TLiteral: Debug + Clone + Eq + Ord> TruthTable<TLiteral> {
    pub(crate) fn new(inputs: Vec<TLiteral>, outputs: Vec<bool>) -> Self {
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

    /// Returns `Some(true)` if the table has no variables but one output equal to `true`,
    /// `Some(false`) if the table has no variables but one output equal to `false`,
    /// `None` otherwise.
    pub fn stub_value(&self) -> Option<bool> {
        if self.variable_count() == 0 {
            self.outputs.first().cloned()
        } else {
            None
        }
    }

    pub fn to_expression_trivial(&self) -> Expression<TLiteral> {
        if let Some(value) = self.stub_value() {
            return ExpressionNode::Constant(value).into();
        }

        let truth_row_indexes = self
            .outputs
            .iter()
            .enumerate()
            .filter(|(_index, is_row_true)| **is_row_true)
            .map(|(index, _value)| index)
            .collect::<Vec<_>>();

        if truth_row_indexes.is_empty() {
            return ExpressionNode::Constant(false).into();
        }
        if truth_row_indexes.len() == self.outputs.len() {
            return ExpressionNode::Constant(true).into();
        }

        let and_expressions = truth_row_indexes
            .into_iter()
            .map(|row_index| self.row(row_index))
            .map(|row_values| {
                Expression::n_ary_and(
                    &row_values
                        .into_iter()
                        .enumerate()
                        .map(|(index, literal_value)| self.cell_to_expression(index, literal_value))
                        .collect::<Vec<_>>(),
                )
            })
            .collect::<Vec<_>>();

        Expression::n_ary_or(&and_expressions)
    }

    fn cell_to_expression(&self, cell_index: usize, literal_value: bool) -> Expression<TLiteral> {
        let literal = self
            .inputs
            .get(cell_index)
            .expect("Number of variables is different from number of values");
        if literal_value {
            ExpressionNode::Literal(literal.clone()).into()
        } else {
            Expression::negate(&ExpressionNode::Literal(literal.clone()).into())
        }
    }

    pub fn row_with_output(&self, row_index: usize) -> (Vec<bool>, bool) {
        (self.row(row_index), self.outputs[row_index])
    }

    pub fn row(&self, row_index: usize) -> Vec<bool> {
        row_index_to_bool_point(row_index, self.variable_count())
    }
}

impl<TLiteral: Debug + Display + Clone + Eq + Ord> TruthTable<TLiteral> {
    fn header_row_iterator(&self) -> impl Iterator<Item = String> + '_ {
        self.inputs
            .iter()
            .map(|literal| literal.to_string())
            .chain(once("result".to_string()))
    }

    fn record_row(
        &self,
        row_index: usize,
        output_value: &bool,
        inputs_formatting: &TableBooleanFormatting,
        output_formatting: &TableBooleanFormatting,
    ) -> Vec<String> {
        row_index_to_bool_point(row_index, self.variable_count())
            .iter()
            .map(move |value| inputs_formatting.format_bool(value))
            .chain(once(output_value).map(move |value| output_formatting.format_bool(value)))
            .map(|bool| bool.to_string())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::expressions::Expression;
    use crate::expressions::{bool, var};
    use crate::table::TruthTable;
    use crate::traits::SemanticEq;

    #[test]
    fn test_to_expression_and_ok() {
        let input_expression = var("x0") & var("x1");
        let input_table = TruthTable::from(input_expression.clone());

        let actual = input_table.to_expression_trivial();
        assert!(actual.semantic_eq(&input_expression));

        // this inner is equal to input
        assert_eq!(actual, Expression::n_ary_or(&[input_expression]));
    }

    #[test]
    fn test_to_expression_or_ok() {
        let input_expression = var("x0") | var("x1");
        let input_table = TruthTable::from(input_expression.clone());

        let actual = input_table.to_expression_trivial();
        assert!(actual.semantic_eq(&input_expression));

        assert_eq!(
            actual,
            (!var("x0") & var("x1")) | (var("x0") & !var("x1")) | (var("x0") & var("x1"))
        );
    }

    #[test]
    fn test_to_expression_always_false_ok() {
        let input_expression = var("x0") & !var("x0");
        let input_table = TruthTable::from(input_expression.clone());

        let actual = input_table.to_expression_trivial();
        assert!(actual.semantic_eq(&input_expression));

        assert_eq!(actual, bool(false));
    }

    #[test]
    fn test_to_expression_always_true_ok() {
        let input_expression = var("x0") | !var("x0");
        let input_table = TruthTable::from(input_expression.clone());

        let actual = input_table.to_expression_trivial();
        assert!(actual.semantic_eq(&input_expression));

        assert_eq!(actual, bool(true));
    }

    #[test]
    fn test_to_expression_variableless_tautology_ok() {
        let input_expression = bool(true) | bool(false);
        let input_table = TruthTable::from(input_expression.clone());

        let actual = input_table.to_expression_trivial();
        assert!(actual.semantic_eq(&input_expression));

        // this inner is equal to input
        assert_eq!(actual, bool(true));
    }

    #[test]
    fn test_to_expression_variableless_contradiction_ok() {
        let input_expression = bool(true) & bool(false);
        let input_table = TruthTable::from(input_expression.clone());

        let actual = input_table.to_expression_trivial();
        assert!(actual.semantic_eq(&input_expression));

        // this inner is equal to input
        assert_eq!(actual, bool(false));
    }
}
