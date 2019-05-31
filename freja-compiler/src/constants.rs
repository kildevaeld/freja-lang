use super::objects::*;
use std::rc::Rc;
use std::fmt;

#[derive(PartialEq, Debug, Clone)]
pub enum Constant {
    Integer(i64),
    Double(f64),
    Boolean(bool),
    String(String),
    Function(Rc<Function>),
    Nil
}

impl Constant {
    #[inline(always)]
    pub fn as_function(&self) -> Option<&Rc<Function>> {
        match self {
            Constant::Function(f) => Some(f),
            _ => None,
        }
    }

    #[inline(always)]
    pub fn as_string(&self) -> Option<&String> {
        match self {
            Constant::String(f) => Some(f),
            _ => None,
        }
    }
}


impl fmt::Display for Constant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Constant::Integer(i) => <i64 as fmt::Display>::fmt(i, f),
            Constant::Double(d) => <f64 as fmt::Display>::fmt(d, f),
            Constant::String(s) =>  write!(f, "{}", s),
            Constant::Boolean(b) => write!(f, "{}", b),
            Constant::Function(fu) => write!(f, "<fn {}>", fu.name.as_ref().map(|a| a.as_str()).unwrap_or("no-name")),
            Constant::Nil => write!(f, "nil"),
            
        }
    }
}