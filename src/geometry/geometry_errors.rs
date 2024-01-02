use std::{fmt};

#[derive(Debug)]
pub struct TriangleError {
    pub err_code: usize,
    pub message: String,
}

impl fmt::Display for TriangleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let err_msg = match self.err_code {
            1002 => "The length of the triangle parameter vector can only be 2",
            1003 => "The length of the triangle parameter vector can only be 3",
            1004 => "The length of the triangle parameter vector can only be 4",
            2001 => "The range of RGB values for color is [0, 255]",
            3001 => "Index out of range",
            _ => "Sorry, something is wrong! Please Try Again!",
        };

        write!(f, "{}", err_msg)
    }
}