use crate::table::display_formatted::{
    FALSE_CAPITALIZED_WORD, FALSE_CHARACTER, FALSE_NUMBER, FALSE_WORD, TRUE_CAPITALIZED_WORD,
    TRUE_CHARACTER, TRUE_NUMBER, TRUE_WORD,
};

pub fn string_to_bool(input: &str) -> Option<bool> {
    match input {
        FALSE_NUMBER | FALSE_CHARACTER | FALSE_WORD | FALSE_CAPITALIZED_WORD => Some(false),
        TRUE_NUMBER | TRUE_CHARACTER | TRUE_WORD | TRUE_CAPITALIZED_WORD => Some(true),
        _ => None,
    }
}
