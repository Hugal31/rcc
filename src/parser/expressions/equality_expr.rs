use std::str::FromStr;

use super::fold_binary_expression;
use super::relational_expr::parse_relational_expression;
use c_ast::{BinaryOperator, Expression};

named!(pub parse_equality_expression<&str, Expression>,
    map!(do_parse!(
        relational_expression: parse_relational_expression >>
        operations: many0!(parse_equality_operation) >>
        (relational_expression, operations)
    ), fold_binary_expression)
);

named!(parse_equality_operation<&str, (BinaryOperator, Expression)>,
   ws!(do_parse!(
        operator: map_res!(alt!(tag!("==") | tag!("!=")), BinaryOperator::from_str) >>
        relational_expression: parse_relational_expression >>
        (operator, relational_expression)
   ))
);

#[cfg(test)]
mod tests {
    use super::*;
    use c_ast::expressions::BinaryOperator;
    use c_ast::Expression::*;
    use nom::IResult::Done;

    #[test]
    fn test_parse_factor() {
        let expression = parse_equality_expression("42");
        assert_eq!(expression, Done("", Constant(42)));
    }

    #[test]
    fn test_less_or_equal() {
        let expression = parse_equality_expression("21 != 42");
        assert_eq!(
            expression,
            Done(
                "",
                BinOp(
                    BinaryOperator::NotEqual,
                    Box::new(Constant(21)),
                    Box::new(Constant(42))
                )
            )
        );
    }
}
