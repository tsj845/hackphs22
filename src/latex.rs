use crate::tree::Operation;

pub fn to_latex(tree: &Box<Operation>) -> String {
    let mut latex = String::new();
    match tree {
	Operation::Num(a) => {
	    latex = format!("{}", a);
	}
	Operation::Add(a, b) => {
	    latex = format!("{} + {}", to_latex(a), to_latex(b));
	},
	Operation::Subtract(a, b) => {
	    latex = format!("{} - {}", to_latex(a), to_latex(b));
	},
	Operation::Multiply(a, b) => {
	    latex = format!("{} \\times {}", to_latex(a), to_latex(b));
	},
	Operation::Divide(a, b) => {
	    latex = format!("\\frac{{{}}}{{{}}}", to_latex(a), to_latex(b));
	},
	Operation::Pow(a, b) => {
	    latex = format!("{}^{{{}}}", to_latex(a), to_latex(b));
	},
	Operation::Mod(a, b) => {
	    latex = format!("{} \\mod {}", to_latex(a), to_latex(b));
	}
    }
    return latex;
}
