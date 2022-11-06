use std::f32::consts::PI;

// use std::rc::Rc;
// use std::cell::RefCell;
use crate::tokens::{Token, FuncName};

#[derive(PartialEq)]
pub enum Operation{
	Var(char),
    Num(f64),
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

	Lim(Box<Operation>,Box<Operation>),
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
		},
		Token::Variable(c) => {
			return Operation::Var(*c);
		},
		_ => panic!("You used a function without any values!")
	    }
	}
	if tokens[0] == Token::GroupStart && tokens[l-1] == Token::GroupEnd && !double_paren(tokens.clone()) {
	    return Operation::Grouping(
		Box::new(Operation::new(tokens[1..l-1].to_vec()))
	    );
	}
	let mut in_parens = 0;
	let mut pemdos_level = 0;
        loop {
            if i >= l {
		if pemdos_level == -1 {
                    break;
		} else {
		    pemdos_level += 1;
		    i = 0;
		}
            }
            let token: &Token = &tokens[i];
            match token{
                Token::Literal(_) => {},
				Token::Variable(_)=>{},
                Token::Function(x) => {
		    if in_parens != 0 {i+=1;continue}
		    match x {
			// pemdos level 0
			FuncName::Add => {
			    if pemdos_level != 0 {i+=1;continue}
			    return Operation::Add(
                                Box::new(Operation::new(tokens[0..i].to_vec() )),
                                Box::new(Operation::new(tokens[i+1..l].to_vec() )),
			    );
			},
			FuncName::Sub => {
			    if pemdos_level != 0 {i+=1;continue}
			    return Operation::Subtract(
                                Box::new(Operation::new(tokens[0..i].to_vec() )),
                                Box::new(Operation::new(tokens[i+1..l].to_vec() )),
			    );
			},
			// pemdos level 1
			FuncName::Mul => {
			    if pemdos_level != 1 {i+=1;continue}
			    return Operation::Multiply(
                                Box::new(Operation::new(tokens[0..i].to_vec() )),
                                Box::new(Operation::new(tokens[i+1..l].to_vec() )),
			    );

			},
			FuncName::Mod => {
			    if pemdos_level != 1 {i+=1;continue}
			    return Operation::Mod(
                                Box::new(Operation::new(tokens[0..i].to_vec() )),
                                Box::new(Operation::new(tokens[i+1..l].to_vec() )),
			    );
			},
			// pemdos level 2
			FuncName::Div => {
			    if pemdos_level != 2 {i+=1;continue}
			    return Operation::Divide(
                                Box::new(Operation::new(tokens[0..i].to_vec() )),
                                Box::new(Operation::new(tokens[i+1..l].to_vec() )),
			    );
			}
			// pemdos level 3
			FuncName::Exp => {
			    if pemdos_level != 3 {i+=1;continue}
			    return Operation::Pow(
                                Box::new(Operation::new(tokens[0..i].to_vec() )),
                                Box::new(Operation::new(tokens[i+1..l].to_vec() )),
			    );
			},
			FuncName::Root => {
			    if pemdos_level != 3 {i+=1;continue}
			    return Operation::Root(
                                Box::new(Operation::new(tokens[0..i].to_vec() )),
                                Box::new(Operation::new(tokens[i+1..l].to_vec() )),
			    );
			},
			FuncName::Sqrt => {
			    if pemdos_level != 3 {i+=1;continue}
			    return Operation::Root(
				Box::new(Operation::Num(2.0)),
				Box::new(Operation::new(tokens[i+1..l].to_vec()))
			    );
			}
			// pemdos level 4
			
			// pemdos level 5
			FuncName::Abs => {
			    if pemdos_level != 5 {i+=1;continue}
			    return Operation::Abs(
				Box::new(Operation::new(tokens[i+1..l].to_vec()))
			    );
			},
			FuncName::Sin => {
			    if pemdos_level != 5 {i+=1;continue}
			    return Operation::Sin(
				Box::new(Operation::new(tokens[i+1..l].to_vec()))
			    );
			},
			FuncName::Cos => {
			    if pemdos_level != 5 {i+=1;continue}
			    return Operation::Cos(
				Box::new(Operation::new(tokens[i+1..l].to_vec()))
			    );
			},
			FuncName::Tan => {
			    if pemdos_level != 5 {i+=1;continue}
			    return Operation::Tan(
				Box::new(Operation::new(tokens[i+1..l].to_vec()))
			    );
			},
			FuncName::Sec => {
			    if pemdos_level != 5 {i+=1;continue}
			    return Operation::Sec(
				Box::new(Operation::new(tokens[i+1..l].to_vec()))
			    );
			},
			FuncName::Csc => {
			    if pemdos_level != 5 {i+=1;continue}
			    return Operation::Csc(
				Box::new(Operation::new(tokens[i+1..l].to_vec()))
			    );
			},
			FuncName::Cot => {
			    if pemdos_level != 5 {i+=1;continue}
			    return Operation::Cot(
				Box::new(Operation::new(tokens[i+1..l].to_vec()))
			    );
			},
			FuncName::Lim => {
				if pemdos_level !=5 {i+=1;continue}

				return Operation::Lim(
					Box::new(Operation::new(vec![tokens[i+1]] )),
					Box::new(Operation::new(tokens[i+5..l].to_vec() ))
				);
			}
			FuncName::Invalid => {}
		    }
                },
		Token::GroupStart => in_parens += 1,
		Token::GroupEnd => in_parens -= 1,
            }
            i += 1;
        }
        return Operation::Num(0.0)
    }
    pub fn calculate(&self) -> f32 {
	match self {
	    Operation::Num(a) => return *a as f32,
	    Operation::Grouping(a) => return a.calculate(),
	    Operation::Add(a, b) => return a.calculate()+b.calculate(),
	    Operation::Subtract(a, b) => return a.calculate()-b.calculate(),
	    Operation::Multiply(a, b) => return a.calculate()*b.calculate(),
	    Operation::Divide(a, b) => return a.calculate()/b.calculate(),
	    Operation::Pow(a, b) => return a.calculate().powf(b.calculate()),
	    Operation::Root(a, b) => return b.calculate().powf(1.0/a.calculate()),
	    Operation::Mod(a, b) => return a.calculate()%b.calculate(),
		Operation::Sin(a) => return a.calculate().sin(),
		Operation::Cos(a) => return a.calculate().cos(),
		Operation::Tan(a) => {
			if a.calculate() == PI/2.0{panic!("undefined");}
			return a.calculate().tan();
		},
		Operation::Csc(a) => {
			if a.calculate() == 0.0{println!("undefined");panic!();}
			return 1.0/a.calculate().sin();
		},
		Operation::Sec(a) => {
			if a.calculate() == PI/2.0{println!("undefined");panic!();}
			return 1.0/a.calculate().cos();
		},
		Operation::Cot(a) => {
			if a.calculate() == 0.0{println!("undefined");panic!();}
			return 1.0/a.calculate().tan();
		},
		Operation::Abs(a) => return a.calculate().abs(),
	    _ => panic!("Not implemented yet!"),
	}
    }
}
