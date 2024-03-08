use crate::parser::TokenizerInput;

// https://stackoverflow.com/a/38447886
/// Advances (i.e. calls .next()) `input` by `pop_count` characters, clears `pop_count` characters from the start of the `buffer`.
pub fn pop_n_left(buffer: &mut String, input: &mut TokenizerInput, pop_count: usize) {
    for _ in 0..pop_count {
        input.next();
    }

    match buffer.char_indices().nth(pop_count) {
        Some((pos, _)) => {
            buffer.drain(..pos);
        }
        None => {
            buffer.clear();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::structs::PositionTracker;
    use itertools::Itertools;

    use super::*;

    #[test]
    fn test_pop_n_left() {
        let mut buffer = String::from("");
        let input = buffer.clone();
        let mut input = PositionTracker::new(input.chars().multipeek());
        pop_n_left(&mut buffer, &mut input, 3);
        assert_eq!(&buffer, "");
        assert_eq!(&input.join(""), "");

        let mut buffer = String::from("a");
        let input = buffer.clone();
        let mut input = PositionTracker::new(input.chars().multipeek());
        pop_n_left(&mut buffer, &mut input, 3);
        assert_eq!(&buffer, "");
        assert_eq!(&input.join(""), "");

        let mut buffer = String::from("ab");
        let input = buffer.clone();
        let mut input = PositionTracker::new(input.chars().multipeek());
        pop_n_left(&mut buffer, &mut input, 3);
        assert_eq!(&buffer, "");
        assert_eq!(&input.join(""), "");

        let mut buffer = String::from("abc");
        let input = buffer.clone();
        let mut input = PositionTracker::new(input.chars().multipeek());
        pop_n_left(&mut buffer, &mut input, 3);
        assert_eq!(&buffer, "");
        assert_eq!(&input.join(""), "");

        let mut buffer = String::from("abcd");
        let input = buffer.clone();
        let mut input = PositionTracker::new(input.chars().multipeek());
        pop_n_left(&mut buffer, &mut input, 3);
        assert_eq!(&buffer, "d");
        assert_eq!(&input.join(""), "d");
    }
}
