use super::super::context::Context;
use super::super::error::RuntimeResult;
use super::super::stack::{Stack, SubStack};
use super::super::value::{Val, Value};
use super::native::Native;
use super::types::Instance;
use std::cell::UnsafeCell;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

macro_rules! map(
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = ::std::collections::HashMap::new();
            $(
                m.insert($key, $value);
            )+
            m
        }
     };
);

pub struct Array {
    inner: UnsafeCell<Vec<Val>>,
    methods: HashMap<&'static str, Value>,
}

impl Array {
    pub fn new(inner: Vec<Val>) -> Array {
        let methods = map! {
            "len" => Native::value(
                |ctx| {
                    let array = ctx.get(0).unwrap().as_array().unwrap();
                    Ok(Value::Integer(array.len() as i64))
                },
                0,
            ),
            "get" => Native::value(
                |ctx| {
                    let array = ctx.get(0).unwrap().as_array().unwrap();
                    let idx = match ctx.get(1).unwrap().as_ref() {
                        Value::Integer(i) => *i,
                        Value::Double(d) => *d as i64,
                        _ => unimplemented!("get"),
                    };
                    match array.get(idx as usize) {
                        None => Ok(Value::Null),
                        Some(s) => Ok(s.as_ref().clone()),
                    }
                },
                1,
            ),
            "push" => Native::value(|ctx| {
                let array = ctx.get_mut(0).unwrap().as_array().unwrap();
                let val = ctx.get(1).unwrap();
                array.push(val.clone());
                Ok(ctx.get(0).unwrap().as_ref().clone())
            }, 1),
            "each" => Native::value(|ctx| {
                let array = ctx.get_mut(0).unwrap().as_array().unwrap();
                let inner = unsafe { (&mut *array.inner.get()) };
                for (i, v) in inner.iter_mut().enumerate() {
                     ctx.dup(1)?;
                     ctx.stack.push(v.as_ptr())?;
                     ctx.push(Value::Integer(i as i64))?;
                     ctx.call(2)?;
                     ctx.pop();
                }
                Ok(ctx.get(0).unwrap().as_ref().clone())
            }, 1),
            "map" => Native::value(|ctx| {
                let array = ctx.get_mut(0).unwrap().as_array().unwrap();
                let inner = unsafe { (&mut *array.inner.get()) };
                let mut out = Vec::new();
                for (i, v) in inner.iter_mut().enumerate() {
                    ctx.dup(1)?;
                    ctx.stack.push(v.as_ptr())?;
                    ctx.push(Value::Integer(i as i64))?;
                    ctx.call(2)?;
                    let item = ctx.pop().unwrap();
                    out.push(item);
                }
                Ok(Value::Array(Rc::new(Array::new(out))))
            }, 1)
        };

        Array {
            inner: UnsafeCell::new(inner),
            methods,
        }
    }

    pub fn len(&self) -> usize {
        unsafe { (&*self.inner.get()).len() }
    }

    pub fn get(&self, idx: usize) -> Option<&Val> {
        unsafe { (&*self.inner.get()).get(idx) }
    }

    pub fn push(&self, val: Val) {
        unsafe { (&mut *self.inner.get()).push(val) }
    }

    pub fn is_empty(&self) -> bool {
        unsafe { (&*self.inner.get()).is_empty() }
    }
}

impl fmt::Display for Array {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = unsafe {
            (&*self.inner.get())
                .iter()
                .map(|m| m.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        };
        write!(f, "[{}]", s)
    }
}

impl fmt::Debug for Array {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = unsafe {
            (&*self.inner.get())
                .iter()
                .map(|m| m.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        };
        write!(f, "[{}]", s)
    }
}

impl Default for Array {
    fn default() -> Array {
        Array::new(Vec::new())
    }
}

impl PartialEq for Array {
    fn eq(&self, other: &Array) -> bool {
        unsafe { (&*self.inner.get()) == (&*other.inner.get()) }
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

    fn find_method(&self, name: &str) -> Option<&Value> {
        self.methods.get(name)
    }
}
