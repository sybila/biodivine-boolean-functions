use std::fmt::{Debug, Display};
use std::hash::Hash;

use tabled::builder::Builder;
use tabled::settings::{Padding, Style};

use crate::table::TruthTable;

#[derive(Default)]
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

#[derive(Default)]
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

impl<TLiteral: Debug + Clone + Display + Eq + Hash> TruthTable<TLiteral> {
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
