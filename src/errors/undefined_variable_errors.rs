use crate::tokens::Expr;
use crate::types::Type;
use std;
use std::error::Error;
use std::fmt;

pub(crate) struct UndefVarError {
    message: String,
}

impl UndefVarError {
    pub fn new(var_name: String, faulty_expression: String) -> Self {
        let message = format!(
            "\n>>> Undefined Variable:\n\n\t\x1B[31m{}\x1B[39m\n\n\tVariable \"{}\" not found\n",
            faulty_expression, var_name
        );
        UndefVarError { message }
    }
}

impl fmt::Display for UndefVarError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.message)
    }
}

impl fmt::Debug for UndefVarError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.message)
    }
}

impl Error for UndefVarError {
    fn description(&self) -> &str {
        &self.message
    }
}
