mod binary;
mod unary;

pub use self::binary::*;
pub use self::unary::*;

#[derive(Clone,Debug,PartialEq)]
pub enum Expression {
    Assign(String, Box<Expression>),
    Var(String),
    Constant(i32),
    UnOp(UnaryOperator, Box<Expression>),
    BinOp(BinaryOperator, Box<Expression>, Box<Expression>),
}
