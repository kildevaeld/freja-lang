use super::super::context::Context;
use super::super::error::RuntimeResult;
use super::super::stack::SubStack;
use super::super::value::Value;
use std::any::Any;
use std::fmt;


pub trait Type: fmt::Debug {
    fn name(&self) -> &str;
    fn as_any(&self) -> &Any;
    fn equal(&self, other: &Type) -> bool;
}

pub trait Instance: fmt::Debug {
    fn set_field(&self, name: &str, value: Value) -> RuntimeResult<()>;
    fn get_field(&self, name: &str) -> Option<&Value>;
    fn find_method(&self, name: &str) -> Option<&Value>;
    fn as_any(&self) -> &Any;
}

// pub trait Callable: fmt::Debug {
//     fn arity(&self) -> i32;
//     fn call(&self, args: &[&Val]) -> RuntimeResult<Val>;
// }

// pub trait IntoHeap {
//     fn make_heap(&mut self);
// }


pub trait Class: Instance {
    fn name(&self) -> &str;
    fn construct(&self, ctx: &Context<SubStack>) -> RuntimeResult<()>;
    fn as_instance(&self) -> &Instance;
    //fn find_method(&self, name: &str) -> Option<&Value>;
}

impl<'a> PartialEq for Class + 'a {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}


pub trait ClassInstance: Instance {
    fn class(&self) -> &Class;
    fn as_instance(&self) -> &Instance;
    //fn find_method(&self, name: &str) -> Option<&Value>;
}

impl<'a> PartialEq for ClassInstance + 'a {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}