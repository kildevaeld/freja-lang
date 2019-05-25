use std::collections::HashMap;
use std::cell::UnsafeCell;
use super::value::Value;

pub struct Global {
    //builtins: HashMap<&'static str, 
    inner:  HashMap<String, Value>
}

impl Global {
    pub fn new() -> Global {
        Global{
            inner: HashMap::new()
        }
    }


}