use itertools::MultiPeek;
use std::str::Chars;

/// Returns `false` if if no characters were peeked from the `input` iterator, `true` otherwise.
pub fn peek_until_n(n: usize, input: &mut MultiPeek<Chars>, buffer: &mut String) -> bool {
    let mut did_read_anything = false;

    while buffer.chars().count() < n {
        let c = input.peek();

        match c {
            Some(c) => {
                did_read_anything = true;
                buffer.push(*c)
            }
            None => break,
        }
    }

    did_read_anything
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;

    #[test]
    fn test_peek_until_n() {
        for input_size in 0..=10 {
            let input = "a".repeat(input_size);
            let mut buffer = String::new();

            let mut input = input.chars().multipeek();

            let target_peak = 6;
            let did_read_anything = peek_until_n(target_peak, &mut input, &mut buffer);

            assert_eq!(did_read_anything, input_size >= 1, "i: {input_size}");
            assert_eq!(
                buffer.chars().count(),
                usize::min(input_size, target_peak),
                "i: {input_size}"
            );
        }
    }
}
