use std::fmt;

pub enum FuncName {
    Add,
    Sub,
    Mul,
    Div,
    Exp,
    Mod,
}

pub enum Token {
    Function(FuncName),
    Literal(i32),
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple(match self {Token::Function(_)=>"Func",Token::Literal(_)=>"Lit"}).field(&match self {Token::Function(x)=>match x{&FuncName::Add=>"+".to_owned(),&FuncName::Sub=>"-".to_owned(),&FuncName::Mul=>"*".to_owned(),&FuncName::Div=>"/".to_owned(),&FuncName::Exp=>"^".to_owned(),&FuncName::Mod=>"%".to_owned(),_=>"NOT SUPPORTED YET".to_owned()},Token::Literal(x)=>format!("{x}")}).finish()
    }
}