use std::{error, fmt};

#[derive(Debug, Clone)]
pub enum VectorErrorType {
    InvalidInitSizeError,
    InvalidCalcError
}

#[derive(Debug, Clone)]
pub struct VectorError {
    pub err_msg: String,
    pub err_type: VectorErrorType
}

impl fmt::Display for VectorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.err_msg)
    }
}

impl error::Error for VectorError {}