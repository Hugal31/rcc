use c::Statement;
use parser::expressions::parse_expression;

named!(pub parse_return<&str, Statement>,
    ws!(
        do_parse!(
            tag!("return") >>
            expr: parse_expression >>
            (Statement::Return(expr))
        )
    )
);

#[cfg(test)]
mod test {
    use super::*;
    use nom::IResult::Done;
    use c::{AdditiveExpression, Term, Factor, Statement};

    #[test]
    fn test_parse_return() {
        assert_eq!(parse_return("return 42"), Done("", Statement::Return(AdditiveExpression::new(Term::new(Factor::Literal(42))))));
    }
}
