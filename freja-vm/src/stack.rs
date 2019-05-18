use super::error::{RuntimeError, RuntimeResult};
use super::value::*;
use heapless::consts::U256;
use heapless::Vec as HVec;
use std::iter::Iterator;

pub trait Stack: AsRef<[Val]> + std::fmt::Debug {
    fn push(&self, frame: Val) -> RuntimeResult<()>;
    fn pop(&self) -> Option<Val>;
    fn last(&self) -> Option<&Val>;
    fn peek(&self, distance: i32) -> Option<&Val>;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn truncate(&self, idx: usize);
    fn set(&self, idx: usize, value: Val);
    fn get(&self, idx: usize) -> Option<&Val>;
    fn get_mut(&self, idx: usize) -> Option<&mut Val>;
    fn substack<'a>(&'a self, from: usize) -> SubStack<'a>;
}

#[derive(Debug)]
pub struct RootStack {
    inner: std::cell::UnsafeCell<HVec<Val, U256>>,
}

impl RootStack {
    pub fn new() -> RootStack {
        RootStack {
            inner: std::cell::UnsafeCell::new(HVec::default()),
        }
    }
}

impl Stack for RootStack {
    fn push(&self, frame: Val) -> RuntimeResult<()> {
        unsafe {
            (&mut *self.inner.get())
                .push(frame)
                .map_err(|_| RuntimeError::StackOverflow)
        }
    }

    fn pop(&self) -> Option<Val> {
        unsafe { (&mut *self.inner.get()).pop() }
    }

    fn last(&self) -> Option<&Val> {
        unsafe { (&mut *self.inner.get()).last() }
    }

    fn peek(&self, distance: i32) -> Option<&Val> {
        let i = -1 - distance as i32;
        let idx = (self.len() as i32) + i;
        unsafe { (&*self.inner.get()).get(idx as usize) }
    }

    fn len(&self) -> usize {
        unsafe { (&*self.inner.get()).len() }
    }

    fn is_empty(&self) -> bool {
        unsafe { (&*self.inner.get()).is_empty() }
    }

    fn truncate(&self, idx: usize) {
        unsafe { (&mut *self.inner.get()).truncate(idx) };
    }

    fn set(&self, idx: usize, value: Val) {
        unsafe {
            (&mut *self.inner.get())[idx] = value;
        }
    }

    fn get(&self, idx: usize) -> Option<&Val> {
        unsafe { (&*self.inner.get()).get(idx) }
    }

    fn get_mut(&self, idx: usize) -> Option<&mut Val> {
        unsafe { (&mut *self.inner.get()).get_mut(idx) }
    }

    fn substack<'a>(&'a self, from: usize) -> SubStack<'a> {
        SubStack::new(self, from)
    }
}

impl AsRef<[Val]> for RootStack {
    fn as_ref(&self) -> &[Val] {
        unsafe { (&*self.inner.get()).as_ref() }
    }
}

#[derive(Debug)]
pub struct SubStack<'a> {
    inner: &'a RootStack,
    idx: usize,
    len: usize
}

impl<'a> SubStack<'a> {
    pub fn new(stack: &'a RootStack, idx: usize) -> SubStack {
        SubStack { inner: stack, idx, len: stack.len() - idx }
    }

    pub fn root(&self) -> &RootStack {
        self.inner
    }
}

impl<'a> Stack for SubStack<'a> {
    fn push(&self, frame: Val) -> RuntimeResult<()> {

        self.inner.push(frame)
    }

    fn pop(&self) -> Option<Val> {
        if self.inner.len() == self.idx {
            return None;
        }
        
        return self.inner.pop();
    }

    fn last(&self) -> Option<&Val> {
        if self.inner.len() == self.idx {
            return None;
        }
        return self.inner.last();
    }

    fn peek(&self, distance: i32) -> Option<&Val> {
        let i = -1 - distance as i32;
        let idx = (self.len() as i32) + i;
        self.inner.peek(self.idx as i32 + idx)
    }

    fn len(&self) -> usize {
        self.inner.len() - self.idx
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn truncate(&self, idx: usize) {
        self.inner.truncate(self.idx + idx);
    }

    fn set(&self, idx: usize, value: Val) {
        self.inner.set(self.idx + idx, value);
    }

    fn get(&self, idx: usize) -> Option<&Val> {
        self.inner.get(self.idx + idx)
    }

    fn get_mut(&self, idx: usize) -> Option<&mut Val> {
        self.inner.get_mut(self.idx + idx)
    }

    fn substack<'b>(&'b self, from: usize) -> SubStack<'b> {
        SubStack::new(self.inner, self.idx + from)
    }
}

impl<'a> AsRef<[Val]> for SubStack<'a> {
    fn as_ref(&self) -> &[Val] {
        let r = self.inner.as_ref();

        &r[self.idx..]
    }
}

#[cfg(test)]
mod test {
    use super::super::value::*;
    use super::*;
    use freja_parser::ast::Number;

    #[test]
    fn substack_test() {
        let stack = RootStack::new();
        stack.push(Val::Stack(Value::Number(Number::Integer(1))));
        stack.push(Val::Stack(Value::Number(Number::Integer(2))));
        stack.push(Val::Stack(Value::Number(Number::Integer(3))));

        let sub = stack.substack(1);
        assert_eq!(sub.get(0).unwrap(), &Val::Stack(Value::Number(Number::Integer(2))));

        //let n = stack.pop().unwrap();
    }

}
