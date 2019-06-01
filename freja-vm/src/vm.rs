use super::context::Context;
use super::error::RuntimeResult;
use super::frames::Frames;
use super::stack::*;
use super::globals::Global;
use super::objects::*;
use super::value::*;
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
            ctx: Context::new(RootStack::new(), Rc::new(Global::default()), Frames::new()),
        };

        vm.ctx
            .globals
            .insert("print".to_string(), NativeFn::value(&freja_print, 1));

        vm.ctx
            .globals
            .insert("Array".to_string(), Value::Class(Rc::new(Box::new(Array::new()))));

        vm.ctx
            .globals
            .insert("Map".to_string(), Value::Class(Rc::new(Box::new(Map::new()))));

        vm
    }

    pub fn eval_script<S: AsRef<str>>(&self, source: S) -> RuntimeResult<()> {
        self.ctx.eval_script(source)
    }

    pub fn call(&self, args_count: u32) -> RuntimeResult<()> {
        self.ctx.call(args_count)
    }
}
