// use std::rc::Rc;
// use std::cell::RefCell;
use crate::tokens::{Token, FuncName};

pub enum Operation{
    Num(i32),
    Grouping(Box<Operation>),
    Add(Box<Operation>,Box<Operation>),
    Subtract(Box<Operation>,Box<Operation>),
    Multiply(Box<Operation>,Box<Operation>),
    Divide(Box<Operation>,Box<Operation>),
    Pow(Box<Operation>,Box<Operation>),
    Root(Box<Operation>,Box<Operation>),
    Mod(Box<Operation>,Box<Operation>),
    Abs(Box<Operation>),
    Sin(Box<Operation>),
    Cos(Box<Operation>),
    Tan(Box<Operation>),
    Csc(Box<Operation>),
    Sec(Box<Operation>),
    Cot(Box<Operation>),
    //Arcsin(Box<Operation>),
    //Arccos(Box<Operation>),
    //Arctan(Box<Operation>),
}

fn double_paren(tokens: Vec<Token>) -> bool {
    let mut parens = 0;
    for token in tokens {
	if token == Token::GroupStart{
	    parens += 1;
	    if parens >= 2 {
		return true;
	    }
	}
    }
    return false;
}

impl Operation {
    pub fn new(tokens: Vec<Token>) -> Operation {
	let tokens = tokens.clone();
	let mut i: usize = 0;
	let l: usize = tokens.len();
	if l == 1 {
	    match &tokens[0] {
		Token::Literal(n) => {
		    return Operation::Num(*n);
		}
		_ => panic!("You used a function without any values!")
	    }
	}
	if tokens[0] == Token::GroupStart && tokens[l-1] == Token::GroupEnd && !double_paren(tokens.clone()) {
	    return Operation::Grouping(
		Box::new(Operation::new(tokens[1..l-1].to_vec()))
	    );
	}
	let mut in_parens = 0;
        loop {
            if i >= l {
                break;
            }
            let token: &Token = &tokens[i];
            match token{
                Token::Literal(_) => {},
                Token::Function(x) => {
		    if in_parens != 0 {i+=1;continue}
		    match x {
			FuncName::Add => {
			    return Operation::Add(
                                Box::new(Operation::new(tokens[0..i].to_vec() )),
                                Box::new(Operation::new(tokens[i+1..l].to_vec() )),
			    );
			},
			FuncName::Sub => {
			    return Operation::Subtract(
                                Box::new(Operation::new(tokens[0..i].to_vec() )),
                                Box::new(Operation::new(tokens[i+1..l].to_vec() )),
			    );
			},
			FuncName::Mul => {
			    return Operation::Multiply(
                                Box::new(Operation::new(tokens[0..i].to_vec() )),
                                Box::new(Operation::new(tokens[i+1..l].to_vec() )),
			    );

			},
			FuncName::Div => {
			    return Operation::Divide(
                                Box::new(Operation::new(tokens[0..i].to_vec() )),
                                Box::new(Operation::new(tokens[i+1..l].to_vec() )),
			    );
			}
			FuncName::Exp => {
			    return Operation::Pow(
                                Box::new(Operation::new(tokens[0..i].to_vec() )),
                                Box::new(Operation::new(tokens[i+1..l].to_vec() )),
			    );
			},
			FuncName::Mod => {
			    return Operation::Mod(
                                Box::new(Operation::new(tokens[0..i].to_vec() )),
                                Box::new(Operation::new(tokens[i+1..l].to_vec() )),
			    );
			},
			FuncName::Abs => {
			    return Operation::Abs(
				Box::new(Operation::new(tokens[i+1..l].to_vec()))
			    );
			},
			FuncName::Root => {
			    return Operation::Root(
                                Box::new(Operation::new(tokens[0..i].to_vec() )),
                                Box::new(Operation::new(tokens[i+1..l].to_vec() )),
			    );
			},
			FuncName::Sin => {
			    return Operation::Sin(
				Box::new(Operation::new(tokens[i+1..l].to_vec()))
			    );
			},
			FuncName::Cos => {
			    return Operation::Cos(
				Box::new(Operation::new(tokens[i+1..l].to_vec()))
			    );
			},
			FuncName::Tan => {
			    return Operation::Tan(
				Box::new(Operation::new(tokens[i+1..l].to_vec()))
			    );
			},
			FuncName::Sec => {
			    return Operation::Sec(
				Box::new(Operation::new(tokens[i+1..l].to_vec()))
			    );
			},
			FuncName::Csc => {
			    return Operation::Csc(
				Box::new(Operation::new(tokens[i+1..l].to_vec()))
			    );
			},
			FuncName::Cot => {
			    return Operation::Cot(
				Box::new(Operation::new(tokens[i+1..l].to_vec()))
			    );
			},
			FuncName::Invalid => {}
		    }
                },
		Token::GroupStart => in_parens += 1,
		Token::GroupEnd => in_parens -= 1,
            }
            i += 1;
        }
        return Operation::Num(0)
    }
}
