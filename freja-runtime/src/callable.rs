use super::error::RuntimeResult;
use super::value::Value;
use super::vm::VM;
use std::fmt;
use std::rc::Rc;

pub trait FrejaCallable: fmt::Debug {
    fn call(&self, vm: &mut VM, args: Vec<Rc<Value>>) -> RuntimeResult<Rc<Value>>;
    fn arity(&self) -> u8;
}
