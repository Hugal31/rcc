use std::str::FromStr;

use nom::digit;

use super::parse_expression;
use c_ast::{Expression, UnaryOperator};
use parser::identifier::parse_identifier;

named!(pub parse_factor<&str, Expression>,
    alt!(
        parse_int_literal
        | parse_variable
        | parse_expression_in_parenthesis
        | parse_unary_operation
    )
);

named!(parse_expression_in_parenthesis<&str, Expression>,
    ws!(do_parse!(
        char!('(') >>
        expr: parse_expression >>
        char!(')') >>
        (expr)
    ))
);

named!(parse_unary_operation<&str, Expression>,
    ws!(do_parse!(
        op: map_res!(alt!(tag!("-") | tag!("~") | tag!("!")), UnaryOperator::from_str) >>
        expr: parse_factor >>
        (Expression::UnOp(op, Box::new(expr)))
    ))
);

named!(parse_int_literal<&str, Expression>,
    do_parse!(
        constant: map_res!(digit, i32::from_str) >>
        (Expression::Constant(constant))
    )
);

named!(parse_variable<&str, Expression>,
    do_parse!(
        name: parse_identifier >>
        (Expression::Var(name.to_owned()))
    )
);

#[cfg(test)]
mod tests {
    use super::*;
    use c_ast::expressions::UnaryOperator;
    use c_ast::Expression::*;
    use nom::IResult::Done;

    #[test]
    fn test_parse_simple_factor() {
        assert_eq!(parse_factor("42"), Done("", Expression::Constant(42)));
    }

    #[test]
    fn test_parse_unary_operator() {
        assert_eq!(
            parse_factor("!42"),
            Done(
                "",
                UnOp(UnaryOperator::LocalNegation, Box::new(Constant(42)))
            )
        );
        assert_eq!(
            parse_factor("!!42"),
            Done(
                "",
                UnOp(
                    UnaryOperator::LocalNegation,
                    Box::new(UnOp(UnaryOperator::LocalNegation, Box::new(Constant(42))))
                )
            )
        );
        assert_eq!(
            parse_factor("~42"),
            Done("", UnOp(UnaryOperator::Bitwise, Box::new(Constant(42))))
        );
        assert_eq!(
            parse_factor("-42"),
            Done("", UnOp(UnaryOperator::Negation, Box::new(Constant(42))))
        );
    }

    #[test]
    fn test_parse_int_literal() {
        assert_eq!(parse_factor("42"), Done("", Constant(42)));
    }

    #[test]
    fn test_parse_variable() {
        assert_eq!(parse_factor("a"), Done("", Var("a".to_owned())));
    }

    #[test]
    fn test_parse_expression_in_parenthesis() {
        let factor = parse_factor("(42)");
        assert_eq!(factor, Done("", Constant(42)));

        let factor_with_space = parse_factor("( 42 )");
        assert_eq!(factor, factor_with_space);
    }
}
