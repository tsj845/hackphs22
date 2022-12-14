use crate::tokens::*;

// turns the "natural" language input and turns it into a string
// of tokens
pub fn parse_input(input: String) -> Vec<Token> {
    // the final return value
    let mut fin: Vec<Token> = Vec::new();
    // a vector of the characters in the natural language input
    let chars: Vec<char> = input.chars().collect();
    // current position
    let mut i: usize = 0;
    // the length of the input
    let l: usize = chars.len();
    let mut mkneg: f64 = 1.0;
    loop {
	// return if we have reached the end of the input
        if i >= l {
            break;
        }
	// ignore spaces
        if chars[i] == ' ' {i+=1;continue;}
	// create parenthesis
	if chars[i] == '(' {fin.push(Token::GroupStart);i+=1;continue;}
	if chars[i] == ')' {fin.push(Token::GroupEnd);i+=1;continue;}
	// checks if the character is a digit in base 10 and
	// creates a literal if it is
        if chars[i].is_digit(10) {
            let f: usize = i;
            while i < l && (chars[i].is_digit(10) || chars[i] == '.') {i += 1;}
            fin.push(Token::Literal(mkneg * String::from_iter(&chars[f..i]).parse::<f64>().unwrap()));
            mkneg = 1.0;
            i -= 1;
        }
	// create a function from natural language words
	else if chars[i].is_alphabetic() {
            let f: usize = i;
            while i < l && (chars[i].is_alphabetic() || chars[i] == '_' || (chars[i] == ' ' && chars[i+1].is_alphabetic())) {i += 1;}
            let string: String = String::from_iter(&chars[f..i]);
            let fnameres: FuncName = FuncName::from_str(&string);
            match fnameres {
                FuncName::Invalid => {for c in string.chars() {fin.push(Token::Variable(c))}},
                _ => {fin.push(Token::Function(fnameres));}
            };
            i -= 1;
        }
	// create a function from normal symbols
	else {
            match chars[i] {
                '+' => {
                    fin.push(Token::Function(FuncName::Add));
                },
                '-' => {
                    let l = fin.len();
                    if l == 0 || match &fin[l-1] {Token::Function(_)=>true,Token::GroupStart=>true,_=>false} {
                        mkneg = -1.0;
                    } else {
                        fin.push(Token::Function(FuncName::Sub));
                    }
                },
                '*' => {
                    fin.push(Token::Function(FuncName::Mul));
                },
                '/' => {
                    fin.push(Token::Function(FuncName::Div));
                },
                '^' => {
                    fin.push(Token::Function(FuncName::Exp));
                },
                '%' => {
                    fin.push(Token::Function(FuncName::Mod));
                },
                _ => {},
            };
        }
        i += 1;
    }
    return fin;
}
