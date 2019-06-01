use super::super::context::Context;
use super::super::error::RuntimeResult;
use super::super::stack::SubStack;
use super::super::value::Value;
use std::fmt;
use std::rc::Rc;

pub trait Native {
    fn call(&self, ctx: &Context<SubStack>) -> RuntimeResult<Value>;
    fn arity(&self) -> i32;
}

impl<'a> PartialEq for Native + 'a {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}

impl<'a> fmt::Debug for Native + 'a {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "native")
    }
}

#[derive(PartialEq)]
pub struct NativeFn<F>
where
    F: Fn(&Context<SubStack>) -> RuntimeResult<Value>,
{
    inner: F,
    arity: i32,
}

impl<F> NativeFn<F>
where
    F: 'static + Fn(&Context<SubStack>) -> RuntimeResult<Value>,
{
    pub fn new(func: F, arity: i32) -> NativeFn<F> {
        NativeFn { inner: func, arity }
    }

    pub fn value(inner: F, arity: i32) -> Value {
        Value::Native(Rc::new(Box::new(NativeFn::new(inner, arity))))
    }
}

impl<F> Native for NativeFn<F>
where
    F: Fn(&Context<SubStack>) -> RuntimeResult<Value>,
{
    fn call(&self, ctx: &Context<SubStack>) -> RuntimeResult<Value> {
        (self.inner)(ctx)
    }
    fn arity(&self) -> i32 {
        self.arity
    }
}

impl<F> fmt::Debug for NativeFn<F>
where
    F: Fn(&Context<SubStack>) -> RuntimeResult<Value>,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Native")
    }
}
