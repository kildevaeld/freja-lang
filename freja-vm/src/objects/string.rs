use super::super::error::RuntimeResult;
use super::super::value::{Val, Value};
use super::objects::Closure;
use super::objects::Native;
use super::types::Instance;
use freja_parser::ast::Number;
use std::rc::Rc;

impl Instance for String {
    fn set_field(&self, name: &str, value: Val) -> RuntimeResult<()> {
        Ok(())
    }

    fn get_field(&self, name: &str) -> Option<&Val> {
        None
    }

    fn find_method(&self, name: &str) -> Option<Rc<Value>> {
        None
    }

    fn call_method(&self, name: &str, values: &[Val]) -> Option<RuntimeResult<Value>> {
        match name {
            "len" => Some(Ok(Value::Number(Number::Integer(self.len() as i64)))),
            _ => None,
        }
    }
}
