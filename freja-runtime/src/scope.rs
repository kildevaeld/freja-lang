use super::error::{RuntimeError, RuntimeResult};
use super::value::Value;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub type EnvPtr<V> = Rc<RefCell<Env<V>>>;

#[derive(Debug)]
pub struct Env<V> {
    parent: Option<Rc<RefCell<Env<V>>>>,
    inner: HashMap<String, V>,
    globals: HashMap<String, V>,
}

impl<V: Clone> Env<V> {
    pub fn new() -> Env<V> {
        Env {
            parent: None,
            inner: HashMap::new(),
            globals: HashMap::new(),
        }
    }

    pub fn with_parent(parent: Rc<RefCell<Env<V>>>) -> Env<V> {
        Env {
            parent: Some(parent),
            inner: HashMap::new(),
            globals: HashMap::new(),
        }
    }

    pub fn define<S: AsRef<str>>(&mut self, name: S, value: V) -> RuntimeResult<()> {
        if self.inner.contains_key(name.as_ref()) {
            return Err("define: already exists".into());
        }
        self.inner.insert(name.as_ref().to_owned(), value);
        Ok(())
    }

    pub fn get<S: AsRef<str>>(&mut self, name: S) -> Option<&V> {
        match self.inner.get(name.as_ref()) {
            Some(s) => Some(s),
            None => match &self.parent {
                Some(inner) => match inner.borrow_mut().get(name.as_ref()) {
                    Some(v) => {
                        self.globals.insert(name.as_ref().to_string(), v.clone());
                        self.globals.get(name.as_ref())
                    }
                    None => None,
                },
                None => None,
            },
        }
    }
}
