use std::fmt;
use std::str::FromStr;

#[derive(Clone,Copy,Debug,PartialEq)]
pub enum AdditiveOperator {
    Addition,
    Subtraction,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseExpressionOperationError {}

impl fmt::Display for ParseExpressionOperationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        "provided string was not a '+' or a '-'".fmt(f)
    }
}

impl FromStr for AdditiveOperator {
    type Err = ParseExpressionOperationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(AdditiveOperator::Addition),
            "-" => Ok(AdditiveOperator::Subtraction),
            _ => Err(ParseExpressionOperationError{}),
        }
    }
}

#[derive(Clone,Copy,Debug,PartialEq)]
pub enum TermOperation {
    Multiplication,
    Division,
//    Modulo
}

impl FromStr for TermOperation {
    type Err = ParseTermOperationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "*" => Ok(TermOperation::Multiplication),
            "/" => Ok(TermOperation::Division),
            _ => Err(ParseTermOperationError{}),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseTermOperationError {}

impl fmt::Display for ParseTermOperationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        "provided string was not a '*' or a '/'".fmt(f)
    }
}

#[derive(Clone,Copy,Debug,PartialEq)]
pub enum RelationalOperator {
    LessThan,
    GreaterThan,
    LessOrEqual,
    GreaterOrEqual,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseRelationalOperatorError {}

impl fmt::Display for ParseRelationalOperatorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        "provided string was not '<', '>', ' <=' or '>='".fmt(f)
    }
}

impl FromStr for RelationalOperator {
    type Err = ParseRelationalOperatorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "<"  => Ok(RelationalOperator::LessThan),
            ">"  => Ok(RelationalOperator::GreaterThan),
            "<=" => Ok(RelationalOperator::LessOrEqual),
            ">=" => Ok(RelationalOperator::GreaterOrEqual),
            _    => Err(ParseRelationalOperatorError{}),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_expression_operation() {
        assert_eq!("+".parse(), Ok(AdditiveOperator::Addition));
        assert_eq!("-".parse(), Ok(AdditiveOperator::Subtraction));
        assert_eq!("nop".parse::<AdditiveOperator>(), Err(ParseExpressionOperationError{}));
    }

    #[test]
    fn test_parse_term_operation() {
        assert_eq!("*".parse(), Ok(TermOperation::Multiplication));
        assert_eq!("/".parse(), Ok(TermOperation::Division));
        assert_eq!("nop".parse::<TermOperation>(), Err(ParseTermOperationError{}));
    }

    #[test]
    fn test_parse_relational_operator() {
        assert_eq!("<".parse(), Ok(RelationalOperator::LessThan));
        assert_eq!(">".parse(), Ok(RelationalOperator::GreaterThan));
        assert_eq!("<=".parse(), Ok(RelationalOperator::LessOrEqual));
        assert_eq!(">=".parse(), Ok(RelationalOperator::GreaterOrEqual));
    }
}
