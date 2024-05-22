[![Codecov](https://img.shields.io/codecov/c/github/sybila/biodivine-boolean-functions?style=flat-square)](https://codecov.io/gh/sybila/biodivine-boolean-functions)
[![GitHub issues](https://img.shields.io/github/issues/sybila/biodivine-boolean-functions?style=flat-square)](https://github.com/sybila/biodivine-boolean-functions/issues)
[![GitHub last commit](https://img.shields.io/github/last-commit/sybila/biodivine-boolean-functions?style=flat-square)](https://github.com/sybila/biodivine-boolean-functions/commits/master)

# Biodivine/Boolean Functions

This is a Rust library with Python PyO3 bindings for Boolean function manipulation.

You can represent Boolean functions as

- Expression trees,
- Truth tables, or
- Reduced, Ordered Binary Decision Diagrams.

## Local usage from Rust

1. Set up the dependency
   ```toml
   [dependencies]
   biodivine-boolean-functions = { git = "https://github.com/sybila/biodivine-boolean-functions" }
   ```

2. Use from Rust!
   ```rust
   use biodivine_boolean_functions::bdd::Bdd;
   use biodivine_boolean_functions::expressions::{bool, var, Expression, ExpressionNode};
   use biodivine_boolean_functions::table::display_formatted::{TableBooleanFormatting, TableStyle};
   use biodivine_boolean_functions::table::TruthTable;
   use biodivine_boolean_functions::traits::BooleanFunction;
   use std::collections::BTreeMap;
   use std::str::FromStr;
   
   // ###############
   // # Expressions #
   // ###############
   
   // Create an expression with convenience functions
   let expression = var("a") & !var("b") & bool(true);
   // Create an expression by nesting nodes
   let other_expression = Expression::n_ary_and(&[
     ExpressionNode::Literal("a".to_string()).into(),
     Expression::negate(&ExpressionNode::Literal("b".to_string()).into()),
     ExpressionNode::Constant(true).into(),
   ]);
   // Create an expression by parsing it from a string
   let parsed_expression =
     Expression::from_str("a and not b and true").expect("This input string should be valid.");
   
   // The three expressions are semantically (and syntactically) equivalent
   assert!(expression.is_equivalent(&other_expression));
   assert!(expression.is_equivalent(&parsed_expression));
   
   let dnf = expression.to_dnf();
   println!("{}", dnf); // (a & !(b) & true)
   
   // ################
   // # Truth Tables #
   // ################
   
   let table: TruthTable<String> = expression.into();
   let table = table.restrict(&BTreeMap::from([("a".to_string(), true)]));
   
   println!(
     "{}",
     table.to_string_formatted(
         TableStyle::Markdown,
         TableBooleanFormatting::Word,
         TableBooleanFormatting::Word
     )
   );
   // | b     | result |
   // |-------|--------|
   // | false | true   |
   // | true  | false  |
   
   // ############################
   // # Binary Decision Diagrams #
   // ############################
   
   let bdd: Bdd<String> = table
     .try_into()
     .expect("Table should not have more than 2^16 variables.");
   println!("{:?}", bdd.sat_point()) // Some([false])
   ```

## Local usage from Python

1. Create a Python virtual environment

2. Run the following to set up PyO3 with `maturin` as per [the docs](https://pyo3.rs/v0.21.2/getting-started).
   ```shell
   $ bash ./scripts/local_dev.sh
   ```

3. Use from Python!
   ```python
   import biodivine_boolean_functions as bbf

   expression = bbf.var("a") & ~bbf.var("b") & bbf.bool(True)
   expression_nested = bbf.Expression.mk_and_n_ary([
   bbf.Expression.mk_literal("a"),
   bbf.Expression.mk_not(bbf.Expression.mk_literal("b")),
   bbf.Expression.mk_constant(True)
   ])
   expression_parsed = bbf.Expression("a and not b and true")
   
   assert expression.semantic_eq(expression_parsed)
   assert expression.semantic_eq(expression_nested)
   
   dnf = expression.to_dnf()
   print(dnf)
   
   table = expression.to_table()
   restricted = table.restrict({"a": True})
   print(restricted.to_string_formatted(bbf.TableStyle.Markdown, bbf.TableBooleanFormatting.Word))
   
   bdd = expression.to_bdd()
   print(bdd.sat_point())
   ```

