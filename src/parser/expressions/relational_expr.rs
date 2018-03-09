use std::str::FromStr;

use c::expressions::{AdditiveExpression, RelationalExpression};
use c::expressions::binary::RelationalOperator;
use super::additive_expr::parse_additive_expression;

named!(pub parse_relational_expression<&str, RelationalExpression>,
    do_parse!(
        additive_expression: parse_additive_expression >>
        operations: many0!(parse_relational_operation) >>
        (RelationalExpression{
            additive_expression,
            operations,
        })
    )
);

named!(parse_relational_operation<&str, (RelationalOperator, AdditiveExpression)>,
    ws!(do_parse!(
        operator: map_res!(
            alt!(tag!("<=") | tag!(">=") | tag!("<") | tag!(">")),
            RelationalOperator::from_str) >>
        additive_expression: parse_additive_expression >>
        (operator, additive_expression)
    ))
);

#[cfg(test)]
mod tests {
    use nom::IResult::Done;
    use c::expressions::{AdditiveExpression, Factor, Term, RelationalExpression};
    use c::expressions::binary::RelationalOperator;
    use super::*;

    #[test]
    fn test_parse_factor() {
        let expression = parse_relational_expression("42");
        assert_eq!(expression, Done("", RelationalExpression::from(Factor::Literal(42))));
    }

    #[test]
    fn test_less_or_equal() {
        let expression = parse_relational_expression("21 <= 42");
        let twenty_one = AdditiveExpression::new(Term::new(Factor::Literal(21)));
        let fourth_two = AdditiveExpression::new(Term::new(Factor::Literal(42)));
        assert_eq!(expression, Done("", RelationalExpression{
            additive_expression: twenty_one,
            operations: vec![(RelationalOperator::LessOrEqual, fourth_two)],
        }));
    }
}
