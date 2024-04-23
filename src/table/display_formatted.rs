use std::fmt::{Debug, Display};

use tabled::builder::Builder;
use tabled::settings::{Padding, Style};

use crate::table::TruthTable;

#[cfg_attr(feature = "python", pyo3::pyclass)]
#[derive(Default, Copy, Clone)]
pub enum TableStyle {
    Ascii,
    Modern,
    Markdown,
    #[default]
    Empty,
}

impl TableStyle {
    fn build_table_with(&self, builder: Builder) -> String {
        let mut built = builder.build();
        match self {
            // The default style is not as empty as it seems
            TableStyle::Empty => built.with(Style::empty()).with(Padding::new(0, 1, 0, 0)),
            TableStyle::Ascii => built.with(Style::ascii()),
            TableStyle::Modern => built.with(Style::modern()),
            TableStyle::Markdown => built.with(Style::markdown()),
        }
        .to_string()
    }
}

#[cfg_attr(feature = "python", pyo3::pyclass)]
#[derive(Default, Copy, Clone)]
pub enum TableBooleanFormatting {
    #[default]
    Number,
    Character,
    Word,
    CapitalizedWord,
}

pub const FALSE_NUMBER: &str = "0";
pub const FALSE_CHARACTER: &str = "F";
pub const FALSE_WORD: &str = "false";
pub const FALSE_CAPITALIZED_WORD: &str = "False";
pub const TRUE_NUMBER: &str = "1";
pub const TRUE_CHARACTER: &str = "T";
pub const TRUE_WORD: &str = "true";
pub const TRUE_CAPITALIZED_WORD: &str = "True";

pub const ALL_BOOL_STRINGS: [&str; 8] = [
    FALSE_NUMBER,
    FALSE_CHARACTER,
    FALSE_WORD,
    FALSE_CAPITALIZED_WORD,
    TRUE_NUMBER,
    TRUE_CHARACTER,
    TRUE_WORD,
    TRUE_CAPITALIZED_WORD,
];

impl TableBooleanFormatting {
    pub fn format_bool(&self, value: &bool) -> String {
        match self {
            TableBooleanFormatting::Number => {
                if *value {
                    TRUE_NUMBER
                } else {
                    FALSE_NUMBER
                }
            }
            TableBooleanFormatting::Character => {
                if *value {
                    TRUE_CHARACTER
                } else {
                    FALSE_CHARACTER
                }
            }
            TableBooleanFormatting::Word => {
                if *value {
                    TRUE_WORD
                } else {
                    FALSE_WORD
                }
            }
            TableBooleanFormatting::CapitalizedWord => {
                if *value {
                    TRUE_CAPITALIZED_WORD
                } else {
                    FALSE_CAPITALIZED_WORD
                }
            }
        }
        .to_string()
    }
}

impl<TLiteral: Debug + Clone + Display + Eq + Ord> TruthTable<TLiteral> {
    pub fn to_string_formatted(
        &self,
        table_style: TableStyle,
        inputs_formating: TableBooleanFormatting,
        output_formatting: TableBooleanFormatting,
    ) -> String {
        let mut builder = Builder::default();

        let header = self.header_row_iterator().collect::<Vec<_>>();
        builder.push_record(header);

        self.outputs
            .iter()
            .enumerate()
            .map(|(row_index, output_value)| {
                self.record_row(
                    row_index,
                    output_value,
                    &inputs_formating,
                    &output_formatting,
                )
            })
            .for_each(|row| builder.push_record(row));

        table_style.build_table_with(builder)
    }
}

#[cfg(test)]
mod tests {
    use super::TableBooleanFormatting::{self, CapitalizedWord, Character, Number, Word};
    use crate::table::display_formatted::TableStyle;
    use crate::table::TruthTable;
    use rstest::rstest;
    use rstest_reuse::{apply, template};

    #[template]
    #[rstest]
    fn formatting_template(
        #[values(Number, Character, Word, CapitalizedWord)]
        inputs_formatting: TableBooleanFormatting,
        #[values(Number, Character, Word, CapitalizedWord)]
        output_formatting: TableBooleanFormatting,
    ) {
    }

    fn replace_macros(
        input_table: &str,
        inputs_formatting: &TableBooleanFormatting,
        output_formatting: &TableBooleanFormatting,
    ) -> String {
        input_table
            .replace(
                'o',
                &format!("{: <5}", inputs_formatting.format_bool(&false)),
            )
            .replace(
                'i',
                &format!("{: <5}", inputs_formatting.format_bool(&true)),
            )
            .replace(
                'O',
                &format!("{: <6}", output_formatting.format_bool(&false)),
            )
            .replace(
                'I',
                &format!("{: <6}", output_formatting.format_bool(&true)),
            )
    }

    #[apply(formatting_template)]
    fn test_modern_ok(
        inputs_formatting: TableBooleanFormatting,
        output_formatting: TableBooleanFormatting,
    ) {
        let input_table = TruthTable::new(vec!["ěýáíé", "ščřžň"], vec![true, false, true, false]);

        let expected = replace_macros(
            concat!(
                "┌───────┬───────┬────────┐\n",
                "│ ěýáíé │ ščřžň │ result │\n",
                "├───────┼───────┼────────┤\n",
                "│ o │ o │ I │\n",
                "├───────┼───────┼────────┤\n",
                "│ o │ i │ O │\n",
                "├───────┼───────┼────────┤\n",
                "│ i │ o │ I │\n",
                "├───────┼───────┼────────┤\n",
                "│ i │ i │ O │\n",
                "└───────┴───────┴────────┘",
            ),
            &inputs_formatting,
            &output_formatting,
        );

        let actual = input_table.to_string_formatted(
            TableStyle::Modern,
            inputs_formatting,
            output_formatting,
        );

        assert_eq!(expected, actual);
    }

    #[apply(formatting_template)]
    fn test_markdown_ok(
        inputs_formatting: TableBooleanFormatting,
        output_formatting: TableBooleanFormatting,
    ) {
        let input_table = TruthTable::new(vec!["ěýáíé", "ščřžň"], vec![true, false, true, false]);

        let expected = replace_macros(
            concat!(
                "| ěýáíé | ščřžň | result |\n",
                "|-------|-------|--------|\n",
                "| o | o | I |\n",
                "| o | i | O |\n",
                "| i | o | I |\n",
                "| i | i | O |"
            ),
            &inputs_formatting,
            &output_formatting,
        );

        let actual = input_table.to_string_formatted(
            TableStyle::Markdown,
            inputs_formatting,
            output_formatting,
        );

        assert_eq!(expected, actual);
    }

    #[apply(formatting_template)]
    fn test_ascii_ok(
        inputs_formatting: TableBooleanFormatting,
        output_formatting: TableBooleanFormatting,
    ) {
        let input_table = TruthTable::new(vec!["ěýáíé", "ščřžň"], vec![true, false, true, false]);

        let expected = replace_macros(
            concat!(
                "+-------+-------+--------+\n",
                "| ěýáíé | ščřžň | result |\n",
                "+-------+-------+--------+\n",
                "| o | o | I |\n",
                "+-------+-------+--------+\n",
                "| o | i | O |\n",
                "+-------+-------+--------+\n",
                "| i | o | I |\n",
                "+-------+-------+--------+\n",
                "| i | i | O |\n",
                "+-------+-------+--------+",
            ),
            &inputs_formatting,
            &output_formatting,
        );

        let actual = input_table.to_string_formatted(
            TableStyle::Ascii,
            inputs_formatting,
            output_formatting,
        );

        assert_eq!(expected, actual);
    }
}
