#[derive(Clone,Copy,Debug,PartialEq)]
pub enum ExpressionOperation {
    Addition,
    Subtraction,
}

#[derive(Clone,Copy,Debug,PartialEq)]
pub enum TermOperation {
    Multiplication,
    Division,
//    Modulo
}
