use std::fmt::{Debug, Display, Formatter};

use crate::table::display_formatted::{TableBooleanFormatting, TableStyle};
use crate::table::TruthTable;

impl<TLiteral: Debug + Display + Clone + Eq + Ord> Display for TruthTable<TLiteral> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.to_string_formatted(
                TableStyle::Empty,
                TableBooleanFormatting::Word,
                TableBooleanFormatting::Word
            )
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::expressions::var;

    use super::*;

    #[test]
    fn test_output_short_variables_and_ok() {
        let input_expression = var("x0") & var("x1");
        let input_table = TruthTable::from(input_expression.clone());

        let expected = concat!(
            "x0    x1    result \n",
            "false false false  \n",
            "false true  false  \n",
            "true  false false  \n",
            "true  true  true   ",
        );
        let actual = input_table.to_string();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_output_long_variables_and_ok() {
        let input_expression = var("longvariablename1") & var("longvariablename2");
        let input_table = TruthTable::from(input_expression.clone());

        let expected = concat!(
            "longvariablename1 longvariablename2 result \n",
            "false             false             false  \n",
            "false             true              false  \n",
            "true              false             false  \n",
            "true              true              true   ",
        );
        let actual = input_table.to_string();

        assert_eq!(expected, actual);
    }
}
