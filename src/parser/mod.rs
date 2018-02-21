mod expressions;
mod instructions;
mod types;

use nom::{alphanumeric, digit};
use nom::IResult::Done;

use c::instructions::unary::Return;
use self::instructions::parse_return;
use self::types::parse_type;

type Result<T> = ::std::result::Result<T, ()>;

#[derive(Debug, PartialEq)]
pub struct Function {
    pub name: String,
    pub instructions: Vec<Return>,
}

named!(parse_function<&str, Function>,
    ws!(
        do_parse!(
            parse_type >>
            name: alphanumeric >>
            char!('(') >> char!(')') >>
            char!('{') >>
            inst: parse_return >>
            char!('}') >>
            (Function{name: name.to_owned(), instructions: vec![inst]})
        )
    )
);

pub fn parse<'a>(input: &'a str) -> Result<Function> {
    let r = parse_function(input);

    r.to_result().map_err(|_| ())
}

#[test]
fn parse_simple_function() {
    let function = "int main() {\
    return 42;\
    }";
    assert_eq!(parse_function(function),
               Done("",
                    Function{name: "main".to_owned(),
                        instructions: vec![Return{expression: 42}]}));
}
