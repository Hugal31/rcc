mod expressions;
mod identifier;
mod instructions;
mod statements;
mod types;

pub use self::identifier::parse_identifier;

use c::Function;
use self::instructions::parse_return;
use self::types::parse_type;

type Result<T> = ::std::result::Result<T, ()>;

named!(parse_function<&str, Function>,
    ws!(
        do_parse!(
            parse_type >>
            name: parse_identifier >>
            char!('(') >> char!(')') >>
            char!('{') >>
            statements: many0!(parse_return) >>
            char!('}') >>
            (Function{name: name.to_owned(), statements: statements})
        )
    )
);

pub fn parse<'a>(input: &'a str) -> Result<Function> {
    let r = parse_function(input);

    r.to_result().map_err(|_| ())
}

#[test]
fn parse_simple_function() {
    use nom::IResult::Done;
    use c::instructions::unary::Return;

    let function = "int main() {\
    return 42;\
    }";
    assert_eq!(parse_function(function),
               Done("",
                    Function{name: "main".to_owned(),
                        statements: vec![Return{expression: 42}]}));
}
