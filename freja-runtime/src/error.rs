use std::error::Error;
use std::fmt;
use std::result::Result;

pub type RuntimeResult<T> = Result<T, RuntimeError>;

#[derive(Debug, PartialEq)]
pub enum RuntimeError {
    Error,
    Unknown(String),
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RuntimeError")
    }
}

impl Error for RuntimeError {}

impl From<&str> for RuntimeError {
    fn from(msg: &str) -> RuntimeError {
        RuntimeError::Unknown(msg.to_owned())
    }
}
