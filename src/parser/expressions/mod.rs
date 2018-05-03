mod additive_expr;
mod equality_expr;
mod factor;
mod logical_and_expr;
mod logical_or_expr;
mod relational_expr;
mod term;

use self::logical_or_expr::parse_logical_or_expression;
use super::identifier::parse_identifier;
use c_ast::{BinaryOperator, Expression};

named!(pub parse_expression<&str, Expression>,
    alt!(parse_assignment | parse_logical_or_expression)
);

named!(parse_assignment<&str, Expression>,
    ws!(do_parse!(
        name: parse_identifier >>
        char!('=') >>
        exp: parse_expression >>
        (Expression::Assign(name.to_owned(), Box::new(exp)))
    ))
);

pub fn fold_binary_expression(
    operations: (Expression, Vec<(BinaryOperator, Expression)>),
) -> Expression {
    let expr = operations.0;
    let operations = operations.1;
    let iter = operations.iter().cloned();
    iter.fold(expr, |lval, (op, rval)| {
        Expression::BinOp(op, Box::from(lval), Box::from(rval))
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use c_ast::BinaryOperator::*;
    use c_ast::Expression::*;
    use nom::IResult::*;

    #[test]
    fn test_fold_binary_expression() {
        // calculate 32 + 15 - 5
        let input = (
            Constant(32),
            vec![(Addition, Constant(15)), (Subtraction, Constant(5))],
        );
        let expected_result = BinOp(
            Subtraction,
            Box::new(BinOp(
                Addition,
                Box::new(Constant(32)),
                Box::new(Constant(15)),
            )),
            Box::new(Constant(5)),
        );
        assert_eq!(fold_binary_expression(input), expected_result);
    }

    #[test]
    fn parse_assigment() {
        assert_eq!(
            parse_expression("a = 4"),
            Done("", Assign("a".to_owned(), Box::new(Constant(4))))
        );
    }
}
