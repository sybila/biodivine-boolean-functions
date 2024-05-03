pub fn row_index_to_bool_point(mut row_index: usize, min_target_len: usize) -> Vec<bool> {
    let mut result = vec![];

    while row_index > 0 {
        let digit = row_index.rem_euclid(2);
        row_index /= 2;
        result.push(digit != 0);
    }

    while result.len() < min_target_len {
        result.push(false);
    }

    result.reverse();

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_row_index_to_bool_point() {
        let expected_results = [
            vec![false, false, false, false, false],
            vec![false, false, false, false, true],
            vec![false, false, false, true, false],
            vec![false, false, false, true, true],
            vec![false, false, true, false, false],
            vec![false, false, true, false, true],
            vec![false, false, true, true, false],
            vec![false, false, true, true, true],
            vec![false, true, false, false, false],
            vec![false, true, false, false, true],
            vec![false, true, false, true, false],
            vec![false, true, false, true, true],
            vec![false, true, true, false, false],
            vec![false, true, true, false, true],
            vec![false, true, true, true, false],
            vec![false, true, true, true, true],
            vec![true, false, false, false, false],
            vec![true, false, false, false, true],
            vec![true, false, false, true, false],
            vec![true, false, false, true, true],
            vec![true, false, true, false, false],
            vec![true, false, true, false, true],
            vec![true, false, true, true, false],
            vec![true, false, true, true, true],
            vec![true, true, false, false, false],
            vec![true, true, false, false, true],
            vec![true, true, false, true, false],
            vec![true, true, false, true, true],
            vec![true, true, true, false, false],
            vec![true, true, true, false, true],
            vec![true, true, true, true, false],
            vec![true, true, true, true, true],
        ];
        let size = expected_results[0].len();

        for (i, expected) in expected_results.iter().enumerate() {
            let actual = &row_index_to_bool_point(i, size);

            assert_eq!(actual, expected);
        }
    }
}
