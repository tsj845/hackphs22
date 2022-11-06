use std::fmt;
use std::cmp::{Eq,PartialEq};
use std::f64;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum FuncName {
    Add,
    Sub,
    Mul,
    Div,
    Exp,
    Mod,
    Abs,
    Root,
    Sqrt,
    Sin,
    Cos,
    Tan,
    Csc,
    Sec,
    Cot,
    //Arcsin,
    //Arccos,
    //Arctan,
    Lim,

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
            "root" => Self::Sqrt,
            "sqrt" => Self::Sqrt,
	    "square root" => Self::Sqrt,
        "squareroot" => Self::Root,
        "cube root" => Self::Root,
        "cuberoot" => Self::Root,
	    "nd root" => Self::Root,
	    "ndroot" => Self::Root,
	    "rd root" => Self::Root,
	    "rdroot" => Self::Root,
	    "st root" => Self::Root,
	    "stroot" => Self::Root,
        "th root" => Self::Root,
        "throot" => Self::Root,
	    "sin" => Self::Sin,
	    "sine" => Self::Sin,
	    "cos" => Self::Cos,
	    "cosine" => Self::Cos,
	    "tan" => Self::Tan,
	    "tangent" => Self::Tan,
	    "sec" => Self::Sec,
	    "secant" => Self::Sec,
	    "csc" => Self::Csc,
	    "cosecant" => Self::Csc,
	    "cot" => Self::Cot,
	    "cotangent" => Self::Cot,
        "limit as x approaches" => Self::Lim,
        "lim as x approaches" => Self::Lim,
        _ => Self::Invalid,
        };
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum Token {
    Function(FuncName),
    Variable(char),
    Literal(f64),
    GroupStart,
    GroupEnd
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple(match self {
	    Token::Function(_) => "Func",
	    Token::Literal(_)=>"Lit",
        Token::Variable(_)=>"Var",
	    Token::GroupStart => "(",
	    Token::GroupEnd => ")",
	}).field(&match self {
	    Token::Function(x) => match x {
		&FuncName::Add => "+".to_owned(),
		&FuncName::Sub => "-".to_owned(),
		&FuncName::Mul => "*".to_owned(),
		&FuncName::Div => "/".to_owned(),
		&FuncName::Exp => "^".to_owned(),
		&FuncName::Mod => "%".to_owned(),
		&FuncName::Abs => "abs(".to_owned(),
		&FuncName::Root => "√".to_owned(),
		&FuncName::Sin => "sin(".to_owned(),
		&FuncName::Cos => "cos(".to_owned(),
		&FuncName::Tan => "tan(".to_owned(),
		&FuncName::Sec => "sec(".to_owned(),
		&FuncName::Csc => "csc(".to_owned(),
		&FuncName::Cot => "cot(".to_owned(),
		_ => "NOT SUPPORTED YET".to_owned()
	    },
	    Token::Literal(x) => format!("{x}"),
        Token::Variable(c) => format!("{c}"),
	    _ => "".to_owned()
	}).finish()
    }
}
