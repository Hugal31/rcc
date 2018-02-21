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
    use c::Expression;

    #[test]
    fn test_parse_statement() {
        assert_eq!(parse_statement("return 42;"), Done("", Statement::Return(Expression::Literal(42))));
    }
}
