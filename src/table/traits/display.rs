use std::fmt::{Debug, Display, Formatter};
use std::hash::Hash;
use std::iter::once;

use tabled::builder::Builder;
use tabled::settings::Style;

use crate::table::TruthTable;

#[derive(Default)]
enum TableStyle {
    Ascii,
    Modern,
    Markdown,
    #[default]
    Empty,
}

#[derive(Default)]
enum TableBooleanFormatting {
    #[default]
    Number,
    Character,
    Word,
}

impl<TLiteral: Debug + Display + Clone + Eq + Hash + Ord> Display for TruthTable<TLiteral> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut builder = Builder::default();

        let header = self
            .inputs
            .iter()
            .map(|literal| literal.to_string())
            .chain(once("result".to_string()))
            .collect::<Vec<_>>();
        builder.push_record(header);

        self.outputs
            .iter()
            .enumerate()
            .map(|(row_index, output_value)| {
                self.row(row_index)
                    .iter()
                    .chain(once(output_value))
                    .map(|bool| bool.to_string())
                    .collect::<Vec<_>>()
            })
            .for_each(|row| builder.push_record(row));

        write!(f, "{}", builder.build().with(Style::empty()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expressions::Expression;
    use crate::expressions::Expression::Literal;

    #[test]
    fn test_output_and_ok() {
        let input_expression = Expression::binary_and(Literal("x0"), Literal("x1"));
        let input_table = TruthTable::from(input_expression.clone());

        let expected = concat!(
            "x0     x1     result \n",
            "false  false  false  \n",
            "false  true   false  \n",
            "true   false  false  \n",
            "true   true   true   \n",
        );
        let actual = input_table.to_string();

        assert_eq!(input_table.to_string(), actual);
    }
}
