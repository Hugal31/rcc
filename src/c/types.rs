use std::str::FromStr;

#[derive(Debug,PartialEq)]
pub enum Type {
    Void,
    Int,
}

impl FromStr for Type {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "void" => Ok(Type::Void),
            "int" => Ok(Type::Int),
            _ => Err(())
        }
    }
}
