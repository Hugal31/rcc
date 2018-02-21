use nom::IResult::Done;

use c::instructions::unary::Return;
use super::expressions::parse_i32;

named!(pub parse_return<&str, Return>,
    ws!(
        do_parse!(
            tag!("return") >>
            int: parse_i32 >>
            char!(';') >>
            (Return{expression: int})
        )
    )
);

#[test]
fn test_parse_return() {
    assert_eq!(parse_return("return 42;"), Done("", Return{expression: 42}));
}
