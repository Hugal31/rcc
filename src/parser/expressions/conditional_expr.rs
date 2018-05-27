use super::parse_expression;
use super::logical_or_expr::parse_logical_or_expression;
use c_ast::Expression;

named!(pub parse_conditional_expression<&str, Expression>,
    ws!(do_parse!(
        condition: parse_logical_or_expression >>
        branches: opt!(do_parse!(
            char!('?') >>
            then: parse_expression >>
            char!(':') >>
            els: parse_conditional_expression >>
            (then, els)
        )) >>
        (if let Some((then, els)) = branches {
            Expression::Conditional {
                condition: Box::new(condition),
                then: Box::new(then),
                els: Box::new(els),
            }
        } else {
            condition
        })
    ))
);

#[cfg(test)]
mod tests {
    use super::*;
    use c_ast::BinaryOperator::*;
    use c_ast::Expression::*;
    use nom::IResult::Done;

    #[test]
    fn test_parse_factor() {
        let expression = parse_conditional_expression("42;");
        assert_eq!(expression, Done(";", Constant(42)));
    }

    #[test]
    fn test_parse_conditional() {
        let expression = parse_conditional_expression("4 == 4 ? 1 : 0;");
        assert_eq!(expression, Done(";", Conditional {
            condition: Box::new(BinOp(Equal, Box::new(Constant(4)), Box::new(Constant(4)))),
            then: Box::new(Constant(1)),
            els: Box::new(Constant(0)),
        }));
    }
}
