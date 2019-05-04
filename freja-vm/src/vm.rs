use super::chunk::OpCode;
use super::compiler::Compiler;
use super::error::CompileResult;
use super::value::{Closure, Native, Value, ValuePtr};
use freja_parser::ast::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

struct CallFrame {
    closure: Rc<Closure>,
    ip: RefCell<usize>,
    slots: Vec<ValuePtr>,
}

impl CallFrame {
    pub fn new(closure: Rc<Closure>) -> CallFrame {
        CallFrame { closure, ip: RefCell::new(0), slots: Vec::new() }
    }

    pub fn read_byte(&self) -> u8 {
        let b = self.closure.function.chunk.code[*self.ip.borrow()];
        *self.ip.borrow_mut() += 1;
        b
    }

    pub fn read_short(&self) -> u16 {
        let ip = *self.ip.borrow();
        let mut jump = (self.closure.function.chunk.code[ip] as u16) << 8;
        jump |= self.closure.function.chunk.code[ip + 1] as u16;
        *self.ip.borrow_mut() += 2;
        jump
    }

    pub fn read_constant(&self) -> Option<&ValuePtr> {
        let b = self.read_byte();
        self.closure.function.chunk.get_constant(b as usize)
    }
}

pub struct VM {
    stack: Vec<ValuePtr>,
    globals: HashMap<String, ValuePtr>,
    frames: Vec<Rc<CallFrame>>,
}

impl VM {
    pub fn new() -> VM {
        VM { stack: Vec::new(), frames: Vec::new(), globals: HashMap::new() }
    }

    pub fn push_native<F: 'static>(&mut self, name: &str, fu: F)
    where
        F: Fn(&[ValuePtr]),
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
        self.call_value(&Value::Closure(Rc::new(cl)), 0)?;
        self.run();
        Ok(())
    }

    fn run(&mut self) {
        let mut frame = self.frames.last().unwrap().clone();

        'outer: while *frame.ip.borrow() < frame.closure.function.chunk.code.len() {
            let instruction = frame.read_byte();
            let instruction = OpCode::from(instruction);

            match instruction {
                OpCode::Constant => self.push(frame.read_constant().expect("constant")),
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
                    self.push(&m);
                }
                OpCode::GetLocal => {
                    let b = frame.read_byte();
                    // println!("{} {}", frame.slots.iter().map(|m| m.to_string()).collect::<Vec<_>>().join(", "), b);
                    //unimplemented!("{:?}", instruction);
                    self.push(&frame.slots[b as usize]);
                }
                OpCode::Return => {
                    let result = self.pop().unwrap();
                    self.frames.pop();
                    self.push(&result);
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
                    self.push(&cl);
                }
                OpCode::Divide | OpCode::Multiply | OpCode::Add | OpCode::Substract => {}
                OpCode::Nil => self.push(&Rc::new(Value::Null)),
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

        let slots = self.stack[self.stack.len() - (closure.function.arity) as usize..].to_vec();
        let frame = CallFrame { closure: closure.clone(), ip: RefCell::new(0), slots: slots };
        self.frames.push(Rc::new(frame));

        Ok(())
    }

    fn push(&mut self, value: &Rc<Value>) {
        self.stack.push(value.clone())
    }

    fn pop(&mut self) -> Option<ValuePtr> {
        self.stack.pop()
    }

    fn peek(&mut self, distance: usize) -> Option<&ValuePtr> {
        let i = -1 - distance as i32;
        let idx = (self.stack.len() as i32) + i;
        self.stack.get(idx as usize)
    }
}
