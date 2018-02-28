
use c::{Factor, Term};
use c::expressions::binary::TermOperation;
use super::factor::parse_factor;

named!(pub parse_term<&str, Term>,
    do_parse!(
        factor: parse_factor >>
        operations: many0!(parse_term_operation) >>
        (Term{
            factor: factor,
            operations: operations,
        })
    )
);

named!(parse_term_operation<&str, ((TermOperation, Factor))>,
    alt!(parse_mul_operation | parse_div_operation)
);

named!(parse_mul_operation<&str, (TermOperation, Factor)>,
    ws!(do_parse!(
        char!('*') >>
        expr: parse_factor >>
        ((TermOperation::Multiplication, expr))
    ))
);

named!(parse_div_operation<&str, (TermOperation, Factor)>,
    ws!(do_parse!(
        char!('/') >>
        expr: parse_factor >>
        ((TermOperation::Division, expr))
    ))
);

#[cfg(test)]
mod test {
    use super::*;
    use nom::IResult::Done;
    use c::expressions::binary::TermOperation::*;
    use c::Term;
    use c::Factor::*;

    #[test]
    fn test_parse_factor() {
        let term = parse_term("42");
        assert_eq!(term, Done("", Term::new(Literal(42))));
    }

    #[test]
    fn test_parse_multiplication() {
        let term = parse_term("42*23");
        let term_with_space = parse_term("42 * 23");
        assert_eq!(term, Done("", Term{
            factor: Literal(42),
            operations: vec![(Multiplication, Literal(23))],
        }));
        assert_eq!(term, term_with_space);
    }
}
