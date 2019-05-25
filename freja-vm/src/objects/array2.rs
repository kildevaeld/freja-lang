use super::super::utils::Pointer;
use super::super::value::{Val, Value};
use std::cell::UnsafeCell;
use std::collections::HashMap;

pub struct Array2 {
    methods: HashMap<&'static str, Value>,
}

pub struct ArrayInstance {
    inner: UnsafeCell<Vec<Val>>,
    class: Pointer<Array2>,
}
