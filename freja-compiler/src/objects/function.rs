use super::super::chunk::Chunk;
use std::fmt;

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

    pub fn arity(&self) -> i32 {
        self.arity
    }

    pub fn up_value_count(&self) -> i32 {
        self.up_value_count
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_ref().map(|f| f.as_str())
    }
}
