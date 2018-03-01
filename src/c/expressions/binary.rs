use std::fmt;
use std::str::FromStr;

#[derive(Clone,Copy,Debug,PartialEq)]
pub enum ExpressionOperation {
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

impl FromStr for ExpressionOperation {
    type Err = ParseExpressionOperationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(ExpressionOperation::Addition),
            "-" => Ok(ExpressionOperation::Subtraction),
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_expression_operation() {
        assert_eq!("+".parse(), Ok(ExpressionOperation::Addition));
        assert_eq!("-".parse(), Ok(ExpressionOperation::Subtraction));
        assert_eq!("nop".parse::<ExpressionOperation>(), Err(ParseExpressionOperationError{}));
    }

    #[test]
    fn test_parse_term_operation() {
        assert_eq!("*".parse(), Ok(TermOperation::Multiplication));
        assert_eq!("/".parse(), Ok(TermOperation::Division));
        assert_eq!("nop".parse::<TermOperation>(), Err(ParseTermOperationError{}));
    }
}
