use super::error::{RuntimeError, RuntimeResult};
use super::value::*;
use heapless::consts::U256;
use heapless::Vec as HVec;
use std::slice::IterMut;

pub struct Stack {
    inner: std::cell::UnsafeCell<HVec<Val, U256>>,
}

impl Stack {
    pub fn new() -> Stack {
        Stack {
            inner: std::cell::UnsafeCell::new(HVec::default()),
        }
    }

    pub fn push(&self, frame: Val) -> RuntimeResult<()> {
        unsafe {
            (&mut *self.inner.get())
                .push(frame)
                .map_err(|_| RuntimeError::StackOverflow)
        }
    }

    pub fn pop(&self) -> Option<Val> {
        unsafe { (&mut *self.inner.get()).pop() }
    }

    pub fn last(&self) -> Option<&Val> {
        unsafe { (&mut *self.inner.get()).last() }
    }

    pub fn peek(&self, distance: i32) -> Option<&Val> {
        let i = -1 - distance as i32;
        let idx = (self.len() as i32) + i;
        unsafe { (&*self.inner.get()).get(idx as usize) }
    }

    pub fn len(&self) -> usize {
        unsafe { (&*self.inner.get()).len() }
    }

    pub fn is_empty(&self) -> bool {
        unsafe { (&*self.inner.get()).is_empty() }
    }

    pub fn truncate(&self, idx: usize) {
        unsafe { (&mut *self.inner.get()).truncate(idx) };
    }

    pub fn set(&self, idx: usize, value: Val) {
        unsafe {
            (&mut *self.inner.get())[idx] = value;
        }
    }

    pub fn get(&self, idx: usize) -> Option<&Val> {
        unsafe { (&*self.inner.get()).get(idx) }
    }

    pub fn get_mut(&self, idx: usize) -> Option<&mut Val> {
        unsafe { (&mut *self.inner.get()).get_mut(idx) }
    }

    pub fn iter_mut<'a>(&'a self) -> IterMut<'a, Val> {
        unsafe { (&mut *self.inner.get()).iter_mut() }
    }
}

impl AsRef<[Val]> for Stack {
    fn as_ref(&self) -> &[Val] {
        unsafe { (&*self.inner.get()).as_ref() }
    }
}
