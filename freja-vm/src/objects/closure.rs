use super::super::chunk::Chunk;
use super::super::context::Context;
use super::super::error::RuntimeResult;
use super::super::stack::SubStack;
use super::super::utils::Pointer;
use super::super::value::{Val, Value};
use super::function::*;
use std::fmt;
use std::rc::Rc;

#[derive(PartialEq, Debug)]
pub struct Closure {
    pub(crate) function: Pointer<Rc<Function>>,
    upvalues: Vec<Val>,
}

impl Closure {
    pub fn new(function: Pointer<Rc<Function>>, upvalues: Vec<Val>) -> Closure {
        Closure { function, upvalues }
    }

    pub fn chunk(&self) -> &Chunk {
        &self.function.chunk
    }

    pub fn name(&self) -> Option<&str> {
        self.function.name.as_ref().map(|s| s.as_str())
    }

    pub fn upvalues(&self) -> &[Val] {
        &self.upvalues
    }
}

// pub enum CloseurePtr {
//     Stack(Rc<Closure>),
//     Ref(*const Closure),
// }

// impl AsRef<Closure> for CloseurePtr {
//     fn as_ref(&self) -> &Closure {
//         match self {
//             CloseurePtr::Ref(r) => unsafe { &**r },
//             CloseurePtr::Stack(r) => r.as_ref(),
//         }
//     }
// }

// impl fmt::Debug for CloseurePtr {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         <Closure as fmt::Debug>::fmt(self.as_ref(), f)
//     }
// }
