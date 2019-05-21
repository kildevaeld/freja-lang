use super::super::context::Context;
use super::super::error::RuntimeResult;
use super::super::stack::SubStack;
use super::super::value::{Val, Value};
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
    fn set_field(&self, _name: &str, _value: Val) -> RuntimeResult<()> {
        Ok(())
    }

    fn get_field(&self, name: &str) -> Option<&Val> {
        println!("name {}", name);
        None
    }

    fn find_method(&self, _name: &str) -> Option<Rc<Value>> {
        None
    }

    fn call_method(&self, name: &str, ctx: &Context<SubStack>) -> Option<RuntimeResult<Value>> {
        match name {
            "len" => Some(Ok(Value::Number(Number::Integer(self.inner.borrow().len() as i64)))),
            "each" => {
                for (i, v) in self.inner.borrow_mut().iter().enumerate() {
                    ctx.dup(0);
                    ctx.push(v.as_value().clone());
                    ctx.push(Value::Number(Number::Integer(i as i64)));
                    ctx.call(2);
                    ctx.pop();

                }
                Some(Ok(Value::Null))
            }
            "map" => {
                let mut out = Vec::new();
                for (i, v) in self.inner.borrow_mut().iter().enumerate() {
                    ctx.dup(0);
                    ctx.push(v.as_value().clone());
                    ctx.push(Value::Number(Number::Integer(i as i64)));
                    ctx.call(2);
                    let  item = ctx.pop().unwrap();
                    out.push(item);

                }
                Some(Ok(Value::Array(Array::new(out))))
            }

            "get" => {
                let idx = if ctx.top() == 0 {
                    0
                } else {
                    match ctx.get(0).unwrap().as_ref() {
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
