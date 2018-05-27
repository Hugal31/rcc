use std::io::Write;

use c_ast::expressions::*;
use compile::*;

impl EmitAsm for Expression {
    fn emit_asm<O>(&self, output: &mut O, ctx: &mut Context) -> Result<()>
    where
        O: Write,
    {
        match *self {
            Expression::Assign(ref name, ref exp) => {
                if !ctx.variable_is_defined(name) {
                    return Err(ErrorKind::UnknownVariable.into());
                }

                exp.emit_asm(output, ctx)?;

                let index = ctx.get_variable_index(name).unwrap();
                let offset = index + 4;
                output
                    .write_fmt(format_args!("movl %eax, -{}(%ebp)\n", offset))
                    .map_err(|e| e.into())
            }
            Expression::Var(ref name) => {
                if !ctx.variable_is_defined(name) {
                    return Err(ErrorKind::UnknownVariable.into());
                }

                let index = ctx.get_variable_index(name).unwrap();
                let offset = index + 4;
                output
                    .write_fmt(format_args!("movl -{}(%ebp), %eax\n", offset))
                    .map_err(|e| e.into())
            }
            Expression::Constant(i) => output
                .write_fmt(format_args!("movl ${}, %eax\n", i))
                .map_err(|e| e.into()),
            Expression::UnOp(ref op, ref expr) => {
                expr.emit_asm(output, ctx)?;
                op.emit_asm(output, ctx)
            }
            Expression::BinOp(ref op, ref lval, ref rval) => {
                lval.emit_asm(output, ctx)?;
                output.write_all(b"push %eax\n")?;
                rval.emit_asm(output, ctx)?;
                output.write_all(b"pop %ecx\n")?;
                op.emit_asm(output, ctx)
            }
            Expression::Conditional {
                ref condition,
                ref then,
                ref els,
            } => {
                let condition_idx = ctx.new_condition();
                let result = condition.emit_asm(output, ctx)
                    .and_then(|()| output.write_all(b"cmpl $0, %eax\n").map_err(|e| e.into()))
                    .and_then(|()| write!(output, "je _cond_{}_else\n", condition_idx).map_err(|e| e.into()))
                    .and_then(|()| then.emit_asm(output, ctx))
                    .and_then(|()| write!(output, "jmp _cond_{}_end\n", condition_idx).map_err(|e| e.into()));
                    result.and_then(|()| write!(output, "_cond_{}_else:\n", condition_idx).map_err(|e| e.into()))
                        .and_then(|()| els.emit_asm(output, ctx))
 .and_then(|()| write!(output, "_cond_{}_end:\n", condition_idx).map_err(|e| e.into()))},
        }
    }
}

impl EmitAsm for UnaryOperator {
    fn emit_asm<O>(&self, output: &mut O, _ctx: &mut Context) -> Result<()>
    where
        O: Write,
    {
        match *self {
            UnaryOperator::Negation => {
                output.write_all(b"neg %eax\n")?;
            }
            UnaryOperator::Bitwise => {
                output.write_all(b"not %eax\n")?;
            }
            UnaryOperator::LocalNegation => {
                output.write_all(b"cmpl $0, %eax\nmovl $0, %eax\nsete %al\n")?;
            }
        }

        Ok(())
    }
}

impl EmitAsm for BinaryOperator {
    // RValue should be in ECX, LValue in EAX
    fn emit_asm<O>(&self, output: &mut O, _ctx: &mut Context) -> Result<()>
    where
        O: Write,
    {
        match *self {
            BinaryOperator::Addition => output.write_all(b"addl %ecx, %eax\n"),
            BinaryOperator::Subtraction => output.write_all(
                b"xchg %ecx, %eax
subl %ecx, %eax\n",
            ),
            BinaryOperator::Multiplication => output.write_all(b"imul %ecx, %eax\n"),
            BinaryOperator::Division => output.write_all(
                b"xchg %ecx, %eax
xor %edx, %edx
divl %ecx\n",
            ),
            //    &BinaryOperator::Modulo => {},
            BinaryOperator::LessThan
            | BinaryOperator::GreaterThan
            | BinaryOperator::LessOrEqual
            | BinaryOperator::GreaterOrEqual
            | BinaryOperator::Equal
            | BinaryOperator::NotEqual => {
                output.write_all(b"cmpl %eax, %ecx\n")?;
                let opcode: &[u8] = match *self {
                    BinaryOperator::LessThan => b"setl",
                    BinaryOperator::GreaterThan => b"setg",
                    BinaryOperator::LessOrEqual => b"setle",
                    BinaryOperator::GreaterOrEqual => b"setge",
                    BinaryOperator::Equal => b"sete",
                    BinaryOperator::NotEqual => b"setne",
                    _ => unreachable!(),
                };
                output.write_all(opcode)?;
                output.write_all(b" %al\n")
            }
            BinaryOperator::LogicalAnd => output.write_all(
                b"cmpl $0, %ecx
setne %cl
cmpl $0, %eax
movl $0, %eax
setne %al
andb %cl, %al\n",
            ),
            BinaryOperator::LogicalOr => output.write_all(
                b"orl %ecx, %eax
movl $0, %eax
setne %al\n",
            ),
        }.map_err(|e| e.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use compile::tests::*;

    #[test]
    fn test_compile_bitwise() {
        test_compile(UnaryOperator::Bitwise, "not %eax\n");
    }

    #[test]
    fn test_compile_negation() {
        test_compile(UnaryOperator::Negation, "neg %eax\n");
    }

    #[test]
    fn test_compile_local_negation() {
        test_compile(
            UnaryOperator::LocalNegation,
            "cmpl $0, %eax\nmovl $0, %eax\nsete %al\n",
        );
    }

    #[test]
    fn test_compile_variable() {
        let ast = Expression::Var("a".to_owned());
        let mut ctx = Context::new();
        ctx.add_variable("a");

        test_compile_with_context(ast, &mut ctx, "movl -4(%ebp), %eax\n");
    }

    #[test]
    fn test_compile_unknown_variable() {
        let ast = Expression::Var("a".to_owned());
        let mut ctx = Context::new();
        let mut output = Vec::new();

        match ast.emit_asm(&mut output, &mut ctx) {
            Err(Error(ErrorKind::UnknownVariable, _)) => (),
            _ => assert!(false, "Should return an error"),
        }
    }
}
