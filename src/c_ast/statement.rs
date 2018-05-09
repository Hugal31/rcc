use c_ast::Expression;

#[derive(Debug, PartialEq)]
pub enum Statement {
    If {
        condition: Expression,
        then: Box<Statement>,
        els: Option<Box<Statement>>,
    },
    Return(Expression),
    Declare(String, Option<Expression>),
    Exp(Expression),
}
