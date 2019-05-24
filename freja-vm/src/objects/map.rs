use super::super::context::Context;
use super::super::error::RuntimeResult;
use super::super::stack::SubStack;
use super::super::value::{Val, Value};
use super::types::Instance;
use freja_parser::ast::Number;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

// pub enum MapKey {
//     String(String),

// }

#[derive(PartialEq, Debug, Clone, Default)]
pub struct Map {
    inner: RefCell<HashMap<String, Val>>,
}

impl Map {
    pub fn new(inner: HashMap<String, Val>) -> Map {
        Map {
            inner: RefCell::new(inner),
        }
    }
    pub fn is_empty(&self) -> bool {
        self.inner.borrow().is_empty()
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = self
            .inner
            .borrow()
            .iter()
            .map(|m| format!("{}: {}", m.0, m.1))
            .collect::<Vec<_>>()
            .join(", ");
        write!(f, "[{}]", s)
    }
}

impl Instance for Map {
    fn set_field(&self, _name: &str, _value: Val) -> RuntimeResult<()> {
        Ok(())
    }

    fn get_field(&self, name: &str) -> Option<&Val> {
        println!("name {}", name);
        None
    }

    fn find_method(&self, _name: &str) -> Option<&Value> {
        None
    }

    fn call_method(&self, name: &str, _ctx: &Context<SubStack>) -> Option<RuntimeResult<Value>> {
        match name {
            "len" => Some(Ok(Value::Number(Number::Integer(self.inner.borrow().len() as i64)))),
            // "get" => {
            //     let idx = if values.is_empty() {
            //         0
            //     } else {
            //         match values.get(0).unwrap().as_ref() {
            //             Value::Number(Number::Double(d)) => *d as usize,
            //             Value::Number(Number::Integer(i)) => *i as usize,
            //             _ => return Some(Err("invalid index".into())),
            //         }
            //     };

            //     Some(Ok(self
            //         .inner
            //         .borrow()
            //         .get(idx)
            //         .map(|m| m.as_ref().clone())
            //         .or_else(|| Some(Value::Null))
            //         .unwrap()))
            // }
            _ => None,
        }
    }
}
