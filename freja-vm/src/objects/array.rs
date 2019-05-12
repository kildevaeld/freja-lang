use super::super::error::RuntimeResult;
use super::super::value::{Val, Value};
use super::objects::Closure;
use super::objects::Native;
use super::types::Instance;
use freja_parser::ast::Number;
use std::fmt;
use std::rc::Rc;

use std::cell::RefCell;

#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
#[derive(PartialEq, Debug, Clone, Default)]
pub struct Array {
    inner: RefCell<Vec<Val>>,
}

impl Array {
    pub fn new(inner: Vec<Val>) -> Array {
        Array {
            inner: RefCell::new(inner),
        }
    }
    pub fn is_empty(&self) -> bool {
        self.inner.borrow_mut().is_empty()
    }
}

impl fmt::Display for Array {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = self
            .inner
            .borrow()
            .iter()
            .map(|m| m.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        write!(f, "[{}]", s)
    }
}

impl Instance for Array {
    fn set_field(&self, name: &str, value: Val) -> RuntimeResult<()> {
        Ok(())
    }

    fn get_field(&self, name: &str) -> Option<&Val> {
        println!("name {}", name);
        None
    }

    fn find_method(&self, name: &str) -> Option<Rc<Value>> {
        None
    }

    fn call_method(&self, name: &str, values: &[Val]) -> Option<RuntimeResult<Value>> {
        match name {
            "len" => Some(Ok(Value::Number(Number::Integer(self.inner.borrow().len() as i64)))),
            "push" => {
                values.iter().for_each(|m| self.inner.borrow_mut().push(m.clone()));
                Some(Ok(Value::Null))
            }
            "get" => {
                let idx = if values.is_empty() {
                    0
                } else {
                    match values.get(0).unwrap().as_ref() {
                        Value::Number(Number::Double(d)) => *d as usize,
                        Value::Number(Number::Integer(i)) => *i as usize,
                        _ => return Some(Err("invalid index".into())),
                    }
                };

                Some(Ok(self
                    .inner
                    .borrow()
                    .get(idx)
                    .map(|m| m.as_ref().clone())
                    .or_else(|| Some(Value::Null))
                    .unwrap()))
            }
            _ => None,
        }
    }
}
