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
mod tests {
    use super::*;
    use nom::IResult::Done;
    use c::Statement;
    use c::expressions::{AdditiveExpression, Term, Factor, RelationalExpression};

    #[test]
    fn test_parse_return() {
        assert_eq!(parse_return("return 42"), Done("", Statement::Return(RelationalExpression::new(AdditiveExpression::new(Term::new(Factor::Literal(42)))))));
    }
}
