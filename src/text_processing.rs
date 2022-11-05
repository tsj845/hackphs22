<<<<<<< HEAD
use crate::tree::Operation;

pub fn to_latex(tree: &Box<Operation>) -> String {
    let latex = String::new();
    match tree {
	Operation::Add(a, b) => {
	    latex += to_latex(a);
	    latex += "+";
	    latex += to_latex(b);
	},
	Operation::Subtract(a, b) => {
	    latex += to_latex(a);
	    latex += "-";
	    latex += to_latex(b);
	},
	Operation::Multiply(a, b) => {
	    latex += to_latex(a);
	    latex += " \times ";
	    latex += to_latex(b);
	},
	Operation::Divide(a, b) => {
	    latex += "\frac{";
	    latex += to_latex(a);
	    latex += "}{";
	    latex += to_latex(b);
	    latex += "}";
	},
	Operation::Pow(a, b) => {
	    latex += to_latex(a);
	    latex += "^{";
	    latex += to_latex(b);
	    latex += "}";
	},
    }
}
=======
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
>>>>>>> 539fe154b95edbd52fc81669367bbedca818f08b
