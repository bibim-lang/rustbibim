use std::{error, fmt};


#[derive(Debug, Clone)]
pub struct RootMemBowlError;
impl error::Error for RootMemBowlError {}
impl fmt::Display for RootMemBowlError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "root bowl should not be Mem(@) bowl")
    }
}
