// use std::rc::Rc;
// use std::cell::RefCell;
use crate::tokens::{Token, FuncName};

pub enum Operation{
    Num(i32),

    Add(Box<Operation>,Box<Operation>),
    Subtract(Box<Operation>,Box<Operation>),
    Multiply(Box<Operation>,Box<Operation>),
    Divide(Box<Operation>,Box<Operation>),
    Pow(Box<Operation>,Box<Operation>),
    Root(Box<Operation>,Box<Operation>),
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

fn not_num(token: &Token) -> bool {//checks to see if the token entered is a number or not
    return match token { Token::Literal(_) => false, _=> true};
}

impl Operation {
    pub fn new(tokens: Vec<Token>) -> Operation {
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
        loop {
            if i >= l {
                break;
            }
            let token: &Token = &tokens[i];
            match token{
                Token::Literal(_) => {},
                Token::Function(x) => {
		    if i == 0 || not_num(&tokens[i-1]) || not_num(&tokens[i+1]) {panic!("You input the wrong thing!")} else {
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
			    FuncName::Abs => {},
			    FuncName::Root => {
				return Operation::Root(
                                    Box::new(Operation::new(tokens[0..i].to_vec() )),
                                    Box::new(Operation::new(tokens[i+1..l].to_vec() )),
				);
			    },
			    FuncName::Invalid => {}
			}
		    }
                },
                _ => {}
            }
            i += 1;
        }
        return Operation::Num(0)
    }
}
