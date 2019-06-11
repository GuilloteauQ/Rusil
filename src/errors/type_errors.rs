use crate::tokens::Expr;
use crate::types::Type;
use std;
use std::error::Error;
use std::fmt;

pub(crate) struct TypeError {
    message: String,
}

impl TypeError {
    pub fn new(expected_type: Type, given_type: Type, faulty_expression: String) -> Self {
        let message = format!(
            "\n>>> TypeError:\n\n\t\x1B[31m{}\x1B[39m\n\n\tExpected: {:?}\n\tFound: {:?}\n",
            faulty_expression, expected_type, given_type
        );
        TypeError { message }
    }
}

impl fmt::Display for TypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.message)
    }
}

impl fmt::Debug for TypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.message)
    }
}

impl Error for TypeError {
    fn description(&self) -> &str {
        &self.message
    }
}
