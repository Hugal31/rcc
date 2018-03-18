use c_ast::Statement;

#[derive(Debug, PartialEq)]
pub struct Function {
    pub name: String,
    pub statements: Vec<Statement>,
}
