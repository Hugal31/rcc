use std::str::FromStr;
use c::{Factor, Term};
use c::expressions::binary::TermOperation;
use super::factor::parse_factor;

named!(pub parse_term<&str, Term>,
    do_parse!(
        factor: parse_factor >>
        operations: many0!(parse_term_operation) >>
        (Term{
            factor,
            operations,
        })
    )
);

named!(parse_term_operation<&str, (TermOperation, Factor)>,
    ws!(do_parse!(
        operator: map_res!(alt!(tag!("*") | tag!("/")), TermOperation::from_str) >>
        expr: parse_factor >>
        (operator, expr)
    ))
);

#[cfg(test)]
mod tests {
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
