use super::chunk::OpCode;
use super::compiler::Compiler;
use super::error::CompileResult;
use super::objects::*;
use super::value::{value_binary, Value, ValuePtr};
use freja_parser::ast::*;
use heapless::consts::U1024;
use heapless::Vec as HVec;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

// type Stack = HVec<ValuePtr, U1024>;
// type Frames<'a> = HVec<CallFrame2<'a>, U1024>;

// struct CallFrame2<'a> {
//     closure: Rc<Closure>,
//     ip: RefCell<usize>,
//     slots: &'a [ValuePtr],
// }

// impl<'a> CallFrame2<'a> {
//     pub fn new(closure: Rc<Closure>, slots: &'a [ValuePtr]) -> CallFrame2<'a> {
//         CallFrame2 { closure, ip: RefCell::new(0), slots }
//     }

//     pub fn read_byte(&self) -> u8 {
//         let b = self.closure.function.chunk.code[*self.ip.borrow()];
//         *self.ip.borrow_mut() += 1;
//         b
//     }

//     pub fn read_short(&self) -> u16 {
//         let ip = *self.ip.borrow();
//         let mut jump = (self.closure.function.chunk.code[ip] as u16) << 8;
//         jump |= self.closure.function.chunk.code[ip + 1] as u16;
//         *self.ip.borrow_mut() += 2;
//         jump
//     }

//     pub fn read_constant(&self) -> Option<&ValuePtr> {
//         let b = self.read_byte();
//         self.closure.function.chunk.get_constant(b as usize)
//     }
// }

// struct Run<'a> {
//     stack: &'a mut Stack,
//     globals: &'a mut HashMap<String, ValuePtr>,
//     frames: Frames<'a>,
//     //closure: Closure,
// }

// impl<'a> Run<'a> {
//     pub fn new(stack: &'a mut Stack, globals: &'a mut HashMap<String, ValuePtr>, closure: Closure) -> Run<'a> {
//         let mut run = Run { stack, globals, frames: HVec::default() };
//         let cl = Rc::new(closure);
//         run.call(&cl);
//         run
//     }

//     fn push(&mut self, value: &Rc<Value>) {
//         self.stack.push(value.clone()).expect("stack overflow")
//     }

//     fn pop(&mut self) -> Option<ValuePtr> {
//         self.stack.pop()
//     }

//     fn peek(stack: &Stack, distance: usize) -> Option<&ValuePtr> {
//         let i = -1 - distance as i32;
//         let idx = (stack.len() as i32) + i;
//         stack.get(idx as usize)
//     }

//     fn run(stack: &mut Stack, globals: &mut HashMap<String, ValuePtr>, frames: &mut Frames) {
//         let mut frame = frames.last().unwrap(); //.clone();

//         'outer: while *frame.ip.borrow() < frame.closure.function.chunk.code.len() {
//             let instruction = frame.read_byte();
//             let instruction = OpCode::from(instruction);

//             match instruction {
//                 OpCode::Constant => {
//                     stack.push(frame.read_constant().expect("constant").clone());
//                 }
//                 OpCode::Pop => {
//                     stack.pop();
//                 }
//                 OpCode::DefineGlobal => {
//                     let name = frame.read_constant().unwrap();
//                     let value = Run::peek(stack, 0).unwrap().clone();
//                     globals.insert(name.as_string().unwrap().to_string(), value);
//                     stack.pop();
//                 }
//                 OpCode::GetGlobal => {
//                     let name = frame.read_constant().unwrap().as_string().unwrap();

//                     let m = match globals.get(name) {
//                         Some(m) => m.clone(),
//                         None => panic!("undefined variable: {}", name),
//                     };
//                     stack.push(m);
//                 }
//                 OpCode::GetLocal => {
//                     let b = frame.read_byte();
//                     stack.push(frame.slots[b as usize]);
//                 }
//                 OpCode::Return => {
//                     let result = stack.pop().unwrap();
//                     frames.pop();
//                     stack.push(result);
//                     frame = match frames.last().as_mut() {
//                         Some(f) => f, //.clone(),
//                         None => break 'outer,
//                     }
//                 }

//                 OpCode::Call0 | OpCode::Call1 => {
//                     let count = (instruction as u8) - (OpCode::Call0 as u8);

//                     let callee = Run::peek(stack, count as usize).expect("expect callee").clone();
//                     Run::call_value2(stack, frames, &callee, count).unwrap();
//                     frame = frames.last().as_mut().unwrap(); //.clone();
//                 }
//                 OpCode::Closure => {
//                     let fu = frame.read_constant().unwrap().as_function().unwrap().clone();
//                     let cl = Rc::new(Value::Closure(Rc::new(Closure::new(fu))));
//                     stack.push(cl);
//                 }
//                 OpCode::Divide | OpCode::Multiply | OpCode::Add | OpCode::Substract | OpCode::Equal | OpCode::Less | OpCode::Greater => {
//                     let right = stack.pop().unwrap();
//                     let left = stack.pop().unwrap();

//                     let ret = Rc::new(value_binary(left.as_ref(), right.as_ref(), instruction).expect("binary"));
//                     stack.push(ret);
//                 }
//                 OpCode::JumpIfFalse => {
//                     let offset = frame.read_short();
//                     let v = Run::peek(stack, 0).unwrap();
//                     if !v.is_truthy() {
//                         *frame.ip.borrow_mut() += offset as usize;
//                     }
//                 }
//                 OpCode::Not => {
//                     let current = stack.pop().unwrap();
//                     let v = Rc::new(Value::Boolean(!current.is_truthy()));
//                     stack.push(v);
//                 }
//                 OpCode::Nil => {
//                     stack.push(Rc::new(Value::Null));
//                 }
//                 _ => unimplemented!("instruction {:?}", instruction),
//             };
//         }
//     }

//     fn call_value2<'b>(stack: &'b mut Stack, frames: &'b mut Frames<'b>, callee: &Value, count: u8) -> CompileResult<()> {
//         match callee {
//             Value::Closure(cl) => {
//                 Run::call2(stack, frames, cl)?;
//             }
//             Value::Native(native) => {
//                 let len = stack.len();
//                 (native.function)(&stack[len - (count as usize)..len])
//             }
//             _ => unimplemented!("call on {} not implemented", callee),
//         }
//         Ok(())
//     }

//     fn call2<'b>(stack: &'b mut Stack, frames: &'b mut Frames<'b>, closure: &Rc<Closure>) -> CompileResult<()> {
//         // TODO check aritity
//         let a = closure.function.arity;
//         let count = if stack.len() == 0 { 0 } else { a + 1 };

//         let slots = &stack[stack.len() - (count as usize)..]; //.to_vec();
//         stack.truncate(stack.len() - (count as usize));
//         let frame = CallFrame2::new(closure.clone(), slots); // { closure: closure.clone(), ip: RefCell::new(0), slots: slots };
//         frames.push(frame);

//         Ok(())
//     }

//     fn call_value(&'a mut self, callee: &Value, count: u8) -> CompileResult<()> {
//         match callee {
//             Value::Closure(cl) => {
//                 self.call(cl)?;
//             }
//             Value::Native(native) => {
//                 let len = self.stack.len();
//                 (native.function)(&self.stack[len - (count as usize)..len])
//             }
//             _ => unimplemented!("call on {} not implemented", callee),
//         }
//         Ok(())
//     }

//     fn call(&'a mut self, closure: &Rc<Closure>) -> CompileResult<()> {
//         // TODO check aritity
//         let a = closure.function.arity;
//         let count = if self.stack.len() == 0 { 0 } else { a + 1 };

//         let slots = &self.stack[self.stack.len() - (count as usize)..]; //.to_vec();
//         self.stack.truncate(self.stack.len() - (count as usize));
//         let frame = CallFrame2::new(closure.clone(), slots); // { closure: closure.clone(), ip: RefCell::new(0), slots: slots };
//         self.frames.push(frame);

//         Ok(())
//     }
// }

struct CallFrame {
    closure: Rc<Closure>,
    ip: RefCell<usize>,
    idx: usize,
    //slots: Vec<ValuePtr>,
}

impl CallFrame {
    pub fn new(closure: Rc<Closure>, idx: usize) -> CallFrame {
        CallFrame { closure, ip: RefCell::new(0), idx }
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
    stack: HVec<ValuePtr, U1024>,
    globals: HashMap<String, ValuePtr>,
    frames: Vec<Rc<CallFrame>>,
}

impl VM {
    pub fn new() -> VM {
        VM { stack: HVec::new(), frames: Vec::new(), globals: HashMap::new() }
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
                    self.stack.push(self.stack[frame.idx + b as usize].clone()).expect("stack overflow");
                    //self.push(&self.stack[frame.idx + b as usize]);
                }
                OpCode::Return => {
                    let result = self.pop().unwrap();
                    self.frames.pop();
                    self.stack.truncate(frame.idx);
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
                OpCode::Divide | OpCode::Multiply | OpCode::Add | OpCode::Substract | OpCode::Equal | OpCode::Less | OpCode::Greater => {
                    let right = self.pop().unwrap();
                    let left = self.pop().unwrap();

                    let ret = Rc::new(value_binary(left.as_ref(), right.as_ref(), instruction).expect("binary"));
                    self.push(&ret);
                }
                OpCode::JumpIfFalse => {
                    let offset = frame.read_short();
                    let v = self.peek(0).unwrap();
                    if !v.is_truthy() {
                        *frame.ip.borrow_mut() += offset as usize;
                    }
                }
                OpCode::Not => {
                    let current = self.pop().unwrap();
                    let v = Rc::new(Value::Boolean(!current.is_truthy()));
                    self.push(&v);
                }
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
        let a = closure.function.arity;
        let count = if self.stack.len() == 0 { 0 } else { a + 1 };
        let idx = self.stack.len() - (count as usize);
        let frame = CallFrame::new(closure.clone(), idx); // { closure: closure.clone(), ip: RefCell::new(0), slots: slots };
        self.frames.push(Rc::new(frame));

        Ok(())
    }

    #[inline(always)]
    fn push(&mut self, value: &Rc<Value>) {
        self.stack.push(value.clone()).expect("stack overflow")
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
    }
}
