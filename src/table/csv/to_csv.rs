use crate::table::display_formatted::TableBooleanFormatting;
use crate::table::TruthTable;
use itertools::Itertools;
use std::fmt::{Debug, Display};
use std::hash::Hash;

impl<TLiteral: Debug + Display + Clone + Eq + Hash> TruthTable<TLiteral> {
    pub fn to_csv(&self) -> String {
        self.to_csv_formatted(
            ',',
            TableBooleanFormatting::Number,
            TableBooleanFormatting::Number,
        )
    }

    pub fn to_csv_formatted(
        &self,
        delimiter: char,
        inputs_formatting: TableBooleanFormatting,
        output_formatting: TableBooleanFormatting,
    ) -> String {
        if self.is_empty() {
            return "".to_string();
        }

        let delimiter = &delimiter.to_string();
        let header = self.header_row_iterator().join(delimiter);

        let rows = self
            .outputs
            .iter()
            .enumerate()
            .map(|(row_index, output_value)| {
                self.record_row(
                    row_index,
                    output_value,
                    &inputs_formatting,
                    &output_formatting,
                )
                .join(delimiter)
            })
            .join("\n");

        format!("{header}\n{rows}")
    }
}

#[cfg(test)]
mod tests {
    use crate::table::csv::error::TruthTableFromCsvError;
    use crate::table::display_formatted::TableBooleanFormatting;
    use crate::table::TruthTable;

    #[test]
    fn test_to_csv_empty_ok() {
        let input = TruthTable::<String> {
            inputs: vec![],
            outputs: vec![],
        };

        let actual = input.to_csv();
        let expected = "".to_string();

        assert_eq!(actual, expected)
    }

    #[test]
    fn test_to_csv_formatted_ok() {
        let input = TruthTable {
            inputs: vec!["vara".to_string(), "varb".to_string()],
            outputs: vec![true, true, true, true],
        };

        let actual = input.to_csv_formatted(
            ';',
            TableBooleanFormatting::Character,
            TableBooleanFormatting::Word,
        );
        let expected = concat!(
            "vara;varb;result\n",
            "F;F;true\n",
            "F;T;true\n",
            "T;F;true\n",
            "T;T;true"
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_to_csv_from_csv_equals_ok() -> Result<(), TruthTableFromCsvError> {
        let input = TruthTable {
            inputs: vec!["x0".to_string(), "x1".to_string()],
            outputs: vec![false, false, true, true],
        };

        let intermediate_csv = input.to_csv();
        let actual = TruthTable::from_csv_string(&intermediate_csv)?;

        assert_eq!(actual, input);

        Ok(())
    }
}
