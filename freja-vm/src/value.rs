use super::objects::*;
use freja_parser::ast::Number;
use std::fmt;
use std::rc::Rc;

pub type ValuePtr = Rc<Value>;

#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde_support", serde(tag = "t", content = "c"))]
#[derive(PartialEq, Debug, Clone)]
pub enum Value {
    #[cfg_attr(feature = "serde_support", serde(rename = "N"))]
    Number(Number),
    #[cfg_attr(feature = "serde_support", serde(rename = "S"))]
    String(String),
    #[cfg_attr(feature = "serde_support", serde(rename = "B"))]
    Boolean(bool),
    #[cfg_attr(feature = "serde_support", serde(rename = "A"))]
    Array(Array),
    #[cfg_attr(feature = "serde_support", serde(rename = "F"))]
    Function(Rc<Function>),
    #[cfg_attr(feature = "serde_support", serde(rename = "C"))]
    Closure(Rc<Closure>),
    #[cfg_attr(feature = "serde_support", serde(skip))]
    Native(Rc<Native>),
    #[cfg_attr(feature = "serde_support", serde(rename = "CC"))]
    Class(Rc<Class>),
    #[cfg_attr(feature = "serde_support", serde(rename = "I"))]
    Instance(ClassInstance),
    #[cfg_attr(feature = "serde_support", serde(rename = "nil"))]
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

    pub fn as_closure(&self) -> Option<&Rc<Closure>> {
        match self {
            Value::Closure(f) => Some(f),
            _ => None,
        }
    }

    pub fn as_string(&self) -> Option<&String> {
        match self {
            Value::String(f) => Some(f),
            _ => None,
        }
    }

    pub fn as_class(&self) -> Option<&Rc<Class>> {
        match self {
            Value::Class(f) => Some(f),
            _ => None,
        }
    }

    pub fn as_instance(&self) -> Option<&ClassInstance> {
        match self {
            Value::Instance(f) => Some(f),
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
            Value::Array(a) => write!(f, "{}", a),
            Value::Class(c) => write!(f, "<class {}>", c.name),
            Value::Instance(i) => write!(f, "<instance {}>", i.class.name),
            Value::Native(_) => write!(f, "<fn native>"),
            Value::Closure(cl) => write!(
                f,
                "<fn {}>",
                cl.function.name.as_ref().map(|a| a.as_str()).unwrap_or("no-name")
            ),
            //_ => write!(f, "Unknown"),
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
            Value::Class(_) => true,
            Value::Instance(_) => true,
            Value::Array(a) => !a.is_empty(),
            Value::Null => false,
            Value::Function(_) | Value::Closure(_) | Value::Native(_) => true
            //Value::Instance(_) => true,
        }
    }
}
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
#[derive(PartialEq, Debug, Clone)]
pub enum Val {
    Heap(ValuePtr),
    Stack(Value),
    #[cfg_attr(feature = "serde_support", serde(skip))]
    Ref(*const Value),
}

impl Val {
    pub fn into_value(self) -> ValuePtr {
        match self {
            Val::Heap(h) => h,
            Val::Stack(s) => Rc::new(s),
            Val::Ref(r) => unsafe { Rc::new((&*r).clone()) },
        }
    }

    pub fn into_heap(&mut self) -> &mut Self {
        let this = std::mem::replace(self, Val::Stack(Value::Null));
        match this {
            Val::Heap(h) => {
                *self = Val::Heap(h);
            }
            Val::Stack(s) => {
                *self = Val::Heap(Rc::new(s));
            }
            Val::Ref(r) => *self = unsafe { Val::Heap(Rc::new((&*r).clone())) },
        }
        self
    }

    pub fn as_value(&self) -> &Value {
        match &self {
            Val::Heap(h) => h,
            Val::Stack(s) => s,
            Val::Ref(r) => unsafe { &**r },
        }
    }

    pub fn is_truthy(&self) -> bool {
        self.as_value().is_truthy()
    }
}

impl fmt::Display for Val {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // match self {
        //     Val::Heap(h) => <Value as fmt::Display>::fmt(h, f),
        //     Val::Stack(h) => <Value as fmt::Display>::fmt(h, f),
        //     Val::Ref(r) => <Value as fmt::Display>::fmt()
        // }
        <Value as fmt::Display>::fmt(self.as_value(), f)
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

macro_rules! value_add {
    ($lhs: expr, $rhs: expr) => {
        match $lhs {
            Value::Number(n) => match $rhs {
                Value::Number(nn) => Ok(Value::Number(n + nn)),
                Value::String(s) => Ok(Value::String(format!("{}{}", n, s))),
                _ => Err("nan".into()),
            },
            Value::String(s) => match $rhs {
                Value::String(ss) => Ok(Value::String([s.as_str(), ss.as_str()].concat())),
                Value::Number(n) => Ok(Value::String(format!("{},{}", s, n))),
                _ => Err("nan".into()),
            },
            _ => {
                println!("n {}", $lhs);
                Err("could not add".into())
            }
        }
    };
}

macro_rules! value_arithmetic {
    ($lhs: expr, $rhs: expr, $op: tt) => {
        match $lhs {
            Value::Number(n) => match $rhs {
                Value::Number(nn) => Ok(Value::Number(n $op nn)),
                _ => Err("nan".into()),
            },
            _ => Err("could not sub".into()),
        }
    };
}

macro_rules! value_comparison {
    ($lhs: expr, $rhs: expr, $op: tt) => {
        match $lhs {
            Value::Number(n) => match $rhs {
                Value::Number(nn) => Ok(Value::Boolean(n $op nn)),
                _ => Err("nan".into()),
            },
            _ => Err("could not sub".into()),
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

#[macro_export]
macro_rules! value_is_truthy {
    ($value: expr) => {
        match $value {
                                                    Value::String(s) => !s.is_empty(),
                                                    Value::Number(Number::Double(d)) => *d > 0.0,
                                                    Value::Number(Number::Integer(d)) => *d > 0,
                                                    Value::Boolean(b) => *b,
                                                    Value::Class(_) => true,
                                                    Value::Instance(_) => true,
                                                    Value::Array(a) => !a.is_empty(),
                                                    Value::Null => false,
                                                    Value::Function(_) | Value::Closure(_) | Value::Native(_) => true
                                                    //Value::Instance(_) => true,
                                                }
    };
}

#[macro_export]
macro_rules! is_falsey {
    ($value: expr) => {
        !value_is_truthy!($value)
    };
}
