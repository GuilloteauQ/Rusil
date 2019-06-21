use crate::tokens::Expr;
use crate::types::Type;
use std;
use std::error::Error;
use std::fmt;

pub(crate) struct OOBError {
    message: String,
}

impl OOBError {
    pub fn new(vec_len: usize, attempted_index: usize, faulty_expression: String) -> Self {
        let message = format!(
            "\n>>> OutOfBoundsError:\n\n\t\x1B[31m{}\x1B[39m\n\n\tThe vector is of size: {:?}\n\tbut index is: {:?}\n",
            faulty_expression, vec_len, attempted_index
        );
        OOBError { message }
    }
}

impl fmt::Display for OOBError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.message)
    }
}

impl fmt::Debug for OOBError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.message)
    }
}

impl Error for OOBError {
    fn description(&self) -> &str {
        &self.message
    }
}
