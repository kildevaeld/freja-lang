use super::chunk::OpCode;
use super::compiler::Compiler;
use super::error::{CompileError, CompileResult};
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
        CallFrame {
            closure,
            ip: Cell::new(0),
            idx,
        }
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
        VM {
            stack: HVec::new(),
            frames: Frames::new(),
            globals: HashMap::new(),
        }
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
        self.globals
            .iter()
            .map(|m| format!("<Key={}, Value={}>", m.0, m.1))
            .collect::<Vec<_>>()
            .join(", ")
    }

    pub fn interpret_ast(&mut self, ast: &ProgramStmt) -> CompileResult<()> {
        let fu = Compiler::new().compile(ast)?;
        let cl = Closure::new(Rc::new(fu));
        // self.call_value(&Value::Closure(Rc::new(cl)), 0)?;
        // self.run();
        call_value(&mut self.stack, &mut self.frames, &Value::Closure(Rc::new(cl)), 0)?;
        run(&mut self.frames, &mut self.stack, &mut self.globals);
        Ok(())
    }
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

macro_rules! dump_stack {
    ($stack: expr) => {
        println!(
            "{}",
            $stack.iter().map(|m| m.to_string()).collect::<Vec<_>>().join(", ")
        )
    };
}

fn run(frames: &mut Frames, stack: &mut Stack, globals: &mut Globals) {
    let mut frame = frames.len() - 1;

    'outer: while frames[frame].ip.get() < frames[frame].closure.function.chunk.code.len() {
        let instruction = frames[frame].read_byte();
        let instruction = OpCode::from(instruction);
        match instruction {
            OpCode::Constant => push!(
                stack,
                Val::Heap(frames[frame].read_constant().expect("constant").clone())
            )
            .expect("stack overflow"),
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
            OpCode::Invoke0 | OpCode::Invoke1 | OpCode::Invoke2 => {
                let count = (instruction as u8) - (OpCode::Invoke0 as u8);
                let method = frames[frame].read_constant().unwrap().as_string().unwrap().clone();
                invoke(stack, frames, method.as_str(), count).unwrap();
                frame = frames.len() - 1;
            }
            OpCode::Closure => {
                let fu = frames[frame].read_constant().unwrap().as_function().unwrap().clone();
                let cl = Value::Closure(Rc::new(Closure::new(fu)));
                push!(stack, Val::Stack(cl));
            }
            OpCode::Divide
            | OpCode::Multiply
            | OpCode::Add
            | OpCode::Substract
            | OpCode::Equal
            | OpCode::Less
            | OpCode::Greater => {
                let right = pop!(stack).unwrap();
                let left = pop!(stack).unwrap();

                let ret = value_binary(&left, &right, instruction).expect("binary");
                push!(stack, Val::Stack(ret));
            }
            OpCode::Array => {
                let o = frames[frame].read_byte();

                if o == 0 {
                    push!(stack, Val::Stack(Value::Array(Array::default())));
                } else {
                    let mut v = Vec::new();
                    let idx = stack.len() - o as usize;
                    for i in stack.iter_mut().skip(idx) {
                        v.push(i.into_heap2().clone());
                    }
                    stack.truncate(idx);
                    push!(stack, Val::Stack(Value::Array(Array::new(v))));
                }
            }
            OpCode::JumpIfFalse => {
                let offset = frames[frame].read_short();
                let v = peek!(stack, 0).unwrap();
                if !v.is_truthy() {
                    let ip = frames[frame].ip.get();
                    frames[frame].ip.set(ip + offset as usize);
                }
            }
            OpCode::Jump => {
                let offset = frames[frame].read_short();
                let ip = frames[frame].ip.get();
                frames[frame].ip.set(ip + offset as usize);
            }
            OpCode::Not => {
                let current = pop!(stack).unwrap();
                let v = Value::Boolean(!current.is_truthy());
                push!(stack, Val::Stack(v));
            }
            // OpCode::Property => {
            //     let prop = pop!(stack).unwrap();
            //     let object = pop!(stack).unwrap();

            //     println!("OBJECT {}, PROP {}", object, prop);
            // }
            OpCode::Class => {
                let name = frames[frame].read_constant().unwrap().as_string().unwrap();
                let class = Class::new(name.to_owned());
                push!(stack, Val::Stack(Value::Class(Rc::new(class))));
            }
            OpCode::Method => {
                let name = frames[frame].read_constant().unwrap().as_string().unwrap();
                let method = pop!(stack).unwrap().as_closure().unwrap().clone();
                let class = pop!(stack).unwrap().as_class().unwrap().clone();
                class.add_method(name.to_owned(), method.clone());
            }
            OpCode::Inherit => {
                let super_c = peek!(stack, 1).unwrap();
                let super_c = match super_c.as_class() {
                    Some(c) => c,
                    None => panic!("super"),
                };
                let subclass = peek!(stack, 0).unwrap().as_class().unwrap();
                subclass.inherit(super_c);
                pop!(stack);
            }
            OpCode::Nil => push!(stack, Val::Stack(Value::Null)).expect("stack overflow"),
            _ => unimplemented!("instruction {:?}", instruction),
        };
    }
}

fn call_value(stack: &mut Stack, frames: &mut Frames, callee: &Value, count: u8) -> CompileResult<()> {
    match callee {
        Value::Closure(cl) => {
            call(stack, frames, cl)?;
        }
        Value::Class(cl) => {
            //
            let s = if count == 0 { 0 } else { count - 1 };

            stack[s as usize] = Val::Stack(Value::Instance(ClassInstance::new(cl.clone())));
            if let Some(initializer) = cl.methods.borrow().get("init") {
                call(stack, frames, initializer)?;
            }
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

fn invoke_from_class(stack: &Stack, frames: &mut Frames, class: &ClassInstance, name: &str) -> CompileResult<()> {
    let methods = class.class.methods.borrow();
    let method = match methods.get(name) {
        Some(m) => m,
        None => return Err(CompileError::InvalidReceiver),
    };

    call(stack, frames, method)?;

    Ok(())
}

fn invoke(stack: &Stack, frames: &mut Frames, name: &str, count: u8) -> CompileResult<()> {
    let receiver = peek!(stack, count).unwrap();

    let instance = match receiver.as_instance() {
        Some(s) => s,
        None => return Err(CompileError::InvalidReceiver),
    };

    invoke_from_class(stack, frames, receiver.as_instance().unwrap(), name)
}
