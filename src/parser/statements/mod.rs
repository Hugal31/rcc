use c::statement::Statement;

mod ret;

use self::ret::parse_return;

named!(pub parse_statement<&str, Statement>,
    ws!(do_parse!(
        inst: parse_return >>
        char!(';') >>
        (inst)
    ))
);

#[cfg(test)]
mod test {
    use super::*;
    use nom::IResult::Done;
    use c::expressions::unary::UnaryOperator::*;
    use c::Expression::*;
    use c::Statement::*;

    #[test]
    fn test_parse_statement() {
        assert_eq!(parse_statement("return 42;"), Done("", Return(Literal(42))));
        assert_eq!(parse_statement("return -42;"),
                   Done("", Return(Unary(Negation, Box::from(Literal(42))))));
    }
}
