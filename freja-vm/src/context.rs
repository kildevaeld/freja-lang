use super::error::RuntimeResult;
use super::frames::Frames;
use super::globals::Global;
use super::runner::{call_value, run as run_frame};
use super::stack::{Stack, SubStack};
use super::utils::Pointer;
use super::value::*;
use freja_compiler::Compiler;
use std::rc::Rc;
use super::objects::Closure;

pub type Idx = usize;

#[derive(Debug)]
pub struct Context<S: Stack> {
    pub(crate) stack: S,
    pub(crate) frames: Frames,
    pub(crate) globals: Rc<Global>,
}

impl<S: Stack> Context<S> {
    pub fn new(stack: S, globals: Rc<Global>, frames: Frames) -> Context<S> {
        Context {
            stack: stack,
            frames,
            globals,
        }
    }

    pub fn child(&self, idx: usize) -> Context<SubStack> {
        Context::new(self.stack.substack(idx), self.globals.clone(), Frames::new())
    }

    pub fn push(&self, val: Value) -> RuntimeResult<&Self> {
        self.stack.push(Pointer::Stack(val))?;
        Ok(self)
    }
    pub fn pop(&self) -> Option<Val> {
        self.stack.pop()
    }

    pub fn set(&self, idx: usize, val: Value) {
        self.stack.set(idx, Pointer::Stack(val))
    }

    pub fn get(&self, idx: Idx) -> Option<&Val> {
        self.stack.get(idx)
    }

    pub fn get_mut(&self, idx: Idx) -> Option<&mut Val> {
        self.stack.get_mut(idx)
    }

    pub fn peek(&self, idx: Idx) -> Option<&Val> {
        self.stack.peek(idx as i32)
    }

    pub fn dup(&self, idx: Idx) -> RuntimeResult<&Self> {
        match self.get_mut(idx) {
            Some(m) => {
                self.stack.push(m.clone())?;
                ()
            }
            None => {
                self.push(Value::Null)?;
                ()
            }
        };
        Ok(self)
    }

    pub fn top(&self) -> usize {
        self.stack.len()
    }

    pub fn eval_script<Str: AsRef<str>>(&self, source: Str) -> RuntimeResult<()> {
        let ast = freja_parser::parser::program(source.as_ref())?;
        let fu = Compiler::new().compile(&ast)?;
        let cl = Closure::new(Pointer::Stack(Rc::new(fu)), Vec::new());
        self.stack.push(Pointer::Stack(Value::Closure(Rc::new(cl))))?;
        self.call(0)
    }

    pub fn call(&self, args_count: u32) -> RuntimeResult<()> {
        if args_count > 8 {
            return Err("max args".into());
        }

        let val = self.stack.peek(args_count as i32).expect("function");

        call_value(self, val, args_count as u8)?;
        self.run()?;
        Ok(())
    }

    pub fn dump(&self) -> String {
        format!(
            "[{}]",
            self.stack
                .as_ref()
                .iter()
                .map(|m| m.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }

    fn run(&self) -> RuntimeResult<()> {
        run_frame(self)
    }
}
