use std::io::{Result, Write};
use c::Compile;

#[derive(Clone,Copy,Debug,PartialEq)]
pub enum UnaryOperator {
    Negation,
    Bitwise,
    LocalNegation,
}

impl Compile for UnaryOperator {
    fn compile<O>(&self, output: &mut O) -> Result<()> where O: Write {
        match self {
            &UnaryOperator::Negation => {
                output.write(b"neg %eax\n")?;
            },
            &UnaryOperator::Bitwise => {
                output.write(b"not %eax\n")?;
            },
            &UnaryOperator::LocalNegation => {
                output.write(b"cmpl $0, %eax\nmovl $0, %eax\nsete %al\n")?;
            },
        }

        Ok(())
    }
}
