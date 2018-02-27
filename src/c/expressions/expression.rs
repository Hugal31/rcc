use std::io::{Result, Write};
use c::Compile;
use super::Term;
use super::binary::ExpressionOperation;

#[derive(Debug,PartialEq)]
pub struct Expression {
    pub term: Term,
    pub operations: Vec<(ExpressionOperation, Term)>,
}

impl Expression {
    pub fn new(term: Term) -> Expression {
        Expression{
            term,
            operations: vec!(),
        }
    }
}

impl Compile for Expression {
    fn compile<O>(&self, output: &mut O) -> Result<()> where O: Write {
        self.term.compile(output)
    }
}
