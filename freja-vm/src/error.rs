use freja_parser::parser::ParseError;
use std::error::Error;
use std::fmt;
use std::result::Result;
use freja_compiler::CompileError;

pub type RuntimeResult<T> = Result<T, RuntimeError>;

#[derive(Debug)]
pub enum RuntimeError {
    Syntax(CompileError),
    Error(String),
    StackOverflow,
    InvalidIndex,
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RuntimeError::Error(s) => write!(f, "{}", s),
            RuntimeError::StackOverflow => write!(f, "StackOverflow"),
            RuntimeError::Syntax(e) => <CompileError as fmt::Display>::fmt(e, f),
            RuntimeError::InvalidIndex => write!(f, "InvalidIndex"),
        }
    }
}

impl Error for RuntimeError {}

impl From<&str> for RuntimeError {
    fn from(msg: &str) -> RuntimeError {
        RuntimeError::Error(msg.to_owned())
    }
}

impl From<String> for RuntimeError {
    fn from(msg: String) -> RuntimeError {
        RuntimeError::Error(msg.clone())
    }
}

impl From<CompileError> for RuntimeError {
    fn from(error: CompileError) -> RuntimeError {
        RuntimeError::Syntax(error)
    }
}

impl From<ParseError> for RuntimeError {
    fn from(error: ParseError) -> RuntimeError {
        RuntimeError::Syntax(CompileError::Parse(error))
    }
}

// impl From<std::option::NoneError> for RuntimeError {
//     fn from(error: std::option::NoneError) -> RuntimeError {
//         RuntimeError::InvalidIndex
//     }
// }
