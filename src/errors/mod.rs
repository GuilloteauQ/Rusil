pub(crate) mod type_errors;
pub(crate) mod undefined_variable_errors;

use crate::errors::type_errors::TypeError;
use crate::errors::undefined_variable_errors::UndefVarError;
use crate::types::Type;
use std;
use std::error::Error;
use std::fmt;

pub(crate) enum LangError {
    TypeError(TypeError),
    UndefVarError(UndefVarError),
}

impl LangError {
    pub fn new_type_error(
        expected_type: Type,
        given_type: Type,
        faulty_expression: String,
    ) -> Self {
        LangError::TypeError(TypeError::new(expected_type, given_type, faulty_expression))
    }
    pub fn new_undefined_variable_error(var_name: String, faulty_expression: String) -> Self {
        LangError::UndefVarError(UndefVarError::new(var_name, faulty_expression))
    }
}

impl fmt::Display for LangError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LangError::TypeError(e) => e.fmt(f),
            LangError::UndefVarError(e) => e.fmt(f),
            _ => unimplemented!(),
        }
    }
}

impl fmt::Debug for LangError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LangError::TypeError(e) => e.fmt(f),
            LangError::UndefVarError(e) => e.fmt(f),
            _ => unimplemented!(),
        }
    }
}

impl Error for LangError {
    fn description(&self) -> &str {
        match self {
            LangError::TypeError(e) => e.description(),
            LangError::UndefVarError(e) => e.description(),
            _ => unimplemented!(),
        }
    }
}
