use std::str::FromStr;

use c::expressions::{AdditiveExpression, Term};
use c::expressions::binary::AdditiveOperator;
use super::term::parse_term;

named!(pub parse_additive_expression<&str, AdditiveExpression>,
    do_parse!(
        term: parse_term >>
        operations: many0!(parse_additive_operator) >>
        (AdditiveExpression{
            term,
            operations,
        })
    )
);

named!(parse_additive_operator<&str, ((AdditiveOperator, Term))>,
    ws!(do_parse!(
        operator: map_res!(alt!(tag!("+") | tag!("-")), AdditiveOperator::from_str) >>
        expr: parse_term >>
        (operator, expr)
    ))
);

#[cfg(test)]
mod tests {
    use super::*;
    use nom::IResult::Done;
    use c::expressions::binary::AdditiveOperator::*;
    use c::expressions::Term;
    use c::expressions::Factor::*;

    #[test]
    fn test_parse_factor() {
        let expression = parse_additive_expression("42");
        assert_eq!(expression, Done("", AdditiveExpression::new(Term::new(Literal(42)))));
    }

    #[test]
    fn test_parse_addition() {
        let expression = parse_additive_expression("42+23");
        let expression_with_space = parse_additive_expression("42 + 23");
        assert_eq!(expression, Done("", AdditiveExpression {term: Term::new(Literal(42)),
            operations: vec![(Addition, Term::new(Literal(23)))]}));
        assert_eq!(expression, expression_with_space);
    }

    #[test]
    fn test_parse_subtraction() {
        let expression = parse_additive_expression("42-23");
        assert_eq!(expression, Done("", AdditiveExpression {term: Term::new(Literal(42)),
            operations: vec![(Subtraction, Term::new(Literal(23)))]}))
    }
}
