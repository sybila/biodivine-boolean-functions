use std::collections::{BTreeMap, BTreeSet};
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Read};
use std::path::Path;

use csv::{ReaderBuilder, StringRecord};

use crate::table::csv::error::TruthTableFromCsvError;
use crate::table::csv::utils::string_to_bool;
use crate::table::display_formatted::ALL_BOOL_STRINGS;
use crate::table::utils::values_to_row_index;
use crate::table::TruthTable;

// a;b;c;d;whatever           <-- maybe header
// 0;1;0;1;0
// T;T;T;F;F
// True;True;True;False;False
// true;true;true;false;false
// 0;T;False;false;1

impl TruthTable<String> {
    pub fn from_csv_file(
        path: impl AsRef<Path>,
    ) -> Result<TruthTable<String>, TruthTableFromCsvError> {
        let file_row_count = BufReader::new(File::open(&path)?).lines().count();
        if file_row_count == 0 {
            return Ok(TruthTable {
                inputs: vec![],
                outputs: vec![],
            });
        }

        Self::from_csv_common(file_row_count, Box::new(File::open(path)?))
    }

    pub fn from_csv_string(input: &str) -> Result<TruthTable<String>, TruthTableFromCsvError> {
        if input.is_empty() {
            return Ok(TruthTable {
                inputs: vec![],
                outputs: vec![],
            });
        }

        let file_row_count = input.split('\n').count();

        Self::from_csv_common(file_row_count, Box::new(io::Cursor::new(input.to_string())))
    }

    fn from_csv_common(
        file_row_count: usize,
        read: Box<dyn Read>,
    ) -> Result<TruthTable<String>, TruthTableFromCsvError> {
        let mut reader = ReaderBuilder::new()
            .has_headers(false)
            .delimiter(b',')
            .flexible(false)
            .from_reader(read);
        let mut maybe_header_record = StringRecord::new();
        if !reader.read_record(&mut maybe_header_record)? {
            return Err(TruthTableFromCsvError::UnexpectedEof);
        }

        let (variable_column_index_map, expected_variable_count) =
            determine_variables(file_row_count, &mut maybe_header_record)?;

        let mut outputs = vec![false; 2_usize.pow(variable_column_index_map.len() as u32)];

        for (csv_row_index, result) in reader.records().enumerate() {
            let record = result?;

            let valuation = parse_input_columns(
                expected_variable_count,
                &record,
                &variable_column_index_map,
                csv_row_index,
            )?;

            let index = values_to_row_index(
                &variable_column_index_map
                    .keys()
                    .cloned()
                    .collect::<Vec<_>>(),
                &valuation,
            );

            // access safe due to ensure_record_count check above
            outputs[index] = parse_output_column(&record)?;
        }

        Ok(TruthTable::new(
            variable_column_index_map.into_keys().collect(),
            outputs,
        ))
    }
}

fn determine_variables(
    file_row_count: usize,
    maybe_header_record: &mut StringRecord,
) -> Result<(BTreeMap<String, usize>, usize), TruthTableFromCsvError> {
    let (variable_column_index_map, expected_variable_count) = if is_header(maybe_header_record)? {
        let mapping = inputs_from_header(maybe_header_record)?;

        // - 1 is because row_count = 1 header_row + record_rows
        ensure_record_count(file_row_count - 1, &mapping)?;

        (mapping, maybe_header_record.len())
    } else {
        let mapping = inputs_from_first_record(maybe_header_record);

        ensure_record_count(file_row_count, &mapping)?;

        let expected = mapping.keys().count();
        (mapping, expected)
    };

    Ok((variable_column_index_map, expected_variable_count))
}

fn ensure_record_count(
    actual_record_count: usize,
    variable_column_index_map: &BTreeMap<String, usize>,
) -> Result<(), TruthTableFromCsvError> {
    let expected_record_count = 2_usize.pow(variable_column_index_map.len() as u32);

    if actual_record_count != expected_record_count {
        return Err(
            TruthTableFromCsvError::MismatchedRecordCountAndVariableCount {
                variable_count: variable_column_index_map.len(),
                actual_row_count: actual_record_count,
            },
        );
    }

    Ok(())
}

fn parse_input_columns(
    expected_variable_count: usize,
    record: &StringRecord,
    variable_column_index_map: &BTreeMap<String, usize>,
    csv_row_index: usize,
) -> Result<BTreeMap<String, bool>, TruthTableFromCsvError> {
    let mut valuation = BTreeMap::new();

    for (key_var, column_index) in variable_column_index_map.iter() {
        let cell_value = record.get(*column_index).ok_or(
            TruthTableFromCsvError::RecordDifferentSizeThanHeader {
                row_index: csv_row_index,
                expected_row_len: expected_variable_count,
                actual_row_len: record.len(),
            },
        )?;
        let cell_value =
            string_to_bool(cell_value).ok_or(TruthTableFromCsvError::NonBooleanCellValue {
                actual: cell_value.to_string(),
            })?;

        valuation.insert(key_var.clone(), cell_value);
    }

    Ok(valuation)
}

fn parse_output_column(record: &StringRecord) -> Result<bool, TruthTableFromCsvError> {
    let output = record
        .iter()
        .last()
        .ok_or(TruthTableFromCsvError::NoOutputColumn)?;

    let bool_output =
        string_to_bool(output).ok_or(TruthTableFromCsvError::NonBooleanCellValue {
            actual: output.to_string(),
        })?;

    Ok(bool_output)
}

fn inputs_from_header(
    header: &StringRecord,
) -> Result<BTreeMap<String, usize>, TruthTableFromCsvError> {
    let input_header_cells = header.iter().enumerate().take(header.len() - 1); // skip last element = output

    let mut unique_test_set = BTreeSet::new();
    for (_index, name) in input_header_cells.clone() {
        if !unique_test_set.insert(name) {
            return Err(TruthTableFromCsvError::DuplicateVariableName {
                name: name.to_string(),
            });
        }
    }

    let result = input_header_cells
        .map(|(index, cell)| (cell.to_string(), index))
        .collect::<BTreeMap<_, _>>();

    Ok(result)
}

fn inputs_from_first_record(first_record: &StringRecord) -> BTreeMap<String, usize> {
    // -1 is for the expected output column
    BTreeMap::from_iter((0..(first_record.len() - 1)).map(|i| (format!("x_{}", i), i)))
}

fn is_header(record: &StringRecord) -> Result<bool, TruthTableFromCsvError> {
    match record.iter().last() {
        None => Err(TruthTableFromCsvError::NoOutputColumn),
        Some(value) => Ok(!ALL_BOOL_STRINGS.contains(&value)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;
    use rstest_reuse::{apply, template};
    use std::io::Write;
    use tempfile::{Builder, NamedTempFile};

    fn contents_to_temp_file(contents: &str) -> NamedTempFile {
        let mut file = Builder::new()
            .prefix("boolean-functions-csv")
            .rand_bytes(8)
            .suffix(".csv")
            .tempfile()
            .expect("Temp file should get created");

        file.write_all(contents.as_bytes())
            .expect("Temp file write should succeed");

        file
    }

    fn remove_first_line(s: &str) -> &str {
        match s.find('\n') {
            Some(index) => &s[index + 1..],
            None => s,
        }
    }

    #[test]
    fn test_empty_table_ok() -> Result<(), TruthTableFromCsvError> {
        let file = contents_to_temp_file("");

        let table = TruthTable::from_csv_file(file.path())?;

        assert_eq!(table.inputs, Vec::<String>::new());
        assert_eq!(table.outputs, vec![]);

        Ok(())
    }

    #[test]
    fn test_empty_string_ok() -> Result<(), TruthTableFromCsvError> {
        let contents = "";

        let table = TruthTable::from_csv_string(contents)?;

        assert_eq!(table.inputs, Vec::<String>::new());
        assert_eq!(table.outputs, vec![]);

        Ok(())
    }

    #[test]
    fn test_onlyheaders_string_ok() -> Result<(), TruthTableFromCsvError> {
        let contents = "a,b,result";

        let actual = TruthTable::from_csv_string(contents);
        let expected_err_message = TruthTableFromCsvError::MismatchedRecordCountAndVariableCount {
            variable_count: 2,
            actual_row_count: 0,
        }
        .to_string();

        assert!(actual.is_err());
        assert_eq!(actual.unwrap_err().to_string(), expected_err_message);

        Ok(())
    }

    #[test]
    fn test_eof_error_nok() -> Result<(), TruthTableFromCsvError> {
        let file = contents_to_temp_file("");

        let table = TruthTable::from_csv_common(0, Box::new(File::open(file.path())?));
        let expected_err_message = TruthTableFromCsvError::UnexpectedEof.to_string();

        assert!(table.is_err());
        assert!(table
            .unwrap_err()
            .to_string()
            .contains(&expected_err_message));
        Ok(())
    }

    #[template]
    #[rstest]
    #[case::with_headers(false)]
    #[case::no_headers(true)]
    fn with_without_headers_template(#[case] remove_headers: bool) {}

    #[apply(with_without_headers_template)]
    fn test_number_one_variable_ok(
        #[case] remove_headers: bool,
    ) -> Result<(), TruthTableFromCsvError> {
        let csv_contents = concat!("x_0,whatever\n", "0,0\n", "1,1\n");
        let file = contents_to_temp_file(if remove_headers {
            remove_first_line(csv_contents)
        } else {
            csv_contents
        });

        let table = TruthTable::from_csv_file(file.path())?;

        assert_eq!(table.inputs, vec!["x_0".to_string()]);
        assert_eq!(table.outputs, vec![false, true]);

        Ok(())
    }

    #[apply(with_without_headers_template)]
    fn test_number_bools_ordered_inputs_ordered_rows_ok(
        #[case] remove_headers: bool,
    ) -> Result<(), TruthTableFromCsvError> {
        let csv_contents = concat!(
            "x_0,x_1,whatever\n",
            "0,0,0\n",
            "0,1,0\n",
            "1,0,1\n",
            "1,1,1\n"
        );
        let file = contents_to_temp_file(if remove_headers {
            remove_first_line(csv_contents)
        } else {
            csv_contents
        });

        let table = TruthTable::from_csv_file(file.path())?;

        assert_eq!(table.inputs, vec!["x_0".to_string(), "x_1".to_string()]);
        assert_eq!(table.outputs, vec![false, false, true, true]);

        Ok(())
    }

    #[test]
    fn test_number_bools_unordered_inputs_ordered_rows_ok() -> Result<(), TruthTableFromCsvError> {
        let csv_contents = concat!("b,a,whatever\n", "0,0,0\n", "1,0,0\n", "0,1,1\n", "1,1,1\n");
        let file = contents_to_temp_file(csv_contents);

        let table = TruthTable::from_csv_file(file.path())?;
        assert_eq!(table.inputs, vec!["a".to_string(), "b".to_string()]);
        assert_eq!(table.outputs, vec![false, false, true, true]);

        Ok(())
    }

    #[test]
    fn test_headerless_number_bools_unordered_inputs_ordered_rows_ok(
    ) -> Result<(), TruthTableFromCsvError> {
        let csv_contents = concat!("0,0,0\n", "1,0,0\n", "0,1,1\n", "1,1,1\n");
        let file = contents_to_temp_file(csv_contents);

        let table = TruthTable::from_csv_file(file.path())?;
        assert_eq!(table.inputs, vec!["x_0".to_string(), "x_1".to_string()]);
        assert_eq!(table.outputs, vec![false, true, false, true]);

        Ok(())
    }

    #[apply(with_without_headers_template)]
    fn test_number_bools_ordered_inputs_unordered_rows_ok(
        #[case] remove_headers: bool,
    ) -> Result<(), TruthTableFromCsvError> {
        let csv_contents = concat!(
            "x_0,x_1,whatever\n",
            "0,0,0\n",
            "1,1,1\n",
            "1,0,1\n",
            "0,1,0\n"
        );
        let file = contents_to_temp_file(if remove_headers {
            remove_first_line(csv_contents)
        } else {
            csv_contents
        });

        let table = TruthTable::from_csv_file(file.path())?;
        assert_eq!(table.inputs, vec!["x_0".to_string(), "x_1".to_string()]);
        assert_eq!(table.outputs, vec![false, false, true, true]);

        Ok(())
    }

    #[test]
    fn test_number_bools_unordered_inputs_unordered_rows_ok() -> Result<(), TruthTableFromCsvError>
    {
        let csv_contents = concat!("b,a,whatever\n", "0,0,0\n", "1,1,1\n", "0,1,1\n", "1,0,0\n");
        let file = contents_to_temp_file(csv_contents);

        let table = TruthTable::from_csv_file(file.path())?;
        assert_eq!(table.inputs, vec!["a".to_string(), "b".to_string()]);
        assert_eq!(table.outputs, vec![false, false, true, true]);

        Ok(())
    }

    #[test]
    fn test_headerless_number_bools_unordered_inputs_unordered_rows_ok(
    ) -> Result<(), TruthTableFromCsvError> {
        let csv_contents = concat!("0,0,0\n", "1,1,1\n", "0,1,1\n", "1,0,0\n");
        let file = contents_to_temp_file(csv_contents);

        let table = TruthTable::from_csv_file(file.path())?;
        assert_eq!(table.inputs, vec!["x_0".to_string(), "x_1".to_string()]);
        assert_eq!(table.outputs, vec![false, true, false, true]);

        Ok(())
    }

    #[apply(with_without_headers_template)]
    fn test_invalid_bool_nok(#[case] remove_headers: bool) -> Result<(), TruthTableFromCsvError> {
        let csv_contents = concat!(
            "b,a,whatever\n",
            "0,0,0\n",
            "1,1,1\n",
            "0,1,1\n",
            "1,falsificate,0\n"
        );
        let file = contents_to_temp_file(if remove_headers {
            remove_first_line(csv_contents)
        } else {
            csv_contents
        });

        let actual = TruthTable::from_csv_file(file.path());
        let expected_err_message = "falsificate";

        assert!(actual.is_err());
        assert!(actual
            .unwrap_err()
            .to_string()
            .contains(expected_err_message));

        Ok(())
    }

    #[apply(with_without_headers_template)]
    fn test_flexible_row_len_shorter_nok(
        #[case] remove_headers: bool,
    ) -> Result<(), TruthTableFromCsvError> {
        let csv_contents = concat!("b,a,whatever\n", "0,0,0\n", "1,1\n", "0,1,1\n", "1,1,0\n");
        let file = contents_to_temp_file(if remove_headers {
            remove_first_line(csv_contents)
        } else {
            csv_contents
        });

        let actual = TruthTable::from_csv_file(file.path());
        let expected_err_message =
            "found record with 2 fields, but the previous record has 3 fields";

        assert!(actual.is_err());
        assert!(actual
            .unwrap_err()
            .to_string()
            .contains(expected_err_message));

        Ok(())
    }

    #[apply(with_without_headers_template)]
    fn test_flexible_row_len_longer_nok(
        #[case] remove_headers: bool,
    ) -> Result<(), TruthTableFromCsvError> {
        let csv_contents = concat!(
            "b,a,whatever\n",
            "0,0,0\n",
            "1,1,1,1\n",
            "0,1,1\n",
            "1,1,0\n"
        );
        let file = contents_to_temp_file(if remove_headers {
            remove_first_line(csv_contents)
        } else {
            csv_contents
        });

        let actual = TruthTable::from_csv_file(file.path());
        let expected_err_message =
            "found record with 4 fields, but the previous record has 3 fields";

        assert!(actual.is_err());
        assert!(actual
            .unwrap_err()
            .to_string()
            .contains(expected_err_message));

        Ok(())
    }

    #[apply(with_without_headers_template)]
    fn test_not_enough_rows_nok(
        #[case] remove_headers: bool,
    ) -> Result<(), TruthTableFromCsvError> {
        let csv_contents = concat!("b,a,whatever\n", "0,0,0\n", "0,1,1\n", "1,1,0\n");
        let file = contents_to_temp_file(if remove_headers {
            remove_first_line(csv_contents)
        } else {
            csv_contents
        });

        let actual = TruthTable::from_csv_file(file.path());
        let expected_err_message = TruthTableFromCsvError::MismatchedRecordCountAndVariableCount {
            variable_count: 2,
            actual_row_count: 3,
        }
        .to_string();

        assert!(actual.is_err());
        assert_eq!(actual.unwrap_err().to_string(), expected_err_message);

        Ok(())
    }

    #[apply(with_without_headers_template)]
    fn test_too_many_rows_nok(#[case] remove_headers: bool) -> Result<(), TruthTableFromCsvError> {
        let csv_contents = concat!(
            "b,a,whatever\n",
            "0,0,0\n",
            "0,1,1\n",
            "0,1,1\n",
            "0,1,1\n",
            "1,1,0\n"
        );
        let file = contents_to_temp_file(if remove_headers {
            remove_first_line(csv_contents)
        } else {
            csv_contents
        });

        let actual = TruthTable::from_csv_file(file.path());
        let expected_err_message = TruthTableFromCsvError::MismatchedRecordCountAndVariableCount {
            variable_count: 2,
            actual_row_count: 5,
        }
        .to_string();

        assert!(actual.is_err());
        assert_eq!(actual.unwrap_err().to_string(), expected_err_message);

        Ok(())
    }

    #[test]
    fn test_same_variable_name_nok() -> Result<(), TruthTableFromCsvError> {
        let csv_contents = concat!("b,b,whatever\n", "0,0,0\n", "0,1,1\n", "0,1,1\n", "0,1,1\n",);
        let file = contents_to_temp_file(csv_contents);

        let actual = TruthTable::from_csv_file(file.path());
        let expected_err_message = TruthTableFromCsvError::DuplicateVariableName {
            name: "b".to_string(),
        }
        .to_string();

        assert!(actual.is_err());
        assert_eq!(actual.unwrap_err().to_string(), expected_err_message);

        Ok(())
    }
}
