pub(crate) mod type_errors;

use crate::errors::type_errors::TypeError;
use crate::types::Type;
use std;
use std::error::Error;
use std::fmt;

pub(crate) enum LangError {
    TypeError(TypeError),
}

impl LangError {
    pub fn new_type_error(
        expected_type: Type,
        given_type: Type,
        faulty_expression: String,
    ) -> Self {
        LangError::TypeError(TypeError::new(expected_type, given_type, faulty_expression))
    }
}

impl fmt::Display for LangError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LangError::TypeError(e) => e.fmt(f),
            _ => unimplemented!(),
        }
    }
}

impl fmt::Debug for LangError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LangError::TypeError(e) => e.fmt(f),
            _ => unimplemented!(),
        }
    }
}

impl Error for LangError {
    fn description(&self) -> &str {
        match self {
            LangError::TypeError(e) => e.description(),
            _ => unimplemented!(),
        }
    }
}
