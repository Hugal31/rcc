use c_ast::{BinaryOperator, Expression};
use super::fold_binary_expression;
use super::logical_and_expr::parse_logical_and_expression;

named!(pub parse_logical_or_expression<&str, Expression>,
    map!(do_parse!(
        expression: parse_logical_and_expression >>
        operations: many0!(parse_logical_or_operation) >>
        (expression, operations)
    ), fold_binary_expression)
);

named!(parse_logical_or_operation<&str, (BinaryOperator, Expression)>,
   ws!(do_parse!(
        tag!("||") >>
        expression: parse_logical_and_expression >>
        (BinaryOperator::LogicalOr, expression)
   ))
);

#[cfg(test)]
mod tests {
    use nom::IResult::Done;
    use c_ast::Expression::*;
    use c_ast::BinaryOperator;
    use super::*;

    #[test]
    fn test_parse_factor() {
        let expression = parse_logical_or_expression("42");
        assert_eq!(expression, Done("", Constant(42)));
    }

    #[test]
    fn test_logical_or() {
        let expression = parse_logical_or_expression("21 || 42");
        assert_eq!(expression, Done("", BinOp(BinaryOperator::LogicalOr,
                                              Box::new(Constant(21)),
                                              Box::new(Constant(42)))));
    }
}
