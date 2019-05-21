use super::error::{RuntimeError, RuntimeResult};
use super::objects::*;
use super::value::*;
use heapless::consts::U256;
use heapless::ArrayLength;
use heapless::Vec as HVec;
use std::cell::Cell;

#[derive(Debug)]
pub struct Frames<N: ArrayLength<CallFrame> = U256> {
    inner: std::cell::UnsafeCell<HVec<CallFrame, N>>,
}

impl Frames {
    pub fn new() -> Frames {
        Frames {
            inner: std::cell::UnsafeCell::new(HVec::default()),
        }
    }

    pub fn push(&self, frame: CallFrame) -> RuntimeResult<()> {
        unsafe {
            (&mut *self.inner.get())
                .push(frame)
                .map_err(|_| RuntimeError::StackOverflow)
        }
    }

    pub fn pop(&self) -> Option<CallFrame> {
        unsafe { (&mut *self.inner.get()).pop() }
    }

    pub fn last(&self) -> Option<&CallFrame> {
        unsafe { (&mut *self.inner.get()).last() }
    }

    pub fn is_empty(&self) -> bool {
        unsafe { (&*self.inner.get()).is_empty() }
    }
}

#[derive(Debug)]
pub struct CallFrame {
    pub(crate) closure: CloseurePtr,
    pub(crate) ip: Cell<usize>,
    pub(crate) idx: usize,
}

impl CallFrame {
    pub fn new(closure: CloseurePtr, idx: usize) -> CallFrame {
        CallFrame {
            closure,
            ip: Cell::new(0),
            idx,
        }
    }

    pub fn read_byte(&self) -> u8 {
        let ip = self.ip.get();
        let b = self.closure.as_ref().function.chunk.code[ip];
        self.ip.set(ip + 1);
        b
    }

    pub fn read_short(&self) -> u16 {
        let ip = self.ip.get();
        let mut jump = (self.closure.as_ref().function.chunk.code[ip] as u16) << 8;
        jump |= self.closure.as_ref().function.chunk.code[ip + 1] as u16;
        self.ip.set(ip + 2);
        jump
    }

    pub fn read_constant(&self) -> Option<&ValuePtr> {
        let b = self.read_byte();
        self.closure.as_ref().function.chunk.get_constant(b as usize)
    }
}
