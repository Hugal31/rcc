use std::{fmt, result::Result as StdResult, str::FromStr};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BinaryOperator {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    //    Modulo,
    LessThan,
    GreaterThan,
    LessOrEqual,
    GreaterOrEqual,
    Equal,
    NotEqual,
    LogicalAnd,
    LogicalOr,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseBinaryOperationError {}

impl fmt::Display for ParseBinaryOperationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        "provided string was not a binary operator".fmt(f)
    }
}

impl FromStr for BinaryOperator {
    type Err = ParseBinaryOperationError;

    fn from_str(s: &str) -> StdResult<Self, Self::Err> {
        match s {
            "+" => Ok(BinaryOperator::Addition),
            "-" => Ok(BinaryOperator::Subtraction),
            "*" => Ok(BinaryOperator::Multiplication),
            "/" => Ok(BinaryOperator::Division),
            //"%"  => Ok(BinaryOperator::Modulo),
            "<" => Ok(BinaryOperator::LessThan),
            ">" => Ok(BinaryOperator::GreaterThan),
            "<=" => Ok(BinaryOperator::LessOrEqual),
            ">=" => Ok(BinaryOperator::GreaterOrEqual),
            "==" => Ok(BinaryOperator::Equal),
            "!=" => Ok(BinaryOperator::NotEqual),
            "&&" => Ok(BinaryOperator::LogicalAnd),
            "||" => Ok(BinaryOperator::LogicalOr),
            _ => Err(ParseBinaryOperationError {}),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::BinaryOperator::*;
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!("+".parse(), Ok(Addition));
        assert_eq!("-".parse(), Ok(Subtraction));
        assert_eq!("*".parse(), Ok(Multiplication));
        assert_eq!("/".parse(), Ok(Division));
        assert_eq!("<".parse(), Ok(LessThan));
        assert_eq!(">".parse(), Ok(GreaterThan));
        assert_eq!("<=".parse(), Ok(LessOrEqual));
        assert_eq!(">=".parse(), Ok(GreaterOrEqual));
        assert_eq!("&&".parse(), Ok(LogicalAnd));
        assert_eq!("||".parse(), Ok(LogicalOr));
        assert_eq!(
            "nop".parse::<BinaryOperator>(),
            Err(ParseBinaryOperationError {})
        );
    }
}
