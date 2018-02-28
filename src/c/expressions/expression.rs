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
        self.term.compile(output)?;
        for operation in &self.operations {
            output.write(b"push %eax\n")?;
            operation.1.compile(output)?;
            output.write(b"pop %ecx\n")?;
            match operation.0 {
                ExpressionOperation::Addition => {
                    output.write(b"addl %ecx, %eax\n")?;
                }
                ExpressionOperation::Subtraction => {
                    output.write(b"xchg %ecx, %eax\n")?;
                    output.write(b"subl %ecx, %eax\n")?;
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
        test_compile(Expression{
            term: Term::new(Factor::Literal(42)),
            operations: vec![(ExpressionOperation::Addition, Term::new(Factor::Literal(32)))]
        }, r#"movl $42, %eax
push %eax
movl $32, %eax
pop %ecx
addl %ecx, %eax
"#);
    }

    #[test]
    fn test_compile_subtraction() {
        test_compile(Expression{
            term: Term::new(Factor::Literal(42)),
            operations: vec![(ExpressionOperation::Subtraction,
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
