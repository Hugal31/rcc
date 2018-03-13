use std::str::FromStr;

use c::expressions::{BinaryOperator, Expression};
use super::fold_binary_expression;
use super::additive_expr::parse_additive_expression;

named!(pub parse_relational_expression<&str, Expression>,
    map!(do_parse!(
        additive_expression: parse_additive_expression >>
        operations: many0!(parse_relational_operation) >>
        (additive_expression, operations)
    ), fold_binary_expression)
);

named!(parse_relational_operation<&str, (BinaryOperator, Expression)>,
    ws!(do_parse!(
        operator: map_res!(
            alt!(tag!("<=") | tag!(">=") | tag!("<") | tag!(">")),
            BinaryOperator::from_str) >>
        additive_expression: parse_additive_expression >>
        (operator, additive_expression)
    ))
);

#[cfg(test)]
mod tests {
    use nom::IResult::Done;
    use c::Expression::*;
    use c::expressions::BinaryOperator;
    use super::*;

    #[test]
    fn test_parse_factor() {
        let expression = parse_relational_expression("42");
        assert_eq!(expression, Done("", Constant(42)));
    }

    #[test]
    fn test_less_or_equal() {
        let expression = parse_relational_expression("21 <= 42");
        assert_eq!(expression, Done("", BinOp(BinaryOperator::LessOrEqual,
        Box::new(Constant(21)),
        Box::new(Constant(42)))));
    }
}
