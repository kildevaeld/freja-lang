use super::error::RuntimeResult;
use super::value::Value;
use super::vm::VM;
use std::fmt;

pub trait FrejaCallable: fmt::Debug {
    fn call(&self, vm: &mut VM, args: Vec<&Value>) -> RuntimeResult<Value>;
    fn arity(&self) -> u8;
}
