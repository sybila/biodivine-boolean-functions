use itertools::MultiPeek;
use std::str::Chars;

// https://stackoverflow.com/a/38447886
/// Advances (i.e. calls .next()) `input` by `pop_count` characters, clears `pop_count` characters from the start of the `buffer`.
pub fn pop_n_left(buffer: &mut String, input: &mut MultiPeek<Chars>, pop_count: usize) {
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
    use super::*;
    use itertools::Itertools;

    #[test]
    fn test_pop_n_left() {
        let mut buffer = String::from("");
        let input = buffer.clone();
        let mut input = input.chars().multipeek();

        pop_n_left(&mut buffer, &mut input, 3);
        assert_eq!(&buffer, "");
        assert_eq!(&input.join(""), "");

        let mut buffer = String::from("a");
        pop_n_left(&mut buffer, &mut input, 3);
        let input = buffer.clone();
        let mut input = input.chars().multipeek();
        assert_eq!(&buffer, "");
        assert_eq!(&input.join(""), "");

        let mut buffer = String::from("ab");
        pop_n_left(&mut buffer, &mut input, 3);
        let input = buffer.clone();
        let mut input = input.chars().multipeek();
        assert_eq!(&buffer, "");
        assert_eq!(&input.join(""), "");

        let mut buffer = String::from("abc");
        pop_n_left(&mut buffer, &mut input, 3);
        let input = buffer.clone();
        let mut input = input.chars().multipeek();
        assert_eq!(&buffer, "");
        assert_eq!(&input.join(""), "");

        let mut buffer = String::from("abcd");
        pop_n_left(&mut buffer, &mut input, 3);
        let input = buffer.clone();
        let mut input = input.chars().multipeek();
        assert_eq!(&buffer, "d");
        assert_eq!(&input.join(""), "d");
    }
}
