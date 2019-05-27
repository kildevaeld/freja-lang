use super::context::Context;
use super::error::RuntimeResult;
use super::frames::{CallFrame, Frames};
use super::objects::Native;
use super::objects::*;
use super::runner::{call_value, run as run_frame, Globals};
use super::stack::{RootStack, Stack, SubStack};
use super::value::*;
use heapless::consts::U8;
use std::cell::RefCell;
use std::rc::Rc;

fn freja_print(ctx: &Context<SubStack>) -> RuntimeResult<Value> {
    let v = ctx.get(0).expect("zero");
    println!("{}", v);
    Ok(Value::Null)
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

        vm.ctx
            .globals
            .borrow_mut()
            .insert("print".to_string(), NativeFn::value(&freja_print, 1));
        vm
    }

    pub fn eval_script<S: AsRef<str>>(&self, source: S) -> RuntimeResult<()> {
        self.ctx.eval_script(source)
    }

    pub fn call(&self, args_count: u32) -> RuntimeResult<()> {
        self.ctx.call(args_count)
    }
}
