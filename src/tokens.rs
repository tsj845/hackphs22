use std::fmt;

pub enum FuncName {
    Add,
    Sub,
    Mul,
    Div,
    Exp,
    Mod,
    Abs,
    Root,
    Invalid,
}

impl FuncName {
    pub fn from_str(string: &str) -> FuncName {
        return match string.to_lowercase().as_str() {
            "add" => Self::Add,
            "plus" => Self::Add,
	    "and" => Self::Add,
            "sub" => Self::Sub,
            "subtract" => Self::Sub,
            "minus" => Self::Sub,
            "mul" => Self::Mul,
	    "times" => Self::Mul,
            "multiply" => Self::Mul,
            "div" => Self::Div,
            "divide" => Self::Div,
	    "by" => Self::Div,
	    "over" => Self::Div,
            "pow" => Self::Exp,
            "power" => Self::Exp,
	    "to the" => Self::Exp,
            "mod" => Self::Mod,
            "modulo" => Self::Mod,
            "modulus" => Self::Mod,
            "remainder" => Self::Mod,
            "abs" => Self::Abs,
            "absval" => Self::Abs,
	    "absolute value" => Self::Abs,
            "root" => Self::Root,
            "sqrt" => Self::Root,
	    "square root" => Self::Root,
            _ => Self::Invalid,
        };
    }
}

pub enum Token {
    Function(FuncName),
    Literal(i32),
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple(match self {Token::Function(_)=>"Func",Token::Literal(_)=>"Lit"}).field(&match self {Token::Function(x)=>match x{&FuncName::Add=>"+".to_owned(),&FuncName::Sub=>"-".to_owned(),&FuncName::Mul=>"*".to_owned(),&FuncName::Div=>"/".to_owned(),&FuncName::Exp=>"^".to_owned(),&FuncName::Mod=>"%".to_owned(),&FuncName::Abs=>"abs(".to_owned(),&FuncName::Root=>"√".to_owned(),_=>"NOT SUPPORTED YET".to_owned()},Token::Literal(x)=>format!("{x}")}).finish()
    }
}
