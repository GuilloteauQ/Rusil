use crate::tokens::Expr;

#[derive(Debug, Clone)]
pub(crate) struct Function {
    name: String,
    args: Vec<String>,
    core: Box<Expr>,
}

impl Function {
    pub(crate) fn new(name: String, args: Vec<String>, core: Box<Expr>) -> Self {
        Function { name, args, core }
    }

    pub(crate) fn get_args(&self) -> Vec<String> {
        self.args.clone()
    }

    pub(crate) fn get_core(&self) -> Box<Expr> {
        self.core.clone()
    }
}
