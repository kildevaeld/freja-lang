use super::chunk::OpCode;
use super::context::Context;
use super::error::{RuntimeError, RuntimeResult};
use super::frames::*;
use super::objects::*;
use super::stack::Stack;
use super::utils::Pointer;
use super::value::*;
use std::collections::HashMap;
use std::rc::Rc;


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

    'outer: while frame.ip.get() < frame.closure.chunk().len() {
        let instruction = frame.read_byte();
        let instruction = OpCode::from(instruction);

        match instruction {
            OpCode::Constant => {
                let val = frame.read_constant().expect("constant") as *const Value;
                push!(stack, Pointer::Ref(val))?;
            }
            OpCode::Pop => {
                pop!(stack);
            }
            OpCode::DefineGlobal => {
                let name = frame.read_constant().unwrap();
                let value = pop!(stack).unwrap();
                ctx.globals
                    .insert(name.as_string().unwrap().to_string(), value.into_inner());
            }
            OpCode::GetGlobal => {
                let name = frame.read_constant().unwrap().as_string().unwrap();

                let m = match ctx.globals.get(name) {
                    Some(m) => m as *const Value,
                    None => panic!("undefined variable: {}", name),
                };
                push!(stack, Pointer::Ref(m))?;
            }
            OpCode::GetLocal => {
                let b = frame.read_byte();
                let idx = frame.idx + b as usize;
                let val = stack.get_mut(idx).expect(format!("get local at idx {}", idx).as_str());

                push!(stack, val.as_ptr())?;
            }
            OpCode::SetLocal => {
                let b = frame.read_byte();
                let val = peek_mut!(stack, 0).unwrap();
                let idx = frame.idx + b as usize;
                stack.set(idx, val.clone());
            }
            OpCode::SetGlobal => {
                let name = frame.read_constant().unwrap().as_string().unwrap();
                if !ctx.globals.contains_key(name.as_str()) {
                    return Err(RuntimeError::InvalidIndex);
                }
                let val = peek!(stack, 0).unwrap();
                ctx.globals.insert(name.clone(), val.clone().into_inner());
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

                instance.set_field(name, value.clone().into_inner())?;
            }
            OpCode::GetProperty => {
                let name = frame.read_constant().unwrap().as_string().unwrap();
                let receiver = pop!(stack).expect("get property receiver");

                let instance = match receiver.as_instance() {
                    Some(i) => i,
                    None => return Err("canget property on an instance".into()),
                };
                match instance.get_field(name) {
                    Some(s) => push!(stack, Pointer::Stack(s.clone())),
                    None => push!(stack, Pointer::Stack(Value::Null)),
                }?;
            }
            OpCode::Return => {
                let mut result = stack.pop().unwrap();
                if result.is_ref() {
                    result = result.clone()
                }

                stack.truncate(frame.idx);

                ctx.frames.pop();

                stack.push(result)?;

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
                let callee = ctx.peek(count as usize).expect("expect callee");
                call_value(ctx, callee, count)?;
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
                invoke_from_class(ctx, class.as_instance().expect("super class"), method, count)?;
                frame = ctx.frames.last().unwrap();
            }
            OpCode::Closure => {
                let fu = frame.read_constant().unwrap().as_function().unwrap();

                let mut values = Vec::new();

                for _i in 0..fu.up_value_count {
                    let local = if frame.read_byte() == 0 { false } else { true };
                    let index = frame.read_byte();
                    if local {
                        let value = stack.get(frame.idx + 1).unwrap();
                        values.push(capture_upvalue(value));
                    } else {
                        // values.push(Val::Ref(
                        //     frame.closure.as_ref().upvalues()[index as usize].as_value() as *const Value
                        // ));
                        values.push(frame.closure.as_ref().upvalues()[index as usize].clone())
                    };
                }

                let cl = Value::Closure(Rc::new(Closure::new(Pointer::Stack(fu.clone()), values)));
                push!(stack, Pointer::Stack(cl))?;
            }
            OpCode::Add => {
                let right = stack.pop().unwrap();
                let left = stack.pop().unwrap();
                let ret = value_add!(left.as_ref(), right.as_ref())?;
                ctx.push(ret)?;
                //push!(stack, Pointer::Stack(ret))?;
            }
            OpCode::Substract => {
                let right = stack.pop().unwrap();
                let left = stack.pop().unwrap();
                let ret = value_arithmetic!(left.as_ref(), right.as_ref(), -)?;
                push!(stack, Pointer::Stack(ret))?;
            }
            OpCode::Greater => {
                let right = stack.pop().unwrap();
                let left = stack.pop().unwrap();
                let ret = value_comparison!(left.as_ref(), right.as_ref(), >)?;
                push!(stack, Pointer::Stack(ret))?;
            }
            OpCode::Divide | OpCode::Multiply | OpCode::Equal | OpCode::Less => {
                let right = stack.pop().unwrap();
                let left = stack.pop().unwrap();
                let ret = value_binary!(left.as_ref(), right.as_ref(), instruction)?;
                push!(stack, Pointer::Stack(ret))?;
            }
            OpCode::Array => {
                let o = frame.read_byte();
                if o == 0 {
                    ctx.stack
                        .push(Pointer::Stack(Value::Class(Rc::new(Box::new(Array2::new())))))?;
                    let calle = ctx.peek(0).unwrap();
                    call_value(ctx, calle, 0)?;
                } else {
                    let idx = stack.len() - o as usize;
                    let v = Vec::from(&stack.as_ref()[idx..]);
                    stack.truncate(idx);
                    let array_class = ctx
                        .globals
                        .get("Array")
                        .map(|a| Pointer::Ref(a as *const Value))
                        .unwrap();

                    ctx.stack.push(array_class)?;
                    for b in v.into_iter() {
                        ctx.stack.push(b)?;
                    }

                    let calle = ctx.peek(o as usize).unwrap();
                    call_value(ctx, calle, o)?;

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
                if !v.is_truthy() {
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
                let current = peek!(stack, 0).unwrap();
                let v = Value::Boolean(!current.is_truthy());
                stack.set(stack.len() - 1, Pointer::Stack(v));
            }
            OpCode::Loop => {
                let offset = frame.read_short();
                let ip = frame.ip.get();
                frame.ip.set(ip - offset as usize);
            }

            OpCode::Class => {
                let name = frame.read_constant().unwrap().as_string().unwrap();
                let class = SourceClass::new(name.to_owned());
                push!(stack, Pointer::Stack(Value::Class(Rc::new(Box::new(class)))))?;
            }
            OpCode::Method => {
                let name = frame.read_constant().unwrap().as_string().unwrap();
                let method = pop!(stack).unwrap();
                let class = pop!(stack).unwrap();
                class
                    .as_class()
                    .expect("instance")
                    .as_any()
                    .downcast_ref::<SourceClass>()
                    .expect("source class")
                    .add_method(name.to_owned(), method.into_inner());
            }
            OpCode::Inherit => {
                let super_c = peek!(stack, 1).unwrap();
                let super_c = match super_c.as_class() {
                    Some(c) => c,
                    None => panic!("super"),
                };
                let subclass = peek!(stack, 0).unwrap().as_instance().unwrap();
                subclass
                    .as_any()
                    .downcast_ref::<SourceClass>()
                    .expect("expected superclass")
                    .inherit(super_c);
                pop!(stack);
            }
            OpCode::Nil => push!(stack, Pointer::Stack(Value::Null))?,
            OpCode::GetUpValue => {
                let idx = frame.read_byte();
                let value = &frame.closure.as_ref().upvalues()[idx as usize];
                push!(stack, value.clone())?;
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
pub(crate) fn call_value<S: Stack>(ctx: &Context<S>, callee: &Val, count: u8) -> RuntimeResult<()> {
    match callee.as_ref() {
        Value::Closure(cl) => {
            call(ctx, Pointer::Ref(cl as *const Rc<Closure>), count)?;
        }
        Value::Class(cl) => {
            let len = ctx.stack.len();
            let s = if count == 0 {
                len - 1
            } else {
                len - (count as usize) - 1
            };

            if let Some(_) = cl.as_instance().as_any().downcast_ref::<SourceClass>() {
                ctx.stack.set(
                    s as usize,
                    Pointer::Stack(Value::ClassInstance(Rc::new(Box::new(SourceClassInstance::new(
                        cl.clone(),
                    ))))),
                );

                if let Some(initializer) = cl.find_method("init") {
                    let closure = initializer.as_closure().unwrap().clone();
                    if closure.arity() != count as i32 {
                        return Err("invalid numbers of parameters".into());
                    }
                    call(ctx, Pointer::Stack(closure), count)?;
                } else if count != 0 {
                    return Err("invalid numbers of parameters".into());
                }
            } else {
                let idx = ctx.stack.len() - count as usize;
                cl.construct(&ctx.child(ctx.stack.len() - 1 - (count as usize)))?;
                ctx.stack.truncate(idx);

            }
        }
        Value::Native(native) => {
            let len = ctx.stack.len();
            let idx = len - (count as usize);

            let substack = ctx.stack.substack(idx);
            let subctx = Context::new(substack, ctx.globals.clone(), Frames::new());

            match native.call(&subctx) {
                Err(e) => {
                    return Err(e);
                }
                Ok(s) => {
                    ctx.stack.truncate(idx - 1);
                    push!(ctx.stack, Pointer::Stack(s))?;
                }
            }
        }

        _ => unimplemented!("call on {:?} not implemented", callee),
    }
    Ok(())
}

#[inline(always)]
pub fn call<S: Stack>(ctx: &Context<S>, closure: Pointer<Rc<Closure>>, count: u8) -> RuntimeResult<()> {
    // TODO check aritity
    let a = closure.arity();
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

    if method.is_native() {
        let native = method.as_native().unwrap();
        let idx = ctx.stack.len() - 1 - count as usize;
        let subctx = ctx.child(idx);
        let ret = native.call(&subctx)?; // (native.function)(&subctx)?;
        ctx.stack.truncate(idx);
        push!(ctx.stack, Pointer::Stack(ret))?;
    } else {
        call(ctx, Pointer::Stack(method.as_closure().unwrap().clone()), count)?;
    }

    Ok(())
}

#[inline(always)]
fn invoke<S: Stack>(ctx: &Context<S>, name: &str, count: u8) -> RuntimeResult<()> {
    let receiver = peek_mut!(ctx.stack, count as i32).unwrap();

    let instance = match receiver.as_instance() {
        Some(s) => s,
        None => return Err(format!("receiver was {} expected instance for call: {}", receiver, name).into()),
    };


    invoke_from_class(ctx, instance, name, count)
}

fn capture_upvalue(local: &Val) -> Val {
    local.clone()
}
