mod additive_expr;
mod factor;
mod term;

use nom::IResult;

use c::expressions::AdditiveExpression;
use self::additive_expr::parse_additive_expression;

pub fn parse_expression(input: &str) -> IResult<&str, AdditiveExpression> {
    parse_additive_expression(input)
}
