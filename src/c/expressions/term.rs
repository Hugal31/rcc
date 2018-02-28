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
        self.factor.compile(output)?;
        for operation in &self.operations {
            output.write(b"push %eax\n")?;
            operation.1.compile(output)?;
            output.write(b"pop %ecx\n")?;
            match operation.0 {
                TermOperation::Multiplication => {
                    output.write(b"imul %ecx, %eax\n")?;
                },
                TermOperation::Division => {
                    output.write(br#"xchg %ecx, %eax
xor %edx, %edx
divl %ecx
"#)?;
                },
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use c::test::test_compile;

    #[test]
    fn test_compile_multiplication() {
        test_compile(Term{
            factor: Factor::Literal(42),
            operations: vec![(TermOperation::Multiplication, Factor::Literal(32))]
        }, r#"movl $42, %eax
push %eax
movl $32, %eax
pop %ecx
imul %ecx, %eax
"#);
    }

    #[test]
    fn test_compile_division() {
        test_compile(Term{
            factor: Factor::Literal(42),
            operations: vec![(TermOperation::Division, Factor::Literal(2))]
        }, r#"movl $42, %eax
push %eax
movl $2, %eax
pop %ecx
xchg %ecx, %eax
xor %edx, %edx
divl %ecx
"#);
    }
}
