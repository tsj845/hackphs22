use crate::tokens::*;

pub fn parse_input(input: String) -> Vec<Token> {
    let mut fin: Vec<Token> = Vec::new();
    let chars: Vec<char> = input.chars().collect();
    let mut i: usize = 0;
    let l: usize = chars.len();
    loop {
        if i >= l {
            break;
        }
        if chars[i] == ' ' {i+=1;continue;}
        if chars[i].is_digit(10) {
            let f: usize = i;
            while i < l && chars[i].is_digit(10) {i += 1;}
            fin.push(Token::Literal(String::from_iter(&chars[f..i]).parse().unwrap()));
            i -= 1;
        } else if chars[i].is_alphabetic() {
            let f: usize = i;
            while i < l && (chars[i].is_alphabetic() || chars[i] == '_') {i += 1;}
            let string: String = String::from_iter(&chars[f..i]);
            let fnameres: FuncName = FuncName::from_str(&string);
            match fnameres {
                FuncName::Invalid => {},
                _ => {fin.push(Token::Function(fnameres));}
            };
            i -= 1;
        } else {
            match chars[i] {
                '+' => {
                    fin.push(Token::Function(FuncName::Add));
                },
                '-' => {
                    fin.push(Token::Function(FuncName::Sub));
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
