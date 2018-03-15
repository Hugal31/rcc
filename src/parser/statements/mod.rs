mod ret;

use c::Expression;
use c::Statement;

use super::expressions::parse_expression;
use super::identifier::parse_identifier;
use super::types::parse_type;
use self::ret::parse_return;

named!(pub parse_statement<&str, Statement>,
    ws!(do_parse!(
        inst: alt!(parse_return | parse_exp_statement | parse_declaration) >>
        char!(';') >>
        (inst)
    ))
);

named!(parse_exp_statement<&str, Statement>,
    do_parse!(
        exp: parse_expression >>
        (Statement::Exp(exp))
    )
);

named!(parse_declaration<&str, Statement>,
    ws!(do_parse!(
        parse_type >>
        name: parse_identifier >>
        assignment: opt!(parse_assignment) >>
        (Statement::Declare(name.to_owned(), assignment))
    ))
);

named!(parse_assignment<&str, Expression>,
    ws!(do_parse!(
        char!('=') >>
        exp: parse_expression >>
        (exp)
    ))
);

#[cfg(test)]
mod tests {
    use super::*;
    use nom::IResult::Done;
    use c::expressions::UnaryOperator::*;
    use c::expressions::BinaryOperator::*;
    use c::Statement::*;
    use c::Expression::*;

    #[test]
    fn test_parse_return() {
        assert_eq!(parse_statement("return 42;"), Done("", Return(Constant(42))));
        assert_eq!(parse_statement("return -42;"),
                   Done("", Return(UnOp(Negation, Box::from(Constant(42))))));
    }

    #[test]
    fn test_parse_expression() {
        assert_eq!(parse_statement("42 + 3;"), Done("", Exp(BinOp(Addition,
                                                                  Box::new(Constant(42)),
                                                                  Box::new(Constant(3))))));
    }

    #[test]
    fn test_parse_declaration() {
        assert!(parse_statement("int;").is_err());
        assert_eq!(parse_statement("int a;"), Done("", Declare("a".to_owned(), None)));
        assert_eq!(parse_statement("int b = 4;"), Done("", Declare("b".to_owned(), Some(Constant(4)))));
    }
}
