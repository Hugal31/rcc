mod additive_expr;
mod equality_expr;
mod relational_expr;
mod factor;
mod term;

use nom::IResult;

use c::expressions::{BinaryOperator, Expression};
use self::equality_expr::parse_equality_expression;

pub fn parse_expression(input: &str) -> IResult<&str, Expression> {
    parse_equality_expression(input)
}

pub fn fold_binary_expression(operations: (Expression, Vec<(BinaryOperator, Expression)>)) -> Expression {
    let expr = operations.0;
    let operations = operations.1;
    let iter = operations.iter().cloned();
    iter.fold(expr,
              |lval, (op, rval)| Expression::BinOp(op, Box::from(lval), Box::from(rval)))
}

#[cfg(test)]
mod tests {
    use c::Expression::*;
    use c::expressions::BinaryOperator::*;
    use super::*;

    #[test]
    fn test_fold_binary_expression() {
        // calculate 32 + 15 - 5
        let input = (Constant(32),
                     vec![(Addition, Constant(15)),
                          (Subtraction, Constant(5))]);
        let expected_result = BinOp(Subtraction,
                                    Box::new(BinOp(Addition,
                                                   Box::new(Constant(32)),
                                                   Box::new(Constant(15)))),
                                    Box::new(Constant(5)));
        assert_eq!(fold_binary_expression(input), expected_result);
    }
}
