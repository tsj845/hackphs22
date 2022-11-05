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
