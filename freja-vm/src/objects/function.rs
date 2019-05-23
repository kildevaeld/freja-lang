use super::super::chunk::Chunk;
use super::super::context::Context;
use super::super::error::RuntimeResult;
use super::super::stack::SubStack;
use super::super::value::{Val, Value};
use std::fmt;
use std::rc::Rc;

#[derive(PartialEq)]
pub struct Function {
    pub(crate) chunk: Chunk,
    pub(crate) up_value_count: i32,
    pub(crate) name: Option<String>,
    pub(crate) arity: i32,
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.chunk)
    }
}

impl fmt::Debug for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Function")
    }
}

impl Function {
    pub fn new() -> Function {
        Function {
            chunk: Chunk::new(),
            up_value_count: 0,
            name: None,
            arity: 0,
        }
    }

    pub fn chunk(&self) -> &Chunk {
        &self.chunk
    }
}

pub enum FunctionPtr {
    Stack(Function),
    Ref(*const Function),
}

impl AsRef<Function> for FunctionPtr {
    fn as_ref(&self) -> &Function {
        match self {
            FunctionPtr::Ref(r) => unsafe { &**r },
            FunctionPtr::Stack(r) => r,
        }
    }
}

impl fmt::Debug for FunctionPtr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Function as fmt::Debug>::fmt(self.as_ref(), f)
    }
}
