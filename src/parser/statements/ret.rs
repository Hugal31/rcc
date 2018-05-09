use c_ast::Statement;
use parser::expressions::parse_expression;
use parser::identifier::continue_ident;

named!(pub parse_return<&str, Statement>,
    do_parse!(
        tag!("return") >>
        not!(continue_ident) >>
        expr: ws!(parse_expression) >>
        char!(';') >>
        (Statement::Return(expr))
    )
);

#[cfg(test)]
mod tests {
    use super::*;
    use c_ast::Expression;
    use c_ast::Statement;
    use nom::IResult::Done;

    #[test]
    fn test_parse_return() {
        assert_eq!(
            parse_return("return 42;"),
            Done("", Statement::Return(Expression::Constant(42)))
        );
        assert_eq!(
            parse_return("return\t42;"),
            Done("", Statement::Return(Expression::Constant(42)))
        );
        assert_eq!(
            parse_return("return\n42;"),
            Done("", Statement::Return(Expression::Constant(42)))
        );
    }

    #[test]
    fn test_parse_no_space() {
        let result = parse_return("return0;");
        assert!(result.is_err(), "Result is {:?} and should be Err", result);
    }
}
