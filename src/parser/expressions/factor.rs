use std::str::FromStr;

use nom::digit;

use c::expressions::{Factor, UnaryOperator};
use super::parse_expression;

named!(pub parse_factor<&str, Factor>,
    alt!(
        parse_int_literal
        | parse_expression_in_parenthesis
        | parse_unary_operation
    )
);

named!(parse_expression_in_parenthesis<&str, Factor>,
    ws!(do_parse!(
        char!('(') >>
        expr: parse_expression >>
        char!(')') >>
        (Factor::Expr(Box::new(expr)))
    ))
);

named!(parse_unary_operation<&str, Factor>,
    alt!(
        parse_negation
        | parse_local_negation
        | parse_bitwise
    )
);

named!(parse_negation<&str, Factor>,
    ws!(
        do_parse!(
            char!('-') >>
            expr: parse_factor >>
            (Factor::Unary(UnaryOperator::Negation, Box::from(expr)))
        )
    )
);

named!(parse_local_negation<&str, Factor>,
    ws!(
        do_parse!(
            char!('!') >>
            expr: parse_factor >>
            (Factor::Unary(UnaryOperator::LocalNegation, Box::from(expr)))
        )
    )
);

named!(parse_bitwise<&str, Factor>,
    ws!(
        do_parse!(
            char!('~') >>
            expr: parse_factor >>
            (Factor::Unary(UnaryOperator::Bitwise, Box::from(expr)))
        )
    )
);

named!(parse_int_literal<&str, Factor>, map!(map_res!(digit, i32::from_str), Factor::from));

#[cfg(test)]
mod tests {
    use super::*;
    use nom::IResult::Done;
    use c::expressions::{AdditiveExpression, Term, RelationalExpression};
    use c::expressions::Factor::*;

    #[test]
    fn test_parse_simple_factor() {
        assert_eq!(parse_factor("42"), Done("", Literal(42)));
    }

    #[test]
    fn test_parse_unary_operator() {
        assert_eq!(parse_factor("!42"),
                   Done("",
                        Unary(UnaryOperator::LocalNegation,
                                          Box::new(Literal(42)))));
        assert_eq!(parse_factor("!!42"),
                   Done("",
                        Unary(UnaryOperator::LocalNegation,
                                          Box::new(Unary(UnaryOperator::LocalNegation,Box::new(Literal(42)))))));
        assert_eq!(parse_factor("~42"),
                   Done("",
                        Unary(UnaryOperator::Bitwise,
                                     Box::new(Literal(42)))));
        assert_eq!(parse_factor("-42"),
                   Done("",
                        Unary(UnaryOperator::Negation,
                                          Box::new(Literal(42)))));
    }

    #[test]
    fn test_parse_int_literal() {
        assert_eq!(parse_int_literal("42"), Done("", Literal(42)));
    }

    #[test]
    fn test_parse_expression_in_parenthesis() {
        let factor = parse_factor("(42)");
        assert_eq!(factor, Done("",
                                Expr(Box::new(RelationalExpression::new(AdditiveExpression::new(Term::new(Literal(42))))))));

        let factor_with_space = parse_factor("( 42 )");
        assert_eq!(factor, factor_with_space);
    }
}
