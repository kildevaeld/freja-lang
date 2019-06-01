use super::super::error::RuntimeResult;
use super::super::value::Value;
use super::types::Instance;
use std::any::Any;

impl Instance for String {
    fn set_field(&self, _name: &str, _value: Value) -> RuntimeResult<()> {
        Ok(())
    }

    fn get_field(&self, _name: &str) -> Option<&Value> {
        None
    }

    fn find_method(&self, _name: &str) -> Option<&Value> {
        None
    }

    // fn call_method(&self, name: &str, _ctx: &Context<SubStack>) -> Option<RuntimeResult<Value>> {
    //     match name {
    //         "len" => Some(Ok(Value::Integer(self.len() as i64))),
    //         _ => None,
    //     }
    // }

    fn as_any(&self) -> &Any {
        self
    }
}
