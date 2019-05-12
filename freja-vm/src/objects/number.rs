use super::super::error::RuntimeResult;
use super::super::value::{Val, Value};
use super::objects::Closure;
use super::types::Instance;
use freja_parser::ast::Number;
use std::rc::Rc;

impl Instance for Number {
    fn set_field(&self, name: &str, value: Val) -> RuntimeResult<()> {
        Ok(())
    }

    fn get_field(&self, name: &str) -> Option<&Val> {
        None
    }

    fn find_method(&self, name: &str) -> Option<Rc<Value>> {
        None
    }
}
