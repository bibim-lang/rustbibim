use std::{error, fmt};

#[derive(Debug, Clone)]
pub struct ParseError;
impl error::Error for ParseError {}
impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "code parsing is failed")
    }
}

#[derive(Debug, Clone)]
pub struct ZeroDenominatorError;
impl error::Error for ZeroDenominatorError {}
impl fmt::Display for ZeroDenominatorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "denominator cannot be zero")
    }
}
