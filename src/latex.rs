use crate::tree::Operation;

pub fn to_latex(tree: Box<Operation>) -> String {
    match *tree {
	Operation::Num(a) => {
	    return format!("{}", a);
	}
	Operation::Add(a, b) => {
	    return format!("{} + {}", to_latex(a), to_latex(b));
	},
	Operation::Subtract(a, b) => {
	    return format!("{} - {}", to_latex(a), to_latex(b));
	},
	Operation::Multiply(a, b) => {
	    return format!("{} \\times {}", to_latex(a), to_latex(b));
	},
	Operation::Divide(a, b) => {
	    return format!("\\frac{{{}}}{{{}}}", to_latex(a), to_latex(b));
	},
	Operation::Pow(a, b) => {
	    return format!("{}^{{{}}}", to_latex(a), to_latex(b));
	},
	Operation::Root(a, b) => {
	    return format!("\\sqrt[{}]{}", to_latex(a), to_latex(b));
	},
	Operation::Mod(a, b) => {
	    return format!("{} \\mod {}", to_latex(a), to_latex(b));
	},
	Operation::Abs(a) => {
	    return format!("|{}|", to_latex(a));
	}
    }
}
