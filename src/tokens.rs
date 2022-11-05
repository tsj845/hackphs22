use std::fmt;

pub enum Token {
    Function(u8),
    Literal(i32),
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple(match self {Token::Function(_)=>"Func",Token::Literal(_)=>"Lit"}).field(&match self {Token::Function(x)=>match x{0=>"+".to_owned(),1=>"-".to_owned(),2=>"*".to_owned(),3=>"/".to_owned(),_=>"INVALID".to_owned()},Token::Literal(x)=>format!("{x}")}).finish()
    }
}