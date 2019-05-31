use std::collections::HashMap;
use std::cell::UnsafeCell;
use super::value::Value;

#[derive(Default, Debug)]
pub struct Global {
    inner: UnsafeCell<HashMap<String, Value>>
}

impl Global {
    pub fn new() -> Global {
        Global{
            inner: UnsafeCell::new(HashMap::new())
        }
    }

    pub fn insert(&self, name: String, value: Value) {
        unsafe {
            (&mut *self.inner.get()).insert(name, value)
        };
    } 

    pub fn get(&self, name: &str) -> Option<&Value> {
        unsafe {
            (&*self.inner.get()).get(name)
        }
    }

    pub fn contains_key(&self, name: &str) -> bool {
        unsafe {
            (&*self.inner.get()).contains_key(name)
        }
    }

}