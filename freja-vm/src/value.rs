use super::chunk::{Chunk, OpCode};
use std::fmt;
use std::rc::Rc;
use super::error::{RuntimeResult, RuntimeError};
use super::objects::*;
use freja_parser::ast::Number;

pub type ValuePtr = Rc<Value>;



#[derive(PartialEq, Debug, Clone)]
pub enum Value {
    Number(Number),
    String(String),
    Boolean(bool),
    Function(Rc<Function>),
    Closure(Rc<Closure>),
    Native(Rc<Native>),
    Null,
}

impl Default for Value {
    fn default() -> Value {
        Value::Null
    }
}

impl Value {
    pub fn as_function(&self) -> Option<&Rc<Function>> {
        match self {
            Value::Function(f) => Some(f),
            _ => None,
        }
    }

    pub fn as_string(&self) -> Option<&String> {
        match self {
            Value::String(f) => Some(f),
            _ => None,
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Number(n) => <Number as fmt::Display>::fmt(n, f),
            Value::String(s) => write!(f, "{}", s),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Function(fu) => write!(f, "<fn {}>", fu.name.as_ref().map(|a| a.as_str()).unwrap_or("no-name")),
            Value::Null => write!(f, "nil"),
            Value::Closure(cl) => write!(f, "<fn {}>", cl.function.name.as_ref().map(|a| a.as_str()).unwrap_or("no-name")),
            _ => write!(f, "Unknown"),
        }
    }
}

impl Value {
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::String(s) => !s.is_empty(),
            Value::Number(Number::Double(d)) => *d > 0.0,
            Value::Number(Number::Integer(d)) => *d > 0,
            Value::Boolean(b) => *b,
            
            //Value::Array(_) => true,
            Value::Null => false,
            Value::Function(_) | Value::Closure(_) | Value::Native(_) => true
            //Value::Instance(_) => true,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Val {
    Heap(ValuePtr),
    Stack(Value),
}

impl Val {
    pub fn into_heap(self) -> ValuePtr {
        match self {
            Val::Heap(h) => h,
            Val::Stack(s) => Rc::new(s),
        }
    }

    pub fn into_heap2(&mut self) {
        let this = std::mem::replace(self, Val::Stack(Value::Null));
        match this {
            Val::Heap(h) => {
                *self = Val::Heap(h);
            },
            Val::Stack(s) => {
                *self = Val::Heap(Rc::new(s));
            },
        }
    }

   

    pub fn as_value(&self) -> &Value {
        match &self {
            Val::Heap(h) => h,
            Val::Stack(s) => s,
        }
    }

    pub fn is_truthy(&self) -> bool {
        self.as_value().is_truthy()
    }
}

impl fmt::Display for Val {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Val::Heap(h) => <Value as fmt::Display>::fmt(h, f),
            Val::Stack(h) => <Value as fmt::Display>::fmt(h, f),
        }
    }
}

impl std::ops::Deref for Val {
    type Target = Value;
    fn deref(&self) -> &Self::Target {
        self.as_value()
    }
}

impl AsRef<Value> for Val {
    fn as_ref(&self) -> &Value {
        self.as_value()
    }
}

#[inline(always)]
fn value_add(lhs: &Value, rhs: &Value) -> RuntimeResult<Value> {
    match lhs {
        Value::Number(n) => match rhs {
            Value::Number(nn) => Ok(Value::Number(n + nn)),
            _ => Err("nan".into()),
        },
        Value::String(s) => match rhs {
            Value::String(ss) => Ok(Value::String([s.as_str(), ss.as_str()].concat())),
            _ => Err("nan".into()),
        },
        _ => {
            println!("n {}", lhs);
            Err("could not add".into())
        },
    }
}

#[inline(always)]
fn value_sub(lhs: &Value, rhs: &Value) -> RuntimeResult<Value> {
    match lhs {
        Value::Number(n) => match rhs {
            Value::Number(nn) => Ok(Value::Number(n - nn)),
            _ => Err("nan".into()),
        },
        _ => Err("could not sub".into()),
    }
}

#[inline(always)]
fn value_mul(lhs: &Value, rhs: &Value) -> RuntimeResult<Value> {
    match lhs {
        Value::Number(n) => match rhs {
            Value::Number(nn) => Ok(Value::Number(n * nn)),
            _ => Err("nan".into()),
        },
        _ => Err("could mul".into()),
    }
}

#[inline(always)]
fn value_div(lhs: &Value, rhs: &Value) -> RuntimeResult<Value> {
    match lhs {
        Value::Number(n) => match rhs {
            Value::Number(nn) => Ok(Value::Number(n / nn)),
            _ => Err("nan".into()),
        },
        _ => Err("could div".into()),
    }
}



#[inline(always)]
fn value_lt(lhs: &Value, rhs: &Value) -> RuntimeResult<Value> {
    match lhs {
        Value::Number(n) => match rhs {
            Value::Number(nn) => Ok(Value::Boolean(n < nn)),
            _ => Err("nan".into()),
        },
        _ => Err("could not less".into()),
    }
}

#[inline(always)]
fn value_gt(lhs: &Value, rhs: &Value) -> RuntimeResult<Value> {
    match lhs {
        Value::Number(n) => match rhs {
            Value::Number(nn) => Ok(Value::Boolean(n > nn)),
            _ => Err("nan".into()),
        },
        _ => Err("could not greater".into()),
    }
}

#[inline(always)]
fn value_eq(lhs: &Value, rhs: &Value) -> RuntimeResult<Value> {
    match lhs {
        Value::Number(n) => match rhs {
            Value::Number(nn) => Ok(Value::Boolean(n == nn)),
            _ => Err("nan".into()),
        },
        _ => Err("could not equal".into()),
    }
}

pub fn value_binary(lhs: &Value, rhs: &Value, op: OpCode) -> RuntimeResult<Value> {
    match op {
        OpCode::Add=> value_add(lhs, rhs),
        OpCode::Substract => value_sub(lhs, rhs),
        OpCode::Multiply => value_mul(lhs, rhs),
        OpCode::Divide => value_div(lhs, rhs),
        OpCode::Equal => value_eq(lhs, rhs),
        OpCode::Less => value_lt(lhs, rhs),
        OpCode::Greater => value_gt(lhs, rhs),
        _ => Err(RuntimeError::Error(format!(
            "invalid binary token {:?}",
            op
        ))),
    }
}