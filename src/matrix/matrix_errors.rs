use std::{error, fmt};

#[derive(Debug, Clone)]
pub enum MatrixErrorType {
    InvalidInitSizeError,
    InvalidCalcError
}

#[derive(Debug, Clone)]
pub struct MatrixError {
    pub err_msg: String,
    pub err_type: MatrixErrorType
}

impl fmt::Display for MatrixError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.err_msg)
    }
}

impl error::Error for MatrixError {}