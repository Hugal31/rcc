mod factor;
mod term;

use c::{Expression, Term};
use c::expressions::binary::ExpressionOperation;
use self::term::parse_term;

named!(pub parse_expression<&str, Expression>,
    do_parse!(
        term: parse_term >>
        operations: many0!(parse_expr_operation) >>
        (Expression{
            term: term,
            operations: operations,
        })
    )
);

named!(parse_expr_operation<&str, ((ExpressionOperation, Term))>,
    alt!(parse_plus_operation | parse_minus_operation)
);

named!(parse_plus_operation<&str, (ExpressionOperation, Term)>,
    ws!(do_parse!(
        char!('+') >>
        expr: parse_term >>
        ((ExpressionOperation::Addition, expr))
    ))
);

named!(parse_minus_operation<&str, (ExpressionOperation, Term)>,
    ws!(do_parse!(
        char!('-') >>
        expr: parse_term >>
        ((ExpressionOperation::Subtraction, expr))
    ))
);

#[cfg(test)]
mod test {
    use super::*;
    use nom::IResult::Done;
    use c::expressions::binary::ExpressionOperation::*;
    use c::Term;
    use c::Factor::*;

    #[test]
    fn test_parse_factor() {
        let expression = parse_expression("42");
        assert_eq!(expression, Done("", Expression::new(Term::new(Literal(42)))));
    }

    #[test]
    fn test_parse_addition() {
        let expression = parse_expression("42+23");
        let expression_with_space = parse_expression("42 + 23");
        assert_eq!(expression, Done("", Expression{term: Term::new(Literal(42)),
            operations: vec![(Addition, Term::new(Literal(23)))]}));
        assert_eq!(expression, expression_with_space);
    }

    #[test]
    fn test_parse_subtraction() {
        let expression = parse_expression("42-23");
        assert_eq!(expression, Done("", Expression{term: Term::new(Literal(42)),
            operations: vec![(Subtraction, Term::new(Literal(23)))]}))
    }
}
