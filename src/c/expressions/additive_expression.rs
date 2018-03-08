use std::io::{Result, Write};

use c::Compile;
use super::Term;
use super::binary::AdditiveOperator;

#[derive(Debug,PartialEq)]
pub struct AdditiveExpression {
    pub term: Term,
    pub operations: Vec<(AdditiveOperator, Term)>,
}

impl AdditiveExpression {
    #[allow(dead_code)]
    pub fn new(term: Term) -> AdditiveExpression {
        AdditiveExpression {
            term,
            operations: vec!(),
        }
    }
}

impl Compile for AdditiveExpression {
    fn compile<O>(&self, output: &mut O) -> Result<()> where O: Write {
        self.term.compile(output)?;
        for operation in &self.operations {
            output.write_all(b"push %eax\n")?;
            operation.1.compile(output)?;
            output.write_all(b"pop %ecx\n")?;
            match operation.0 {
                AdditiveOperator::Addition => {
                    output.write_all(b"addl %ecx, %eax\n")?;
                }
                AdditiveOperator::Subtraction => {
                    output.write_all(b"xchg %ecx, %eax\n")?;
                    output.write_all(b"subl %ecx, %eax\n")?;
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use c::Factor;
    use c::test::test_compile;

    #[test]
    fn test_compile_addition() {
        test_compile(AdditiveExpression {
            term: Term::new(Factor::Literal(42)),
            operations: vec![(AdditiveOperator::Addition, Term::new(Factor::Literal(32)))]
        }, r#"movl $42, %eax
push %eax
movl $32, %eax
pop %ecx
addl %ecx, %eax
"#);
    }

    #[test]
    fn test_compile_subtraction() {
        test_compile(AdditiveExpression {
            term: Term::new(Factor::Literal(42)),
            operations: vec![(AdditiveOperator::Subtraction,
                              Term::new(Factor::Literal(32)))]
        }, r#"movl $42, %eax
push %eax
movl $32, %eax
pop %ecx
xchg %ecx, %eax
subl %ecx, %eax
"#);
    }
}
