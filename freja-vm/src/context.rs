use super::chunk::OpCode;
use super::compiler::Compiler;
use super::error::RuntimeResult;
use super::frames::{CallFrame, Frames};
use super::objects::Native;
use super::objects::*;
use super::runner::{call_value, run as run_frame, Globals};
use super::stack::{RootStack, Stack};
use super::value::*;
use std::cell::RefCell;
use std::rc::Rc;

pub type Idx = usize;

#[derive(Debug)]
pub struct Context<S: Stack> {
    pub(crate) stack: S,
    pub(crate) frames: Frames,
    pub(crate) globals: Rc<RefCell<Globals>>,
}

impl<S: Stack> Context<S> {
    pub fn new(stack: S, globals: Rc<RefCell<Globals>>, frames: Frames) -> Context<S> {
        Context {
            stack: stack,
            frames,
            globals,
        }
    }
    pub fn push(&self, val: Value) -> RuntimeResult<&Self> {
        self.stack.push(Val::Stack(val))?;
        Ok(self)
    }
    pub fn pop(&self) -> Option<Val> {
        self.stack.pop()
    }

    pub fn get(&self, idx: Idx) -> Option<&Val> {
        self.stack.get(idx)
    }

    pub fn get_mut(&self, idx: Idx) -> Option<&mut Val> {
        self.stack.get_mut(idx)
    }

    pub fn dup(&self, idx: Idx) -> RuntimeResult<&Self> {
        match self.get_mut(idx) {
            Some(m) => {
                self.stack.push(m.into_heap().clone())?;
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

    // pub fn get(&self, idx: Idx) -> RuntimeResult<&Value> {}
    pub fn eval_script<Str: AsRef<str>>(&self, source: Str) -> RuntimeResult<()> {
        let ast = freja_parser::parser::program(source.as_ref())?;
        let fu = Compiler::new().compile(&ast)?;
        let cl = Rc::new(Closure::new(Rc::new(fu), Vec::new()));
        self.stack.push(Val::Stack(Value::Closure(cl.clone())))?;
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

#[derive(Debug)]
pub struct VM {
    ctx: Context<RootStack>,
}

impl VM {
    pub fn new() -> VM {
        let vm = VM {
            ctx: Context::new(
                RootStack::new(),
                Rc::new(RefCell::new(Globals::default())),
                Frames::new(),
            ),
        };

        vm.ctx.globals.borrow_mut().insert(
            "print".to_string(),
            Rc::new(Value::Native(Rc::new(Native {
                arity: 1,
                function: Box::new(|ctx| {
                    let v = ctx.get(0).expect("zero");
                    println!("{}", v);
                    Ok(Value::Null)
                }),
            }))),
        );

        vm
    }

    pub fn eval_script<S: AsRef<str>>(&self, source: S) -> RuntimeResult<()> {
        self.ctx.eval_script(source)
    }

    pub fn call(&self, args_count: u32) -> RuntimeResult<()> {
        self.ctx.call(args_count)
    }
}
