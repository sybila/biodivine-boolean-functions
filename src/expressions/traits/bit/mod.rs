mod and;
mod not;
mod or;

#[cfg(test)]
mod tests {
    use crate::expressions::{var, Expression};
    use rstest::rstest;

    #[rstest]
    fn test_op_op_ok<F, F2, F3>(
        #[values(<Expression<String> as std::ops::BitAnd>::bitand, <Expression<String> as std::ops::BitOr>::bitor)]
        main_op: F,
        #[values(Expression::<String>::binary_and, Expression::<String>::binary_or)] lhs_op: F2,
        #[values(Expression::<String>::binary_and, Expression::<String>::binary_or)] rhs_op: F3,
    ) where
        F: Fn(Expression<String>, Expression<String>) -> Expression<String>,
        F2: Fn(&Expression<String>, &Expression<String>) -> Expression<String>,
        F3: Fn(&Expression<String>, &Expression<String>) -> Expression<String>,
    {
        let lhs = lhs_op(&var("a"), &var("b"));
        let rhs = rhs_op(&var("c"), &var("d"));

        let actual = main_op(lhs, rhs);

        let actual_variables_in_order = actual
            .to_string()
            .replace(|c| !['a', 'b', 'c', 'd'].contains(&c), "");
        let expected = "abcd";

        assert_eq!(actual_variables_in_order, expected);
    }
}
