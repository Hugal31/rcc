use std::str::FromStr;

use super::factor::parse_factor;
use super::fold_binary_expression;
use c_ast::{BinaryOperator, Expression};

named!(pub parse_term<&str, Expression>,
    map!(do_parse!(
        factor: parse_factor >>
        operations: many0!(parse_term_operation) >>
        (factor, operations)
    ), fold_binary_expression)
);

named!(parse_term_operation<&str, (BinaryOperator, Expression)>,
    ws!(do_parse!(
        operator: map_res!(alt!(tag!("*") | tag!("/")), BinaryOperator::from_str) >>
        expr: parse_factor >>
        (operator, expr)
    ))
);

#[cfg(test)]
mod tests {
    use super::*;
    use c_ast::BinaryOperator;
    use c_ast::Expression::*;
    use nom::IResult::Done;

    #[test]
    fn test_parse_factor() {
        let term = parse_term("42");
        assert_eq!(term, Done("", Constant(42)));
    }

    #[test]
    fn test_parse_multiplication() {
        let term = parse_term("42*23");
        let term_with_space = parse_term("42 * 23");
        assert_eq!(
            term,
            Done(
                "",
                BinOp(
                    BinaryOperator::Multiplication,
                    Box::new(Constant(42)),
                    Box::new(Constant(23))
                )
            )
        );
        assert_eq!(term, term_with_space);
    }
}
