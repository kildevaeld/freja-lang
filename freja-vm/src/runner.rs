use super::chunk::OpCode;
use super::compiler::Compiler;
use super::context::Context;
use super::error::{RuntimeError, RuntimeResult};
use super::frames::*;
use super::objects::*;
use super::stack::{RootStack, Stack};
use super::value::*;
use freja_parser::ast::*;
use std::collections::HashMap;
use std::rc::Rc;

// pub type Stack = HVec<Val, U256>;
pub type Globals = HashMap<String, ValuePtr>;

macro_rules! push {
    ($stack: expr, $val: expr) => {
        $stack.push($val).map_err(|_| RuntimeError::StackOverflow)
    };
}

macro_rules! pop {
    ($stack: expr) => {
        $stack.pop()
    };
}

macro_rules! peek {
    ($stack: expr, $distance: expr) => {{
        $stack.peek($distance)
    }};
}

macro_rules! peek_mut {
    ($stack: expr, $distance: expr) => {{
        $stack.peek_mut($distance)
    }};
}

#[allow(unused_macros)]
macro_rules! dump_stack {
    ($stack: expr) => {
        println!(
            "stack [{}]",
            $stack
                .as_ref()
                .iter()
                .map(|m| m.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )
    };
}

pub(crate) fn run<S: Stack>(ctx: &Context<S>) -> RuntimeResult<()> {
    let mut frame = ctx.frames.last().unwrap(); // frames.len() - 1;
    let stack = &ctx.stack;

    'outer: while frame.ip.get() < frame.closure.as_ref().chunk().code.len() {
        let instruction = frame.read_byte();
        let instruction = OpCode::from(instruction);

        match instruction {
            OpCode::Constant => {
                let val = frame.read_constant().expect("constant").as_ref() as *const Value;
                push!(stack, Val::Ref(val))?;
            }
            OpCode::Pop => {
                pop!(stack);
            }
            OpCode::DefineGlobal => {
                let name = frame.read_constant().unwrap();
                let value = pop!(stack).unwrap();
                ctx.globals
                    .borrow_mut()
                    .insert(name.as_string().unwrap().to_string(), value.into_value());
            }
            OpCode::GetGlobal => {
                let name = frame.read_constant().unwrap().as_string().unwrap();
                let m = match ctx.globals.borrow().get(name) {
                    Some(m) => m.as_ref() as *const Value,
                    None => panic!("undefined variable: {}", name),
                };
                push!(stack, Val::Ref(m))?;
            }
            OpCode::GetLocal => {
                let b = frame.read_byte();
                let idx = frame.idx + b as usize;
                let val = stack
                    .get(idx)
                    .expect(format!("get local at idx {}", idx).as_str())
                    .as_ref() as *const Value;
                push!(stack, Val::Ref(val))?;
            }
            OpCode::SetLocal => {
                let b = frame.read_byte();
                let val = peek_mut!(stack, 0).unwrap();
                let idx = frame.idx + b as usize;
                stack.set(idx, val.into_heap().clone());
            }
            OpCode::SetGlobal => {
                let name = frame.read_constant().unwrap().as_string().unwrap();
                if !ctx.globals.borrow().contains_key(name.as_str()) {
                    return Err(RuntimeError::InvalidIndex);
                }
                let val = peek!(stack, 0).unwrap();
                ctx.globals.borrow_mut().insert(name.clone(), val.clone().into_value());
            }

            OpCode::SetProperty => {
                let name = frame.read_constant().unwrap().as_string().unwrap();

                let value = peek_mut!(stack, 0).expect("property expected property");
                let receiver = peek!(stack, 1).expect("property expected receiver");

                let instance = match receiver.as_instance() {
                    Some(i) => i,
                    None => return Err(format!("can only set property on an instance, got: {}", receiver).into()),
                };

                pop!(stack);
                pop!(stack);

                instance.set_field(name, value.into_heap().clone())?;
            }
            OpCode::GetProperty => {
                let name = frame.read_constant().unwrap().as_string().unwrap();
                let receiver = pop!(stack).expect("get property receiver");

                let instance = match receiver.as_instance() {
                    Some(i) => i,
                    None => return Err("canget property on an instance".into()),
                };
                match instance.get_field(name) {
                    Some(s) => push!(stack, s.clone()),
                    None => push!(stack, Val::Stack(Value::Null)),
                }?;
            }
            OpCode::Return => {
                let mut result = pop!(stack).unwrap();
                // Move to heap, before popping the stack or we risc to hit a dangling pointer  ಠ益ಠ
                match result.as_value() {
                    Value::ClassInstance(_) => {
                        result.into_heap();
                    }
                    Value::Closure(_) => {
                        result.into_heap();
                    }
                    _ => {}
                };

                stack.truncate(frame.idx);

                ctx.frames.pop();

                push!(ctx.stack, result)?;

                if ctx.frames.is_empty() {
                    break 'outer;
                }

                frame = ctx.frames.last().unwrap();
            }

            OpCode::Call0
            | OpCode::Call1
            | OpCode::Call2
            | OpCode::Call3
            | OpCode::Call4
            | OpCode::Call5
            | OpCode::Call6
            | OpCode::Call7
            | OpCode::Call8 => {
                let count = (instruction as u8) - (OpCode::Call0 as u8);
                let callee = peek!(ctx.stack, count as i32).expect("expect callee");
                call_value(ctx, callee.as_value(), count)?;
                frame = ctx.frames.last().unwrap();
            }
            OpCode::Invoke0
            | OpCode::Invoke1
            | OpCode::Invoke2
            | OpCode::Invoke3
            | OpCode::Invoke4
            | OpCode::Invoke5
            | OpCode::Invoke6
            | OpCode::Invoke7
            | OpCode::Invoke8 => {
                let count = (instruction as u8) - (OpCode::Invoke0 as u8);
                let method = frame.read_constant().unwrap().as_string().unwrap();
                invoke(ctx, method.as_str(), count).unwrap();
                frame = ctx.frames.last().unwrap();
            }
            OpCode::Super0
            | OpCode::Super1
            | OpCode::Super2
            | OpCode::Super3
            | OpCode::Super4
            | OpCode::Super5
            | OpCode::Super6
            | OpCode::Super7
            | OpCode::Super8 => {
                let count = (instruction as u8) - (OpCode::Super0 as u8);
                let method = frame.read_constant().unwrap().as_string().unwrap();
                let class = pop!(ctx.stack).expect("super class");
                invoke_from_class(ctx, class.as_class().expect("super class").as_ref(), method, count)?;
                frame = ctx.frames.last().unwrap();
            }
            OpCode::Closure => {
                let fu = frame.read_constant().unwrap().as_function().unwrap().clone();

                let mut values = Vec::new();

                for _i in 0..fu.up_value_count {
                    let local = if frame.read_byte() == 0 { false } else { true };
                    let index = frame.read_byte();
                    if local {
                        let value = stack.get(frame.idx + 1).unwrap();
                        values.push(capture_upvalue(value));
                    } else {
                        values.push(Val::Ref(
                            frame.closure.as_ref().upvalues()[index as usize].as_value() as *const Value
                        ));
                    };
                }

                let cl = Value::Closure(Rc::new(Closure::new(fu, values)));
                push!(stack, Val::Stack(cl))?;
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
                let ret = value_binary!(left.as_ref(), right.as_ref(), instruction)?;
                push!(stack, Val::Stack(ret))?;
            }
            OpCode::Array => {
                let o = frame.read_byte();
                if o == 0 {
                    push!(stack, Val::Stack(Value::Array(Array::default())))?;
                } else {
                    let mut v = Vec::new();
                    let idx = stack.len() - o as usize;
                    for i in idx..stack.len() {
                        let m = stack.get_mut(i).unwrap();
                        v.push(m.into_heap().clone());
                    }

                    stack.truncate(idx);
                    push!(stack, Val::Stack(Value::Array(Array::new(v))))?;
                }
            }
            // OpCode::Map => {
            //     let o = frame.read_byte();
            //     if o == 0 {
            //         push!(stack, Val::Stack(Value::Array(Array::default())))?;
            //     } else {
            //         let mut v = HashMap::new();
            //         let idx = stack.len() - o as usize;
            //         for i in stack.iter_mut().skip(idx) {
            //             v.push(i.into_heap().clone());
            //         }
            //         stack.truncate(idx);
            //         push!(stack, Val::Stack(Value::Array(Array::new(v))))?;
            //     }
            // }
            OpCode::JumpIfFalse => {
                let offset = frame.read_short();
                let v = peek!(stack, 0).unwrap();
                if !value_is_truthy!(v.as_ref()) {
                    let ip = frame.ip.get();
                    frame.ip.set(ip + offset as usize);
                }
            }
            OpCode::Jump => {
                let offset = frame.read_short();
                let ip = frame.ip.get();
                frame.ip.set(ip + offset as usize);
            }
            OpCode::Not => {
                //let current = pop!(stack).unwrap();
                let current = peek!(stack, 0).unwrap();
                let v = Value::Boolean(!value_is_truthy!(current.as_ref()));
                stack.set(stack.len() - 1, Val::Stack(v));
                //push!(stack, Val::Stack(v))?;
            }
            OpCode::Loop => {
                let offset = frame.read_short();
                let ip = frame.ip.get();
                frame.ip.set(ip - offset as usize);
            }

            OpCode::Class => {
                let name = frame.read_constant().unwrap().as_string().unwrap();
                let class = Class::new(name.to_owned());
                push!(stack, Val::Stack(Value::Class(Rc::new(class))))?;
            }
            OpCode::Method => {
                let name = frame.read_constant().unwrap().as_string().unwrap();
                let method = pop!(stack).unwrap();
                let class = pop!(stack).unwrap();
                class
                    .as_class()
                    .unwrap()
                    .add_method(name.to_owned(), method.into_value());
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
            OpCode::Nil => push!(stack, Val::Stack(Value::Null))?,
            OpCode::GetUpValue => {
                let idx = frame.read_byte();
                let value = &frame.closure.as_ref().upvalues()[idx as usize];
                push!(stack, Val::Ref(value.as_ref() as *const Value))?;
            }
            OpCode::CloseUpValue => {
                //
            }
            _ => unimplemented!("instruction {:?}", instruction),
        };
    }

    Ok(())
}

#[inline(always)]
pub(crate) fn call_value<S: Stack>(ctx: &Context<S>, callee: &Value, count: u8) -> RuntimeResult<()> {
    match callee {
        Value::Closure(cl) => {
            call(ctx, CloseurePtr::Ref(cl.as_ref() as *const Closure), count)?;
        }
        Value::Class(cl) => {
            let len = ctx.stack.len();
            let s = if count == 0 {
                len - 1
            } else {
                len - (count as usize) - 1
            };

            ctx.stack.set(
                s as usize,
                Val::Stack(Value::ClassInstance(ClassInstance::new(cl.clone()))),
            );
            if let Some(initializer) = cl.find_method("init") {
                let closure = initializer.as_closure().unwrap().clone();
                if closure.function.arity != count as i32 {
                    return Err("invalid numbers of parameters".into());
                }
                call(ctx, CloseurePtr::Stack(closure), count)?;
            } else if count != 0 {
                return Err("invalid numbers of parameters".into());
            }
        }
        Value::Native(native) => {
            let len = ctx.stack.len();
            let idx = len - (count as usize);

            let substack = ctx.stack.substack(idx);
            let subctx = Context::new(substack, ctx.globals.clone(), Frames::new());

            match (native.function)(&subctx) {
                Err(e) => {
                    return Err(e);
                }
                Ok(s) => {
                    ctx.stack.truncate(idx - 1);
                    push!(ctx.stack, Val::Stack(s));
                }
            }
        }

        _ => unimplemented!("call on {:?} not implemented", callee),
    }
    Ok(())
}

#[inline(always)]
fn call<S: Stack>(ctx: &Context<S>, closure: CloseurePtr, count: u8) -> RuntimeResult<()> {
    // TODO check aritity
    let a = closure.as_ref().function.arity;
    if a != count as i32 {
        return Err("Invalid parameter count".into());
    }

    let count = if ctx.stack.len() == 0 { 1 } else { a + 1 };
    let idx = ctx.stack.len() - (count as usize);
    let frame = CallFrame::new(closure, idx);
    ctx.frames.push(frame).expect("too many frames");

    Ok(())
}

#[inline(always)]
fn invoke_from_class<S: Stack>(ctx: &Context<S>, instance: &Instance, name: &str, count: u8) -> RuntimeResult<()> {
    let method = match instance.find_method(name) {
        Some(m) => m,
        None => return Err(format!("could not find method: '{}', on receiver: {:?}", name, instance).into()),
    };

    call(ctx, CloseurePtr::Stack(method.as_closure().unwrap().clone()), count)?;

    Ok(())
}

#[inline(always)]
fn invoke<S: Stack>(ctx: &Context<S>, name: &str, count: u8) -> RuntimeResult<()> {
    let receiver = peek_mut!(ctx.stack, count as i32).unwrap();

    let instance = match receiver.as_instance() {
        Some(s) => s,
        None => return Err(format!("receiver was {} expected instance for call: {}", receiver, name).into()),
    };

    let len = ctx.stack.len();
    let idx = len - (count as usize);
    let subctx = Context::new(ctx.stack.substack(idx), ctx.globals.clone(), Frames::new());

    if let Some(ret) = instance.call_method(name, &subctx) {
        match ret {
            Ok(m) => {
                ctx.stack.truncate(len - 1 - count as usize);
                push!(ctx.stack, Val::Stack(m));

                return Ok(());
            }
            Err(e) => return Err(e),
        }
    }

    invoke_from_class(ctx, instance, name, count)
}

fn capture_upvalue(local: &Val) -> Val {
    local.clone()
}