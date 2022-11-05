use crate::tokens::*;

pub struct Processer {}

pub fn parse_input(input: String) -> Vec<Token> {
    let mut fin: Vec<Token> = Vec::new();
    let chars: Vec<char> = input.chars().collect();
    let mut i: usize = 0;
    let l: usize = chars.len();
    loop {
        if i >= l {
            break;
        }
        if chars[i].is_digit(10) {
            let f: usize = i;
            while i < chars.len() && chars[i].is_digit(10) {i += 1;}
            fin.push(Token::Literal(String::from_iter(&chars[f..i]).parse().unwrap()));
            i -= 1;
        } else if chars[i].is_alphabetic() {
            //
        } else {
            match chars[i] {
                '+' => {
                    fin.push(Token::Function(Operation::Add));
                },
                '-' => {
                    fin.push(Token::Function(Operation::Sub));
                },
                '*' => {
                    fin.push(Token::Function(Operation::Mul));
                },
                '/' => {
                    fin.push(Token::Function(Operation::Div));
                },
                '^' => {
                    fin.push(Token::Function(Operation::Exp));
                },
                '%' => {
                    fin.push(Token::Function(Operation::Mod));
                },
                _ => {},
            };
        }
        i += 1;
    }
    return fin;
}