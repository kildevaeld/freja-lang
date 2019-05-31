use super::super::chunk::Chunk;
use super::super::utils::Pointer;
use super::super::value::Val;
use super::function::*;
use std::rc::Rc;

#[derive(PartialEq, Debug)]
pub struct Closure {
    function: Pointer<Rc<Function>>,
    upvalues: Vec<Val>,
}

impl Closure {
    pub fn new(function: Pointer<Rc<Function>>, upvalues: Vec<Val>) -> Closure {
        Closure { function, upvalues }
    }

    #[inline(always)]
    pub fn chunk(&self) -> &Chunk {
        &self.function.chunk
    }

    #[inline(always)]
    pub fn arity(&self) -> i32 {
        self.function.arity
    }

    pub fn name(&self) -> Option<&str> {
        self.function.name.as_ref().map(|s| s.as_str())
    }

    #[inline(always)]
    pub fn upvalues(&self) -> &[Val] {
        &self.upvalues
    }
}
