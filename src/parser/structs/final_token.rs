#[derive(PartialEq, Debug)]
pub enum FinalToken {
    And,
    Or,
    Not,
    ConstantTrue,
    ConstantFalse,
    Literal(String),
    Parentheses(Vec<FinalToken>),
}
