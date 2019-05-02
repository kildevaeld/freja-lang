use super::error::RuntimeResult;
use super::scope::EnvPtr;
use super::value::{Value, ValuePtr};
use super::vm::VM;
use std::fmt;
use std::rc::Rc;

pub trait Instance: fmt::Debug {
    fn get(&self, name: &str) -> RuntimeResult<ValuePtr>;
}

pub trait FrejaCallable: fmt::Debug {
    fn call(&self, vm: &mut VM, args: Vec<ValuePtr>) -> RuntimeResult<ValuePtr>;
    fn arity(&self) -> u8;
    fn bind(&self, instance: ValuePtr) -> Box<dyn FrejaCallable>;
}
