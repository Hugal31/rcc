use std::str::FromStr;

use c::expressions::{EqualityExpression, EqualityOperator, RelationalExpression};
use super::relational_expr::parse_relational_expression;

named!(pub parse_equality_expression<&str, EqualityExpression>,
    do_parse!(
        relational_expression: parse_relational_expression >>
        operations: many0!(parse_equality_operation) >>
        (EqualityExpression{
            relational_expression,
            operations,
        })
    )
);

named!(parse_equality_operation<&str, (EqualityOperator, RelationalExpression)>,
   ws!(do_parse!(
        operator: map_res!(alt!(tag!("==") | tag!("!=")), EqualityOperator::from_str) >>
        relational_expression: parse_relational_expression >>
        (operator, relational_expression)
   ))
);

#[cfg(test)]
mod tests {
    use nom::IResult::Done;
    use c::expressions::{EqualityExpression, EqualityOperator, Factor, RelationalExpression};
    use super::*;

    #[test]
    fn test_parse_factor() {
        let expression = parse_equality_expression("42");
        assert_eq!(expression, Done("", EqualityExpression::from(Factor::Literal(42))));
    }

    #[test]
    fn test_less_or_equal() {
        let expression = parse_equality_expression("21 != 42");
        let twenty_one = RelationalExpression::from(Factor::Literal(21));
        let fourth_two = RelationalExpression::from(Factor::Literal(42));
        assert_eq!(expression, Done("", EqualityExpression{
            relational_expression: twenty_one,
            operations: vec![(EqualityOperator::NotEqual, fourth_two)],
        }));
    }
}
