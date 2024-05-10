pub fn boolean_point_to_row_index(point: &[bool]) -> usize {
    point
        .iter()
        .rev()
        .enumerate()
        .map(|(digit_index, var_is_true)| {
            if *var_is_true {
                2_usize.pow(digit_index as u32)
            } else {
                0
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let inputs = [
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

        for (expected, input) in inputs.iter().enumerate() {
            let actual = boolean_point_to_row_index(input);

            assert_eq!(actual, expected);
        }
    }
}
