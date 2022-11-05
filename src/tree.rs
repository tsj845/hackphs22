// use std::rc::Rc;
// use std::cell::RefCell;
use crate::tokens::Token;

pub enum Operation{
    Num(i32),

    Add(Box<Operation>,Box<Operation>),
    Subtract(Box<Operation>,Box<Operation>),
    Multiply(Box<Operation>,Box<Operation>),
    Divide(Box<Operation>,Box<Operation>),
    Pow(Box<Operation>,Box<Operation>),
    Mod(Box<Operation>,Box<Operation>),

    /*
    Abs(Box<Operation>),

    Sin(Box<Operation>),
    Cos(Box<Operation>),
    Tan(Box<Operation>),
    Csc(Box<Operation>),
    Sec(Box<Operation>),
    Cot(Box<Operation>),
    Arcsin(Box<Operation>),
    Arccos(Box<Operation>),
    Arctan(Box<Operation>),
    */
}

impl Operation {
    pub fn new() -> Operation {
        Operation::Num(0)
    }
}

pub fn make(mut toks: Vec<Token>) -> Box<Operation> {
    return Box::new(Operation::Num(0));
}