use crate::parser::TokenizerInput;

pub fn trim_whitespace_left(input: &mut TokenizerInput) {
    while let Some(c) = input.iterator.peek() {
        if !c.is_whitespace() {
            break;
        }

        input.next();
    }
    input.iterator.reset_peek();
}
