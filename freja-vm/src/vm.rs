use super::chunk::OpCode;
use super::compiler::Compiler;
use super::error::CompileResult;
use super::objects::*;
use super::value::{value_binary, Val, Value, ValuePtr};
use freja_parser::ast::*;
use heapless::consts::{U256, U64};
use heapless::Vec as HVec;
use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

pub type Stack = HVec<Val, U256>;
type Frames = HVec<CallFrame, U64>;
pub type Globals = HashMap<String, ValuePtr>;

#[derive(Debug)]
struct CallFrame {
    closure: Rc<Closure>,
    ip: Cell<usize>,
    idx: usize,
}

impl CallFrame {
    pub fn new(closure: Rc<Closure>, idx: usize) -> CallFrame {
        CallFrame { closure, ip: Cell::new(0), idx }
    }

    pub fn read_byte(&self) -> u8 {
        let ip = self.ip.get();
        let b = self.closure.function.chunk.code[ip];
        self.ip.set(ip + 1);
        b
    }

    pub fn read_short(&self) -> u16 {
        let ip = self.ip.get();
        let mut jump = (self.closure.function.chunk.code[ip] as u16) << 8;
        jump |= self.closure.function.chunk.code[ip + 1] as u16;
        self.ip.set(ip + 2);
        jump
    }

    pub fn read_constant(&self) -> Option<&ValuePtr> {
        let b = self.read_byte();
        self.closure.function.chunk.get_constant(b as usize)
    }
}

pub struct VM {
    stack: Stack,
    globals: HashMap<String, ValuePtr>,
    // frames: HVec<Rc<CallFrame>, U64>,
    frames: Frames,
}

impl VM {
    pub fn new() -> VM {
        VM { stack: HVec::new(), frames: Frames::new(), globals: HashMap::new() }
    }

    pub fn push_native<F: 'static>(&mut self, name: &str, fu: F)
    where
        F: Fn(&[Val]),
    {
        let value = Value::Native(Rc::new(Native { function: Box::new(fu) }));
        self.globals.insert(name.to_string(), Rc::new(value));
    }

    pub fn dump(&self) -> String {
        self.stack.iter().map(|m| m.to_string()).collect::<Vec<_>>().join(", ")
    }

    pub fn dump_global(&self) -> String {
        self.globals.iter().map(|m| format!("<Key={}, Value={}>", m.0, m.1)).collect::<Vec<_>>().join(", ")
    }

    pub fn interpret_ast(&mut self, ast: &ProgramStmt) -> CompileResult<()> {
        let fu = Compiler::new().compile(ast)?;
        let cl = Closure::new(Rc::new(fu));
        // self.call_value(&Value::Closure(Rc::new(cl)), 0)?;
        // self.run();
        call_value(&self.stack, &mut self.frames, &Value::Closure(Rc::new(cl)), 0)?;
        run(&mut self.frames, &mut self.stack, &mut self.globals);
        Ok(())
    }
    /*
    fn run(&mut self) {
        let mut frame = self.frames.last().unwrap().clone();

        'outer: while frame.ip.get() < frame.closure.function.chunk.code.len() {
            let instruction = frame.read_byte();
            let instruction = OpCode::from(instruction);

            match instruction {
                OpCode::Constant => self.push(frame.read_constant().expect("constant").clone()),
                OpCode::Pop => {
                    self.pop();
                }
                OpCode::DefineGlobal => {
                    let name = frame.read_constant().unwrap();
                    let value = self.peek(0).unwrap().clone();
                    self.globals.insert(name.as_string().unwrap().to_string(), value);
                    self.pop();
                }
                OpCode::GetGlobal => {
                    let name = frame.read_constant().unwrap().as_string().unwrap();

                    let m = match self.globals.get(name) {
                        Some(m) => m.clone(),
                        None => panic!("undefined variable: {}", name),
                    };
                    self.push(m);
                }
                OpCode::GetLocal => {
                    let b = frame.read_byte();
                    self.stack.push(self.stack[frame.idx + b as usize].clone()).expect("stack overflow");
                    //self.push(&self.stack[frame.idx + b as usize]);
                }
                OpCode::Return => {
                    let result = self.pop().unwrap();
                    self.frames.pop();
                    self.stack.truncate(frame.idx);
                    self.push(result);
                    frame = match self.frames.last() {
                        Some(f) => f.clone(),
                        None => break 'outer,
                    }
                }

                OpCode::Call0 | OpCode::Call1 => {
                    let count = (instruction as u8) - (OpCode::Call0 as u8);
                    let callee = self.peek(count as usize).expect("expect callee").clone();
                    self.call_value(&callee, count).unwrap();
                    frame = self.frames.last().unwrap().clone();
                }
                OpCode::Closure => {
                    let fu = frame.read_constant().unwrap().as_function().unwrap().clone();
                    let cl = Rc::new(Value::Closure(Rc::new(Closure::new(fu))));
                    self.push(cl);
                }
                OpCode::Divide | OpCode::Multiply | OpCode::Add | OpCode::Substract | OpCode::Equal | OpCode::Less | OpCode::Greater => {
                    let right = self.pop().unwrap();
                    let left = self.pop().unwrap();

                    let ret = Rc::new(value_binary(left.as_ref(), right.as_ref(), instruction).expect("binary"));
                    self.push(ret);
                }
                OpCode::JumpIfFalse => {
                    let offset = frame.read_short();
                    let v = self.peek(0).unwrap();
                    if !v.is_truthy() {
                        let ip = frame.ip.get();
                        frame.ip.set(ip + offset as usize);
                    }
                }
                OpCode::Not => {
                    let current = self.pop().unwrap();
                    let v = Rc::new(Value::Boolean(!current.is_truthy()));
                    self.push(v);
                }
                OpCode::Nil => self.push(Rc::new(Value::Null)),
                _ => unimplemented!("instruction {:?}", instruction),
            };
        }
    }

    fn call_value(&mut self, callee: &Value, count: u8) -> CompileResult<()> {
        match callee {
            Value::Closure(cl) => {
                self.call(cl)?;
            }
            Value::Native(native) => {
                let len = self.stack.len();
                (native.function)(&self.stack[len - (count as usize)..len])
            }
            _ => unimplemented!("call on {} not implemented", callee),
        }
        Ok(())
    }

    fn call(&mut self, closure: &Rc<Closure>) -> CompileResult<()> {
        // TODO check aritity
        let a = closure.function.arity;
        let count = if self.stack.len() == 0 { 0 } else { a + 1 };
        let idx = self.stack.len() - (count as usize);
        let frame = CallFrame::new(closure.clone(), idx); // { closure: closure.clone(), ip: RefCell::new(0), slots: slots };
        self.frames.push(Rc::new(frame)).expect("to many frames");

        Ok(())
    }

    #[inline(always)]
    fn push(&mut self, value: Rc<Value>) {
        self.stack.push(value).expect("stack overflow")
    }

    #[inline(always)]
    fn pop(&mut self) -> Option<ValuePtr> {
        self.stack.pop()
    }

    #[inline(always)]
    fn peek(&mut self, distance: usize) -> Option<&ValuePtr> {
        let i = -1 - distance as i32;
        let idx = (self.stack.len() as i32) + i;
        self.stack.get(idx as usize)
    }*/
}

macro_rules! push {
    ($stack: expr, $val: expr) => {
        $stack.push($val)
    };
}

macro_rules! pop {
    ($stack: expr) => {
        $stack.pop()
    };
}

macro_rules! peek {
    ($stack: expr, $distance: expr) => {{
        let i = -1 - $distance as i32;
        let idx = ($stack.len() as i32) + i;
        $stack.get(idx as usize)
    }};
}

fn run(frames: &mut Frames, stack: &mut Stack, globals: &mut Globals) {
    let mut frame = frames.len() - 1;

    'outer: while frames[frame].ip.get() < frames[frame].closure.function.chunk.code.len() {
        let instruction = frames[frame].read_byte();
        let instruction = OpCode::from(instruction);
        match instruction {
            OpCode::Constant => push!(stack, Val::Heap(frames[frame].read_constant().expect("constant").clone())).expect("stack overflow"),
            OpCode::Pop => {
                pop!(stack);
            }
            OpCode::DefineGlobal => {
                let name = frames[frame].read_constant().unwrap();
                let value = pop!(stack).unwrap();
                globals.insert(name.as_string().unwrap().to_string(), value.into_heap());
            }
            OpCode::GetGlobal => {
                let name = frames[frame].read_constant().unwrap().as_string().unwrap();

                let m = match globals.get(name) {
                    Some(m) => m.clone(),
                    None => panic!("undefined variable: {}", name),
                };
                push!(stack, Val::Heap(m));
            }
            OpCode::GetLocal => {
                let b = frames[frame].read_byte();
                let idx = frames[frame].idx + b as usize;
                stack[idx].into_heap2();
                stack.push(stack[idx].clone()).expect("stack overflow");
            }
            OpCode::Return => {
                let result = pop!(stack).unwrap();
                stack.truncate(frames[frame].idx);
                frames.pop();
                push!(stack, result);
                if frames.is_empty() {
                    break 'outer;
                }
                frame = frames.len() - 1;
            }

            OpCode::Call0 | OpCode::Call1 => {
                let count = (instruction as u8) - (OpCode::Call0 as u8);
                let callee = peek!(stack, count as usize).expect("expect callee").clone();
                call_value(stack, frames, callee.as_value(), count).unwrap();
                frame = frames.len() - 1
            }
            OpCode::Closure => {
                let fu = frames[frame].read_constant().unwrap().as_function().unwrap().clone();
                let cl = Value::Closure(Rc::new(Closure::new(fu)));
                push!(stack, Val::Stack(cl));
            }
            OpCode::Divide | OpCode::Multiply | OpCode::Add | OpCode::Substract | OpCode::Equal | OpCode::Less | OpCode::Greater => {
                let right = pop!(stack).unwrap();
                let left = pop!(stack).unwrap();

                let ret = value_binary(&left, &right, instruction).expect("binary");
                push!(stack, Val::Stack(ret));
            }
            OpCode::JumpIfFalse => {
                let offset = frames[frame].read_short();
                let v = peek!(stack, 0).unwrap();
                if !v.is_truthy() {
                    let ip = frames[frame].ip.get();
                    frames[frame].ip.set(ip + offset as usize);
                }
            }
            OpCode::Not => {
                let current = pop!(stack).unwrap();
                let v = Value::Boolean(!current.is_truthy());
                push!(stack, Val::Stack(v));
            }
            OpCode::Nil => push!(stack, Val::Stack(Value::Null)).expect("stack overflow"),
            _ => unimplemented!("instruction {:?}", instruction),
        };
    }
}

fn call_value(stack: &Stack, frames: &mut Frames, callee: &Value, count: u8) -> CompileResult<()> {
    match callee {
        Value::Closure(cl) => {
            call(stack, frames, cl)?;
        }
        Value::Native(native) => {
            let len = stack.len();
            (native.function)(&stack[len - (count as usize)..len])
        }
        _ => unimplemented!("call on {} not implemented", callee),
    }
    Ok(())
}

fn call(stack: &Stack, frames: &mut Frames, closure: &Rc<Closure>) -> CompileResult<()> {
    // TODO check aritity
    let a = closure.function.arity;

    let count = if stack.len() == 0 { 0 } else { a + 1 };
    let idx = stack.len() - (count as usize);
    let frame = CallFrame::new(closure.clone(), idx); // { closure: closure.clone(), ip: RefCell::new(0), slots: slots };
    frames.push(frame).expect("too many frames");

    Ok(())
}
