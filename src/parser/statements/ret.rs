use nom::{ErrorKind, IResult, Needed};

use c::Statement;
use parser::expressions::parse_expression;
use parser::identifier::is_alphanumeric_or_underscore;

named!(pub parse_return<&str, Statement>,
    do_parse!(
        tag!("return") >>
        not!(continue_ident) >>
        expr: ws!(parse_expression) >>
        (Statement::Return(expr))
    )
);

fn continue_ident(input: &str) -> IResult<&str, &str> {
    if input.is_empty() {
        return IResult::Incomplete(Needed::Size(1));
    }

    if is_alphanumeric_or_underscore(input.chars().next().unwrap()) {
        IResult::Done(&input[1..], &input[..1])
    } else {
        IResult::Error(error_position!(ErrorKind::Custom(1), input))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::IResult::Done;
    use c::Statement;
    use c::Expression;

    #[test]
    fn test_parse_return() {
        assert_eq!(parse_return("return 42"), Done("", Statement::Return(Expression::Constant(42))));
        assert_eq!(parse_return("return\t42"), Done("", Statement::Return(Expression::Constant(42))));
        assert_eq!(parse_return("return\n42"), Done("", Statement::Return(Expression::Constant(42))));
    }

    #[test]
    fn test_parse_no_space() {
        let result = parse_return("return0");
        assert!(result.is_err(), "Result is {:?} and should be Err", result);
    }
}
