mod ret;

use c_ast::{Expression, Statement};

use self::ret::parse_return;
use super::expressions::parse_expression;
use super::identifier::{continue_ident, parse_identifier};
use super::types::parse_type;

named!(pub parse_block_item<&str, Statement>,
   alt!(parse_declaration | parse_statement)
);

named!(pub parse_statement<&str, Statement>,
    ws!(do_parse!(
        inst: alt!(parse_if
                   | parse_return
                   | parse_exp_statement) >>
        (inst)
    ))
);

named!(parse_if<&str, Statement>,
    ws!(do_parse!(
        tag!("if") >>
        char!('(') >>
        condition: parse_expression >>
        char!(')') >>
        then: parse_statement >>
        els: opt!(do_parse!(
            tag!("else") >>
            not!(continue_ident) >>
            stmt: parse_statement >>
            (stmt)
        )) >>
        (Statement::If {
            condition,
            then: Box::new(then),
            els: els.map(Box::new)
        })
    ))
);

named!(parse_exp_statement<&str, Statement>,
    do_parse!(
        exp: parse_expression >>
        char!(';') >>
        (Statement::Exp(exp))
    )
);

named!(parse_declaration<&str, Statement>,
    ws!(do_parse!(
        parse_type >>
        name: parse_identifier >>
        assignment: opt!(parse_assignment) >>
        char!(';') >>
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
    use nom::IResult::Done;

    use super::*;
    use c_ast::BinaryOperator::*;
    use c_ast::Expression::*;
    use c_ast::Statement::*;
    use c_ast::UnaryOperator::*;

    #[test]
    fn test_parse_if() {
        assert_eq!(
            parse_statement("if (1) return 1;;"),
            Done(";", If {
                condition: Constant(1),
                then: Box::new(Return(Constant(1))),
                els: None,
            })
        );
        assert_eq!(
            parse_statement("if (1) return 1; else return 0;"),
            Done("", If {
                condition: Constant(1),
                then: Box::new(Return(Constant(1))),
                els: Some(Box::new(Return(Constant(0)))),
            })
        );
    }

    #[test]
    fn test_parse_return() {
        assert_eq!(
            parse_statement("return 42;"),
            Done("", Return(Constant(42)))
        );
        assert_eq!(
            parse_statement("return -42;"),
            Done("", Return(UnOp(Negation, Box::from(Constant(42)))))
        );
    }

    #[test]
    fn test_parse_expression() {
        assert_eq!(
            parse_statement("42 + 3;"),
            Done(
                "",
                Exp(BinOp(
                    Addition,
                    Box::new(Constant(42)),
                    Box::new(Constant(3))
                ))
            )
        );
    }

    #[test]
    fn test_parse_declaration() {
        assert!(parse_block_item("int;").is_err());
        assert_eq!(
            parse_block_item("int a;"),
            Done("", Declare("a".to_owned(), None))
        );
        assert_eq!(
            parse_block_item("int b = 4;"),
            Done("", Declare("b".to_owned(), Some(Constant(4))))
        );
    }
}
