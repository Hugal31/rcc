use c_ast::Expression;

#[derive(Debug, PartialEq)]
pub enum Statement {
    Return(Expression),
    Declare(String, Option<Expression>),
    Exp(Expression),
}
