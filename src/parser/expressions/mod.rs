mod factor;

use c::expressions::{Expression, Term};
use self::factor::parse_factor;

named!(pub parse_expression<&str, Expression>,
    do_parse!(
        factor: parse_factor >>
        (Expression{
            term: Term{
                factor: factor,
                operations: vec!(),
            },
            operations: vec!(),
        })
    )
);

#[cfg(test)]
mod test {
    use super::*;
    use nom::IResult::Done;
    use c::Term;
    use c::Factor::*;

    #[test]
    fn test_parse_factor() {
        let expression = parse_expression("42");
        assert_eq!(expression, Done("", Expression::new(Term::new(Literal(42)))));
    }
}
