use std::io::{Result, Write};
use c::Compile;
use super::Factor;
use super::binary::TermOperation;

#[derive(Debug,PartialEq)]
pub struct Term {
    pub factor: Factor,
    pub operations: Vec<(TermOperation, Factor)>,
}

impl Term {
    pub fn new(factor: Factor) -> Term {
        Term{
            factor,
            operations: vec!(),
        }
    }
}

impl Compile for Term {
    fn compile<O>(&self, output: &mut O) -> Result<()> where O: Write {
        self.factor.compile(output)
    }
}
