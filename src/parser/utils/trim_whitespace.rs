use itertools::MultiPeek;
use std::str::Chars;

pub fn trim_whitespace_left(input: &mut MultiPeek<Chars>) {
    while let Some(c) = input.peek() {
        if !c.is_whitespace() {
            break;
        }

        input.next();
    }
    input.reset_peek();
}
