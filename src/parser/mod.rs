mod expressions;
mod identifier;
mod statements;
mod types;

pub use self::identifier::parse_identifier;

use c::Function;
use self::statements::parse_statement;
use self::types::parse_type;

named!(parse_function<&str, Function>,
    ws!(
        do_parse!(
            parse_type >>
            name: parse_identifier >>
            char!('(') >> char!(')') >>
            char!('{') >>
            statements: many0!(parse_statement) >>
            char!('}') >>
            (Function{name: name.to_owned(), statements})
        )
    )
);

pub fn parse(input: &str) -> Result<Function, ::nom::Err> {
    let r = parse_function(input);

    r.to_result()
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::IResult::Done;
    use c::Statement;
    use c::expressions::{RelationalExpression, AdditiveExpression, Term, Factor};

    #[test]
    fn parse_simple_function() {

        let function = "int main() {\
    return 42;\
    }";
        assert_eq!(parse_function(function),
                   Done("",
                        Function {
                            name: "main".to_owned(),
                            statements: vec![Statement::Return(RelationalExpression::new(AdditiveExpression::new(Term::new(Factor::Literal(42)))))]
                        }));
    }
}
