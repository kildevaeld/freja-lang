use super::super::error::RuntimeResult;
use super::super::value::{Val, Value};
use std::fmt;
use std::rc::Rc;

pub trait Instance: fmt::Debug {
    fn set_field(&self, name: &str, value: Val) -> RuntimeResult<()>;
    fn get_field(&self, name: &str) -> Option<&Val>;
    fn find_method(&self, name: &str) -> Option<Rc<Value>>;
    fn call_method(&self, _name: &str, _values: &[Val]) -> Option<RuntimeResult<Value>> {
        None
    }
}

pub trait Callable: fmt::Debug {
    fn arity(&self) -> i32;
    fn call(&self, args: &[&Val]) -> RuntimeResult<Val>;
}

pub trait IntoHeap {
    fn make_heap(&mut self);
}
