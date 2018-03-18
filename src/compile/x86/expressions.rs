use std::io::Write;

use c_ast::expressions::*;
use compile::*;

impl Compile for Expression {
    fn compile<O>(&self, output: &mut O, scope: &mut Scope) -> Result<()> where O: Write {
        match *self {
            Expression::Assign(ref name, ref exp) => {
                if !scope.contains(name) {
                    return Err(ErrorKind::UnknownVariable.into());
                }

                exp.compile(output, scope)?;

                let index = scope.get_variable_index(name).unwrap();
                let offset = scope.get_size() - index;
                output.write_fmt(format_args!("movl %eax, -{}(%ebp)\n", offset))
                    .map_err(|e| e.into())
            },
            Expression::Var(ref name) => {
                if !scope.contains(name) {
                    return Err(ErrorKind::UnknownVariable.into());
                }

                let index = scope.get_variable_index(name).unwrap();
                let offset = scope.get_size() - index;
                output.write_fmt(format_args!("movl -{}(%ebp), %eax\n", offset))
                    .map_err(|e| e.into())
            },
            Expression::Constant(i) => output.write_fmt(format_args!("movl ${}, %eax\n", i))
                .map_err(|e| e.into()),
            Expression::UnOp(ref op, ref expr) => {
                expr.compile(output, scope)?;
                op.compile(output, scope)
            },
            Expression::BinOp(ref op, ref lval, ref rval) => {
                lval.compile(output, scope)?;
                output.write_all(b"push %eax\n")?;
                rval.compile(output, scope)?;
                output.write_all(b"pop %ecx\n")?;
                op.compile(output, scope)
            },
        }
    }
}

impl Compile for UnaryOperator {
    fn compile<O>(&self, output: &mut O, _scope: &mut Scope) -> Result<()> where O: Write {
        match *self {
            UnaryOperator::Negation => {
                output.write_all(b"neg %eax\n")?;
            },
            UnaryOperator::Bitwise => {
                output.write_all(b"not %eax\n")?;
            },
            UnaryOperator::LocalNegation => {
                output.write_all(b"cmpl $0, %eax\nmovl $0, %eax\nsete %al\n")?;
            },
        }

        Ok(())
    }
}

impl Compile for BinaryOperator {
    // RValue should be in ECX, LValue in EAX
    fn compile<O>(&self, output: &mut O, _scope: &mut Scope) -> Result<()> where O: Write {
        match *self {
            BinaryOperator::Addition => output.write_all(b"addl %ecx, %eax\n"),
            BinaryOperator::Subtraction => output.write_all(b"xchg %ecx, %eax
subl %ecx, %eax\n"),
            BinaryOperator::Multiplication => output.write_all(b"imul %ecx, %eax\n"),
            BinaryOperator::Division => output.write_all(b"xchg %ecx, %eax
xor %edx, %edx
divl %ecx\n"),
//    &BinaryOperator::Modulo => {},
            BinaryOperator::LessThan |
            BinaryOperator::GreaterThan |
            BinaryOperator::LessOrEqual |
            BinaryOperator::GreaterOrEqual |
            BinaryOperator::Equal |
            BinaryOperator::NotEqual => {
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
            },
            BinaryOperator::LogicalAnd => output.write_all(b"cmpl $0, %ecx
setne %cl
cmpl $0, %eax
movl $0, %eax
setne %al
andb %cl, %al\n"),
            BinaryOperator::LogicalOr => output.write_all(b"orl %ecx, %eax
movl $0, %eax
setne %al\n")
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
        test_compile(UnaryOperator::LocalNegation, "cmpl $0, %eax\nmovl $0, %eax\nsete %al\n");
    }

    #[test]
    fn test_compile_variable() {
        let variable = Variable::new("a");
        let ast = Expression::Var(variable.name.clone());
        let mut scope = Scope::new();
        scope.add_variable(variable);

        test_compile_with_scope(ast, &mut scope, "movl -4(%ebp), %eax\n");
    }

    #[test]
    fn test_compile_unknown_variable() {
        let ast = Expression::Var("a".to_owned());
        let mut scope = Scope::new();
        let mut output = Vec::new();

        match ast.compile(&mut output, &mut scope) {
            Err(Error(ErrorKind::UnknownVariable, _)) => (),
            _ => assert!(false, "Should return an error"),
        }
    }
}
