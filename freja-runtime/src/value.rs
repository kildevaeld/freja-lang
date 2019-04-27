use super::callable::FrejaCallable;
use super::error::*;
use freja_ast::{Number, Token, TokenType};
use std::fmt;

#[derive(Debug)]
pub enum Value {
    String(String),
    Number(Number),
    Boolean(bool),
    Function(Box<FrejaCallable + 'static>),
    Null,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::String(s) => write!(f, "{}", s),
            Value::Number(n) => <Number as fmt::Display>::fmt(n, f),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Null => write!(f, "null"),
            Value::Function(_) => write!(f, "<native fn>"),
        }
    }
}

#[inline]
fn value_add(lhs: &Value, rhs: &Value) -> RuntimeResult<Value> {
    match lhs {
        Value::Number(n) => match rhs {
            Value::Number(nn) => Ok(Value::Number(n + nn)),
            _ => Err("nan".into()),
        },
        _ => Err("nan 2".into()),
    }
}

#[inline]
fn value_sub(lhs: &Value, rhs: &Value) -> RuntimeResult<Value> {
    match lhs {
        Value::Number(n) => match rhs {
            Value::Number(nn) => Ok(Value::Number(n - nn)),
            _ => Err("nan".into()),
        },
        _ => Err("nan 2".into()),
    }
}

#[inline]
fn value_mul(lhs: &Value, rhs: &Value) -> RuntimeResult<Value> {
    match lhs {
        Value::Number(n) => match rhs {
            Value::Number(nn) => Ok(Value::Number(n * nn)),
            _ => Err("nan".into()),
        },
        _ => Err("nan 2".into()),
    }
}

#[inline]
fn value_div(lhs: &Value, rhs: &Value) -> RuntimeResult<Value> {
    match lhs {
        Value::Number(n) => match rhs {
            Value::Number(nn) => Ok(Value::Number(n / nn)),
            _ => Err("nan".into()),
        },
        _ => Err("nan 2".into()),
    }
}

pub fn value_binary(lhs: &Value, rhs: &Value, op: &Token) -> RuntimeResult<Value> {
    match op.value {
        "+" => value_add(lhs, rhs),
        "-" => value_sub(lhs, rhs),
        "*" => value_mul(lhs, rhs),
        "/" => value_div(lhs, rhs),
        _ => Err(RuntimeError::Unknown(format!(
            "invalid binary token {:?}",
            op
        ))),
    }
}
