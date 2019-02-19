use std::error::Error as StdError;
use std::fmt;
use std::result::Result;
pub type InterpretResult<T> = Result<T, Error>;

#[derive(Debug)]
pub enum ErrorKind {
    Parse,
    Unknown(String),
}

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
}

impl Error {
    pub fn new(kind: ErrorKind) -> Error {
        Error { kind }
    }
}

impl From<ErrorKind> for Error {
    fn from(error: ErrorKind) -> Error {
        Error::new(error)
    }
}

impl From<&str> for Error {
    fn from(msg: &str) -> Error {
        Error::new(ErrorKind::Unknown(msg.to_owned()))
    }
}
