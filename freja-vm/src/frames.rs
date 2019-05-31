use super::error::{RuntimeError, RuntimeResult};
use super::objects::*;
use super::utils::Pointer;
use heapless::consts::U256;
use heapless::ArrayLength;
use heapless::Vec as HVec;
use std::cell::Cell;
use std::rc::Rc;
use freja_compiler::Constant;

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
    pub(crate) closure: Pointer<Rc<Closure>>,
    pub(crate) ip: Cell<usize>,
    pub(crate) idx: usize,
}

impl CallFrame {
    pub fn new(closure: Pointer<Rc<Closure>>, idx: usize) -> CallFrame {
        CallFrame {
            closure,
            ip: Cell::new(0),
            idx,
        }
    }

    #[inline(always)]
    pub fn read_byte(&self) -> u8 {
        let ip = self.ip.get();
        // let b = self.closure.chunk().code[ip];
        let b = self.closure.chunk().get(ip);

        self.ip.set(ip + 1);
        b
    }

    #[inline(always)]
    pub fn read_short(&self) -> u16 {
        let ip = self.ip.get();
        let mut jump = (self.closure.chunk().get(ip) as u16) << 8;
        jump |= self.closure.chunk().get(ip + 1) as u16;
        self.ip.set(ip + 2);
        jump
    }

    #[inline(always)]
    pub fn read_constant(&self) -> Option<&Constant> {
        let b = self.read_byte();
        self.closure.chunk().get_constant(b as usize)
    }
}
