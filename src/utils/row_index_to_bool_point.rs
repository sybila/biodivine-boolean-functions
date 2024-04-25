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
