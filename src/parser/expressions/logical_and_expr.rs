use c_ast::{BinaryOperator, Expression};
use super::fold_binary_expression;
use super::equality_expr::parse_equality_expression;

named!(pub parse_logical_and_expression<&str, Expression>,
    map!(do_parse!(
        expression: parse_equality_expression >>
        operations: many0!(parse_logical_and_operation) >>
        (expression, operations)
    ), fold_binary_expression)
);

named!(parse_logical_and_operation<&str, (BinaryOperator, Expression)>,
   ws!(do_parse!(
        tag!("&&") >>
        expression: parse_equality_expression >>
        (BinaryOperator::LogicalAnd, expression)
   ))
);

#[cfg(test)]
mod tests {
    use nom::IResult::Done;
    use c_ast::expressions::BinaryOperator;
    use c_ast::Expression::*;
    use super::*;

    #[test]
    fn test_parse_factor() {
        let expression = parse_logical_and_expression("42");
        assert_eq!(expression, Done("", Constant(42)));
    }

    #[test]
    fn test_logical_and() {
        let expression = parse_logical_and_expression("21 && 42");
        assert_eq!(expression, Done("", BinOp(BinaryOperator::LogicalAnd,
                                              Box::new(Constant(21)),
                                              Box::new(Constant(42)))));
    }
}
