use super::super::chunk::Chunk;
use super::super::error::RuntimeResult;
use super::super::value::{Val, Value};
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
#[derive(PartialEq)]
pub struct Function {
    #[cfg_attr(feature = "serde_support", serde(rename = "c"))]
    pub(crate) chunk: Chunk,
    #[cfg_attr(feature = "serde_support", serde(rename = "u"))]
    pub(crate) up_value_count: i32,
    #[cfg_attr(feature = "serde_support", serde(rename = "n"))]
    pub(crate) name: Option<String>,
    #[cfg_attr(feature = "serde_support", serde(rename = "a"))]
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

#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
#[derive(PartialEq, Debug)]
pub struct Closure {
    pub(crate) function: Rc<Function>,
}

impl Closure {
    pub fn new(function: Rc<Function>) -> Closure {
        Closure { function }
    }

    pub fn chunk(&self) -> &Chunk {
        &self.function.chunk
    }

    pub fn name(&self) -> Option<&str> {
        self.function.name.as_ref().map(|s| s.as_str())
    }
}

pub enum CloseurePtr {
    Stack(Rc<Closure>),
    Ref(*const Closure),
}

impl AsRef<Closure> for CloseurePtr {
    fn as_ref(&self) -> &Closure {
        match self {
            CloseurePtr::Ref(r) => unsafe { &**r },
            CloseurePtr::Stack(r) => r.as_ref(),
        }
    }
}

impl fmt::Debug for CloseurePtr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Closure as fmt::Debug>::fmt(self.as_ref(), f)
    }
}

pub struct Native {
    pub(crate) function: Box<Fn(&[Val]) -> RuntimeResult<Value>>,
}

impl Native {
    pub fn new<F: 'static + Fn(&[Val]) -> RuntimeResult<Value>>(inner: F) -> Native {
        Native {
            function: Box::new(inner),
        }
    }
}

impl PartialEq for Native {
    fn eq(&self, _other: &Native) -> bool {
        false
    }
}

impl fmt::Debug for Native {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Native")
    }
}