use std::fmt;

pub enum Operation {
    Add,
    Sub,
    Mul,
    Div,
    Exp,
    Mod,
}

pub enum Token {
    Function(Operation),
    Literal(i32),
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple(match self {Token::Function(_)=>"Func",Token::Literal(_)=>"Lit"}).field(&match self {Token::Function(x)=>match x{&Operation::Add=>"+".to_owned(),&Operation::Sub=>"-".to_owned(),&Operation::Mul=>"*".to_owned(),&Operation::Div=>"/".to_owned(),&Operation::Exp=>"^".to_owned(),&Operation::Mod=>"%".to_owned(),_=>"NOT SUPPORTED YET".to_owned()},Token::Literal(x)=>format!("{x}")}).finish()
    }
}