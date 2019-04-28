use super::value::Value;
use std::error::Error;
use std::fmt;
use std::rc::Rc;
use std::result::Result;

pub type RuntimeResult<T> = Result<T, RuntimeError>;

#[derive(Debug)]
pub enum RuntimeError {
    Error,
    Unknown(String),
    Return(Rc<Value>),
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
