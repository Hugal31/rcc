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

pub fn parse(input: &str) -> Function {
    let r = parse_function(input);

    r.to_result().expect("Parsing error")
}

#[cfg(test)]
mod test {
    use super::*;
    use nom::IResult::Done;
    use c::{Expression, Term, Factor, Statement};

    #[test]
    fn parse_simple_function() {

        let function = "int main() {\
    return 42;\
    }";
        assert_eq!(parse_function(function),
                   Done("",
                        Function {
                            name: "main".to_owned(),
                            statements: vec![Statement::Return(Expression::new(Term::new(Factor::Literal(42))))]
                        }));
    }
}
