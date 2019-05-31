use freja_parser::parser::ParseError;
use std::error::Error;
use std::fmt;
use std::result::Result;

pub type CompileResult<T> = Result<T, CompileError>;

#[derive(Debug, PartialEq)]
pub enum CompileError {
    DuplicateVariable,
    ReturnTopLevel,
    InvalidReceiver,
    Parse(ParseError),
}

impl fmt::Display for CompileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CompileError")
    }
}

impl Error for CompileError {}

