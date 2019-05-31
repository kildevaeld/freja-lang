use super::objects::Instance;
use super::objects::*;
use super::utils::Pointer;
use std::fmt;
use std::rc::Rc;

pub type ValuePtr = Rc<Value>;

#[derive(PartialEq, Debug, Clone)]
pub enum Value {
    Integer(i64),
    Double(f64),
    Boolean(bool),

    String(String),

    //Array(Rc<Array>),

    Map(Map),

    Function(Rc<Function>),

    Closure(Rc<Closure>),

    Native(Rc<Box<Native>>),

    Class(Rc<Box<Class>>),

    ClassInstance(Rc<Box<ClassInstance>>),

    Null,
}

impl Default for Value {
    fn default() -> Value {
        Value::Null
    }
}

impl Value {
    #[inline(always)]
    pub fn as_function(&self) -> Option<&Rc<Function>> {
        match self {
            Value::Function(f) => Some(f),
            _ => None,
        }
    }

    #[inline(always)]
    pub fn as_closure(&self) -> Option<&Rc<Closure>> {
        match self {
            Value::Closure(f) => Some(f),
            _ => None,
        }
    }

    #[inline(always)]
    pub fn as_native(&self) -> Option<&Rc<Box<Native>>> {
        match self {
            Value::Native(f) => Some(f),
            _ => None,
        }
    }

    #[inline(always)]
    pub fn as_string(&self) -> Option<&String> {
        match self {
            Value::String(f) => Some(f),
            _ => None,
        }
    }

    #[inline(always)]
    pub fn as_class(&self) -> Option<&Rc<Box<Class>>> {
        match self {
            Value::Class(f) => Some(f),
            _ => None,
        }
    }


    /*pub fn as_array(&self) -> Option<&Rc<Array>> {
        match self {
            Value::Array(f) => Some(f),
            _ => None,
        }
    }

    pub fn as_array_mut(&mut self) -> Option<&mut Rc<Array>> {
        match self {
            Value::Array(f) => Some(f),
            _ => None,
        }
    }*/

    #[inline(always)]
    pub fn as_instance(&self) -> Option<&Instance> {
        match self {
            Value::ClassInstance(i) => Some(i.as_instance()),
            Value::Class(c) => Some(c.as_instance()),
            // Value::Number(n) => Some(n),
            // Value::String(s) => Some(s),
            //Value::Array(a) => Some(a),
            _ => None,
        }
    }

    #[inline(always)]
    pub fn is_native(&self) -> bool {
        match self {
            Value::Native(_) => true,
            _ => false,
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Integer(i) => <i64 as fmt::Display>::fmt(i, f),
            Value::Double(d) => <f64 as fmt::Display>::fmt(d, f),
            Value::String(s) => write!(f, "{}", s),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Function(fu) => write!(f, "<fn {}>", fu.name.as_ref().map(|a| a.as_str()).unwrap_or("no-name")),
            Value::Null => write!(f, "nil"),
            Value::Map(a) => write!(f, "{}", a),
            Value::Class(c) => write!(f, "<class {}>", c.name()),
            Value::ClassInstance(i) => write!(f, "<instance {}>", i.class().name()),
            Value::Native(_) => write!(f, "<fn native>"),
            Value::Closure(cl) => write!(f, "<closure {}>", cl.name().unwrap_or("no-name")),
            //_ => write!(f, "Unknown"),
        }
    }
}

impl Value {
    #[inline(always)]
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::String(s) => !s.is_empty(),
            Value::Integer(i) => *i > 0,
            Value::Double(d) => *d > 0.0,
            Value::Boolean(b) => *b,
            Value::Class(_) => true,
            Value::ClassInstance(_) => true,
            //Value::Array(a) => !a.is_empty(),
            Value::Map(a) => !a.is_empty(),
            Value::Null => false,
            Value::Function(_) | Value::Closure(_) | Value::Native(_) => true,
        }
    }
}

pub type Val = Pointer<Value>;


#[macro_export]
macro_rules! value_add {
    ($lhs: expr, $rhs: expr) => {
        match $lhs {
            Value::Integer(n) => match $rhs {
                Value::Integer(nn) => Ok(Value::Integer(n + nn)),
                Value::Double(nn) => Ok(Value::Double(*n as f64 + *nn)),
                Value::String(s) => Ok(Value::String(format!("{}{}", n, s))),
                _ => Err("nan".into()),
            },
            Value::Double(n) => match $rhs {
                Value::Integer(nn) => Ok(Value::Double(*n + (*nn as f64))),
                Value::Double(nn) => Ok(Value::Double(*n as f64 + *nn)),
                Value::String(s) => Ok(Value::String(format!("{}{}", n, s))),
                _ => Err("nan".into()),
            },
            Value::String(s) => match $rhs {
                Value::String(ss) => Ok(Value::String([s.as_str(), ss.as_str()].concat())),
                Value::Integer(n) => Ok(Value::String(format!("{}{}", s, n))),
                Value::Double(n) => Ok(Value::String(format!("{}{}", s, n))),
                _ => Err("nan".into()),
            },
            _ => {
                println!("n {}", $lhs);
                Err(RuntimeError::Error(format!("could not add token {:?}", $lhs)))
            }
        }
    };
}
#[macro_export]
macro_rules! value_arithmetic {
    ($lhs: expr, $rhs: expr, $op: tt) => {
        match $lhs {
            Value::Integer(n) => match $rhs {
                Value::Integer(nn) => Ok(Value::Integer(n $op nn)),
                Value::Double(nn) => Ok(Value::Double(*n as f64 $op *nn)),
                _ => Err("nan".into()),
            },
            Value::Double(n) => match $rhs {
                Value::Integer(nn) => Ok(Value::Double(*n $op (*nn as f64))),
                Value::Double(nn) => Ok(Value::Double(*n as f64 $op *nn)),
                _ => Err("nan".into()),
            },
            _ => Err(RuntimeError::Error(format!("could not token {:?}", stringify!($op)))),
        }
    };
}
#[macro_export]
macro_rules! value_comparison {
    ($lhs: expr, $rhs: expr, $op: tt) => {
        match $lhs {
            Value::Integer(n) => match $rhs {
                Value::Integer(nn) => Ok(Value::Boolean(n $op nn)),
                Value::Double(nn) => {
                    let n = *n as f64;
                    Ok(Value::Boolean(&n $op nn))
                },
                _ => Err("nan".into()),
            },
            Value::Double(n) => match $rhs {
                Value::Integer(nn) => Ok(Value::Boolean(*n $op (*nn as f64))),
                Value::Double(nn) => Ok(Value::Boolean(n $op nn)),
                _ => Err("nan".into()),
            },
            _ => Err(RuntimeError::Error(format!("could not equal {:?} {} {:?}", $lhs,stringify!($op), $rhs))),
        }
    };
}

// #[inline(always)]
// pub fn value_binary(lhs: &Value, rhs: &Value, op: OpCode) -> RuntimeResult<Value> {
//     match op {
//         OpCode::Add => value_add!(lhs, rhs),
//         OpCode::Substract => value_arithmetic!(lhs, rhs, -),
//         OpCode::Multiply => value_arithmetic!(lhs, rhs, *),
//         OpCode::Divide => value_arithmetic!(lhs, rhs, /),
//         OpCode::Equal => value_comparison!(lhs, rhs, ==),
//         OpCode::Less => value_comparison!(lhs, rhs, <),
//         OpCode::Greater => value_comparison!(lhs, rhs, >),
//         _ => Err(RuntimeError::Error(format!("invalid binary token {:?}", op))),
//     }
// }

#[macro_export]
macro_rules! value_binary {
    ($lhs: expr, $rhs: expr, $op: expr) => {
        match $op {
            OpCode::Add => value_add!($lhs, $rhs),
            OpCode::Substract => value_arithmetic!($lhs, $rhs, -),
            OpCode::Multiply => value_arithmetic!($lhs, $rhs, *),
            OpCode::Divide => value_arithmetic!($lhs, $rhs, /),
            OpCode::Equal => value_comparison!($lhs, $rhs, ==),
            OpCode::Less => value_comparison!($lhs, $rhs, <),
            OpCode::Greater => value_comparison!($lhs, $rhs, >),
            _ => Err(RuntimeError::Error(format!("invalid binary token {:?}", $op))),
        }
    };
}
