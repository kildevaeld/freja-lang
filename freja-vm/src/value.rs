use super::chunk::Chunk;
use std::fmt;
use std::rc::Rc;

pub type ValuePtr = Rc<Value>;

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

impl Function {
    pub fn new() -> Function {
        Function { chunk: Chunk::new(), up_value_count: 0, name: None, arity: 0 }
    }
}

#[derive(PartialEq)]
pub struct Closure {
    pub(crate) function: Rc<Function>,
}

impl Closure {
    pub fn new(function: Rc<Function>) -> Closure {
        Closure { function }
    }
}

pub struct Native {
    pub(crate) function: Box<Fn(&[ValuePtr])>,
}

impl PartialEq for Native {
    fn eq(&self, other: &Native) -> bool {
        false
    }
}

#[derive(PartialEq)]
pub enum Value {
    Double(f64),
    Integer(i64),
    String(String),
    Boolean(bool),
    Function(Rc<Function>),
    Closure(Rc<Closure>),
    Native(Rc<Native>),
    Null,
}

impl Value {
    pub fn as_function(&self) -> Option<&Rc<Function>> {
        match self {
            Value::Function(f) => Some(f),
            _ => None,
        }
    }

    pub fn as_string(&self) -> Option<&String> {
        match self {
            Value::String(f) => Some(f),
            _ => None,
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Double(d) => write!(f, "{}", d),
            Value::Integer(i) => write!(f, "{}", i),
            Value::String(s) => write!(f, "{}", s),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Function(fu) => write!(f, "<fn {}>", fu.name.as_ref().map(|a| a.as_str()).unwrap_or("no-name")),
            Value::Null => write!(f, "nil"),
            _ => write!(f, "Unknown"),
        }
    }
}
