use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::iter::once;

use tabled::builder::Builder;
use tabled::settings::Style;

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
            TableStyle::Empty => built.with(Style::empty()),
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
}

impl TableBooleanFormatting {
    fn format_bool(&self, value: &bool) -> String {
        match self {
            TableBooleanFormatting::Number => {
                if *value {
                    "1"
                } else {
                    "0"
                }
            }
            TableBooleanFormatting::Character => {
                if *value {
                    "T"
                } else {
                    "F"
                }
            }
            TableBooleanFormatting::Word => {
                if *value {
                    "true"
                } else {
                    "false"
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
                    .map(|value| inputs_formating.format_bool(value))
                    .chain(once(output_value).map(|value| output_formatting.format_bool(value)))
                    .map(|bool| bool.to_string())
                    .collect::<Vec<_>>()
            })
            .for_each(|row| builder.push_record(row));

        table_style.build_table_with(builder)
    }
}
