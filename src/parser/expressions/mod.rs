mod additive_expr;
mod equality_expr;
mod relational_expr;
mod factor;
mod term;

use nom::IResult;

use c::expressions::Expression;
use self::equality_expr::parse_equality_expression;

pub fn parse_expression(input: &str) -> IResult<&str, Expression> {
    parse_equality_expression(input)
}
