use super::super::context::Context;
use super::super::error::RuntimeResult;
use super::super::stack::SubStack;
use super::super::value::{Val, Value};
use std::any::Any;
use std::fmt;

pub trait Instance: fmt::Debug {
    fn set_field(&self, name: &str, value: Value) -> RuntimeResult<()>;
    fn get_field(&self, name: &str) -> Option<&Value>;
    fn find_method(&self, name: &str) -> Option<&Value>;
    fn call_method(&self, _name: &str, _ctx: &Context<SubStack>) -> Option<RuntimeResult<Value>> {
        None
    }
    fn as_any(&self) -> &Any;
}

pub trait Callable: fmt::Debug {
    fn arity(&self) -> i32;
    fn call(&self, args: &[&Val]) -> RuntimeResult<Val>;
}

pub trait IntoHeap {
    fn make_heap(&mut self);
}
