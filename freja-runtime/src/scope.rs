use super::error::{RuntimeError, RuntimeResult};
use super::value::Value;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug)]
pub struct Env<V> {
    parent: Option<Rc<Env<V>>>,
    inner: HashMap<String, V>,
}

impl<V> Env<V> {
    pub fn new() -> Env<V> {
        Env {
            parent: None,
            inner: HashMap::new(),
        }
    }

    pub fn with_parent(parent: Rc<Env<V>>) -> Env<V> {
        Env {
            parent: Some(parent),
            inner: HashMap::new(),
        }
    }

    pub fn define<S: AsRef<str>>(&mut self, name: S, value: V) -> RuntimeResult<()> {
        if self.inner.contains_key(name.as_ref()) {
            return Err("define: already exists".into());
        }
        self.inner.insert(name.as_ref().to_owned(), value);
        Ok(())
    }

    pub fn get<S: AsRef<str>>(&self, name: S) -> Option<&V> {
        self.inner.get(name.as_ref())
    }
}
