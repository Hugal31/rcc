use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseTypeError {
}

impl fmt::Display for ParseTypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        "provided string was not a type`".fmt(f)
    }
}

#[derive(Debug,PartialEq)]
pub enum Type {
    Void,
    Int,
}

impl FromStr for Type {
    type Err = ParseTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "void" => Ok(Type::Void),
            "int" => Ok(Type::Int),
            _ => Err(ParseTypeError{})
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_from_str() {
        assert_eq!("void".parse(), Ok(Type::Void));
        assert_eq!("int".parse(), Ok(Type::Int));
        assert_eq!("nop".parse::<Type>(), Err(ParseTypeError{}));
    }
}
