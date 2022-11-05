use crate::tree::Operation;

pub fn to_latex(tree: &Box<Operation>) -> String {
    let mut latex = String::new();
    match tree {
	Operation::Num(a) => {
	    latex += format!("{}", a);
	}
	Operation::Add(a, b) => {
	    latex.push_str(&to_latex(a));
	    latex.push_str("+");
	    latex.push_str(&to_latex(b));
	},
	Operation::Subtract(a, b) => {
	    latex.push_str(&to_latex(a));
	    latex.push_str("-");
	    latex.push_str(&to_latex(b));
	},
	Operation::Multiply(a, b) => {
	    latex.push_str(&to_latex(a));
	    latex.push_str(" \times ");
	    latex.push_str(&to_latex(b));
	},
	Operation::Divide(a, b) => {
	    latex.push_str("\frac{");
	    latex.push_str(&to_latex(a));
	    latex.push_str("}{");
	    latex.push_str(&to_latex(b));
	    latex.push_str("}");
	},
	Operation::Pow(a, b) => {
	    latex.push_str(&to_latex(a));
	    latex.push_str("^{");
	    latex.push_str(&to_latex(b));
	    latex.push_str("}");
	},
	Operation::Mod(a, b) => {
	    latex.push_str(&to_latex(a));
	    latex.push_str(" \mod ");
	    latex.push_str(&to_latex(b));
	}
    }
    return latex;
}
