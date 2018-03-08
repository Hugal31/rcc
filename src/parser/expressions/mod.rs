mod additive_expr;
mod relational_expr;
mod factor;
mod term;

use nom::IResult;

use c::expressions::Expression;
use self::relational_expr::parse_relational_expression;

pub fn parse_expression(input: &str) -> IResult<&str, Expression> {
    parse_relational_expression(input)
}
