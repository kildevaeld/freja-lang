use super::super::error::RuntimeResult;
use super::super::value::{Val, Value};
use super::types::Instance;
use freja_parser::ast::Number;

impl Instance for Number {
    fn set_field(&self, _name: &str, _value: Val) -> RuntimeResult<()> {
        Ok(())
    }

    fn get_field(&self, _name: &str) -> Option<&Val> {
        None
    }

    fn find_method(&self, _name: &str) -> Option<&Value> {
        None
    }
}
