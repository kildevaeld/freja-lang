use super::super::context::Context;
use super::super::error::RuntimeResult;
use super::super::stack::SubStack;
use super::super::value::{Val, Value};
use super::types::Instance;
use freja_parser::ast::Number;
use std::rc::Rc;

impl Instance for String {
    fn set_field(&self, _name: &str, _value: Val) -> RuntimeResult<()> {
        Ok(())
    }

    fn get_field(&self, _name: &str) -> Option<&Val> {
        None
    }

    fn find_method(&self, _name: &str) -> Option<&Value> {
        None
    }

    fn call_method(&self, name: &str, _ctx: &Context<SubStack>) -> Option<RuntimeResult<Value>> {
        match name {
            "len" => Some(Ok(Value::Integer(self.len() as i64))),
            _ => None,
        }
    }
}
