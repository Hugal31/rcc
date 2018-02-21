use c::instructions::Return;
use super::expressions::parse_expression;

named!(pub parse_return<&str, Return>,
    ws!(
        do_parse!(
            tag!("return") >>
            expr: parse_expression >>
            char!(';') >>
            (Return{expression: expr})
        )
    )
);

#[test]
fn test_parse_return() {
    use nom::IResult::Done;

    assert_eq!(parse_return("return 42;"), Done("", Return{expression: 42}));
}
