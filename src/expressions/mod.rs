pub use structs::{Expression, ExpressionNode};

mod iterators;
mod structs;
mod traits;

/// A utility function to quickly create a list of literal expressions.
pub fn vars<const K: usize>(names: [&str; K]) -> [Expression<String>; K] {
    std::array::from_fn(|i| ExpressionNode::Literal(names[i].to_string()).into())
}

/// A utility function to quickly create a single literal expression.
pub fn var<T: ToString>(name: T) -> Expression<String> {
    ExpressionNode::Literal(name.to_string()).into()
}

/// A utility function to quickly create a constant expression.
pub fn bool(value: bool) -> Expression<String> {
    ExpressionNode::Constant(value).into()
}
