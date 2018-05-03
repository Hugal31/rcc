use std::str::FromStr;

use super::fold_binary_expression;
use super::term::parse_term;
use c_ast::{BinaryOperator, Expression};

named!(pub parse_additive_expression<&str, Expression>,
    map!(do_parse!(
        term: parse_term >>
        operations: many0!(parse_additive_operator) >>
        (term, operations)
    ), fold_binary_expression)
);

named!(parse_additive_operator<&str, (BinaryOperator, Expression)>,
    ws!(do_parse!(
        operator: map_res!(alt!(tag!("+") | tag!("-")), BinaryOperator::from_str) >>
        expr: parse_term >>
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
        let expression = parse_additive_expression("42");
        assert_eq!(expression, Done("", Constant(42)));
    }

    #[test]
    fn test_parse_addition() {
        let expression = parse_additive_expression("42+23");
        let expression_with_space = parse_additive_expression("42 + 23");
        assert_eq!(
            expression,
            Done(
                "",
                BinOp(
                    BinaryOperator::Addition,
                    Box::new(Constant(42)),
                    Box::new(Constant(23))
                )
            )
        );
        assert_eq!(expression, expression_with_space);
    }

    #[test]
    fn test_parse_subtraction() {
        let expression = parse_additive_expression("42-23");
        assert_eq!(
            expression,
            Done(
                "",
                BinOp(
                    BinaryOperator::Subtraction,
                    Box::new(Constant(42)),
                    Box::new(Constant(23))
                )
            )
        );
    }
}
