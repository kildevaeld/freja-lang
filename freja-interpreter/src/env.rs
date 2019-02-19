use super::error::{Error, InterpretResult};
use std::collections::BTreeMap;

#[derive(Debug)]
pub enum Builtin {
    String(String),
}

#[derive(Debug)]
pub enum Instance {
    Builtin(Builtin),
}

#[derive(Debug)]
pub struct Env {
    inner: BTreeMap<String, Instance>,
}

impl Env {
    pub fn new() -> Env {
        Env {
            inner: BTreeMap::new(),
        }
    }

    pub fn define<S: AsRef<str>>(&mut self, name: S, value: Instance) -> InterpretResult<()> {
        if self.inner.contains_key(name.as_ref()) {
            return Err("define: already exists".into());
        }
        self.inner.insert(name.as_ref().to_owned(), value);
        Ok(())
    }

    pub fn get<S: AsRef<str>>(&self, name: S) -> Option<&Instance> {
        self.inner.get(name.as_ref())
    }
}
