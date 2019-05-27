use super::super::error::RuntimeResult;
use super::super::value::{Val, Value};
use super::types::Instance;
use freja_parser::ast::Number;
use std::any::Any;

impl Instance for Number {
    fn set_field(&self, _name: &str, _value: Value) -> RuntimeResult<()> {
        Ok(())
    }

    fn get_field(&self, _name: &str) -> Option<&Value> {
        None
    }

    fn find_method(&self, _name: &str) -> Option<&Value> {
        None
    }

    fn as_any(&self) -> &Any {
        self
    }
}
