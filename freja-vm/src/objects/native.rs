use super::super::context::Context;
use super::super::error::RuntimeResult;
use super::super::stack::SubStack;
use super::super::value::Value;
use std::fmt;
use std::rc::Rc;

pub struct Native {
    pub(crate) function: Box<Fn(&Context<SubStack>) -> RuntimeResult<Value>>,
    pub(crate) arity: u32,
}

impl Native {
    pub fn new<F: 'static + Fn(&Context<SubStack>) -> RuntimeResult<Value>>(inner: F, arity: u32) -> Native {
        Native {
            function: Box::new(inner),
            arity,
        }
    }

    pub fn value<F: 'static + Fn(&Context<SubStack>) -> RuntimeResult<Value>>(inner: F, arity: u32) -> Value {
        Value::Native(Rc::new(Native::new(inner, arity)))
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
