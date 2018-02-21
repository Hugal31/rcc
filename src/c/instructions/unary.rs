use c::expressions::Expression;

#[derive(Debug, PartialEq)]
pub struct Return {
    pub expression: Expression,
}
