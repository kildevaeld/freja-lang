use super::super::error::RuntimeResult;
use super::super::value::Val;
use super::objects::Closure;
use std::fmt;
use std::rc::Rc;

pub trait Instance: fmt::Debug {
    fn set_field(&self, name: &str, value: Val) -> RuntimeResult<()>;
    fn get_field(&self, name: &str) -> Option<&Val>;
    fn find_method(&self, name: &str) -> Option<Rc<Closure>>;
}

pub trait IntoHeap {
    fn make_heap(&mut self);
}
