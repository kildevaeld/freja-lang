use super::objects::*;
use std::fmt;
use std::rc::Rc;



#[derive(PartialEq, Debug, Clone)]
pub enum Constant {
    Integer(i64),
    Double(f64),
    Boolean(bool),
    String(String),
    Function(Rc<Function>),
    Nil,
}

impl Default for Constant {
    fn default() -> Constant {
        Constant::Nil
    }
}

impl Constant {
    #[inline(always)]
    pub fn as_function(&self) -> Option<&Rc<Function>> {
        match self {
            Constant::Function(f) => Some(f),
            _ => None,
        }
    }


    #[inline(always)]
    pub fn as_string(&self) -> Option<&String> {
        match self {
            Constant::String(f) => Some(f),
            _ => None,
        }
    }

}


impl fmt::Display for Constant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Constant::Integer(i) => <i64 as fmt::Display>::fmt(i, f),
            Constant::Double(d) => <f64 as fmt::Display>::fmt(d, f),
            Constant::String(s) => write!(f, "{}", s),
            Constant::Boolean(b) => write!(f, "{}", b),
            Constant::Function(fu) => write!(f, "<fn {}>", fu.name.as_ref().map(|a| a.as_str()).unwrap_or("no-name")),
            Constant::Nil => write!(f, "nil"),
        }
    }
}

// impl Constant {
//     #[inline(always)]
//     pub fn is_truthy(&self) -> bool {
//         match self {
//             Constant::String(s) => !s.is_empty(),
//             Constant::Integer(i) => *i > 0,
//             Constant::Double(d) => *d > 0.0,
//             Constant::Boolean(b) => *b,
          
//             Constant::Null => false,
//             /*Constant::Function(_) |*/ Constant::Closure(_) | Constant::Native(_) => true,
//         }
//     }
// }

// pub type Val = Pointer<Value>;


// #[macro_export]
// macro_rules! value_add {
//     ($lhs: expr, $rhs: expr) => {
//         match $lhs {
//             Constant::Integer(n) => match $rhs {
//                 Constant::Integer(nn) => Ok(Constant::Integer(n + nn)),
//                 Constant::Double(nn) => Ok(Constant::Double(*n as f64 + *nn)),
//                 Constant::String(s) => Ok(Constant::String(format!("{}{}", n, s))),
//                 _ => Err("nan".into()),
//             },
//             Constant::Double(n) => match $rhs {
//                 Constant::Integer(nn) => Ok(Constant::Double(*n + (*nn as f64))),
//                 Constant::Double(nn) => Ok(Constant::Double(*n as f64 + *nn)),
//                 Constant::String(s) => Ok(Constant::String(format!("{}{}", n, s))),
//                 _ => Err("nan".into()),
//             },
//             Constant::String(s) => match $rhs {
//                 Constant::String(ss) => Ok(Constant::String([s.as_str(), ss.as_str()].concat())),
//                 Constant::Integer(n) => Ok(Constant::String(format!("{}{}", s, n))),
//                 Constant::Double(n) => Ok(Constant::String(format!("{}{}", s, n))),
//                 _ => Err("nan".into()),
//             },
//             _ => {
//                 println!("n {}", $lhs);
//                 Err(RuntimeError::Error(format!("could not add token {:?}", $lhs)))
//             }
//         }
//     };
// }
// #[macro_export]
// macro_rules! value_arithmetic {
//     ($lhs: expr, $rhs: expr, $op: tt) => {
//         match $lhs {
//             Constant::Integer(n) => match $rhs {
//                 Constant::Integer(nn) => Ok(Constant::Integer(n $op nn)),
//                 Constant::Double(nn) => Ok(Constant::Double(*n as f64 $op *nn)),
//                 _ => Err("nan".into()),
//             },
//             Constant::Double(n) => match $rhs {
//                 Constant::Integer(nn) => Ok(Constant::Double(*n $op (*nn as f64))),
//                 Constant::Double(nn) => Ok(Constant::Double(*n as f64 $op *nn)),
//                 _ => Err("nan".into()),
//             },
//             _ => Err(RuntimeError::Error(format!("could not token {:?}", stringify!($op)))),
//         }
//     };
// }
// #[macro_export]
// macro_rules! value_comparison {
//     ($lhs: expr, $rhs: expr, $op: tt) => {
//         match $lhs {
//             Constant::Integer(n) => match $rhs {
//                 Constant::Integer(nn) => Ok(Constant::Boolean(n $op nn)),
//                 Constant::Double(nn) => {
//                     let n = *n as f64;
//                     Ok(Constant::Boolean(&n $op nn))
//                 },
//                 _ => Err("nan".into()),
//             },
//             Constant::Double(n) => match $rhs {
//                 Constant::Integer(nn) => Ok(Constant::Boolean(*n $op (*nn as f64))),
//                 Constant::Double(nn) => Ok(Constant::Boolean(n $op nn)),
//                 _ => Err("nan".into()),
//             },
//             _ => Err(RuntimeError::Error(format!("could not equal {:?} {} {:?}", $lhs,stringify!($op), $rhs))),
//         }
//     };
// }

// // #[inline(always)]
// // pub fn value_binary(lhs: &Value, rhs: &Value, op: OpCode) -> RuntimeResult<Value> {
// //     match op {
// //         OpCode::Add => value_add!(lhs, rhs),
// //         OpCode::Substract => value_arithmetic!(lhs, rhs, -),
// //         OpCode::Multiply => value_arithmetic!(lhs, rhs, *),
// //         OpCode::Divide => value_arithmetic!(lhs, rhs, /),
// //         OpCode::Equal => value_comparison!(lhs, rhs, ==),
// //         OpCode::Less => value_comparison!(lhs, rhs, <),
// //         OpCode::Greater => value_comparison!(lhs, rhs, >),
// //         _ => Err(RuntimeError::Error(format!("invalid binary token {:?}", op))),
// //     }
// // }

// #[macro_export]
// macro_rules! value_binary {
//     ($lhs: expr, $rhs: expr, $op: expr) => {
//         match $op {
//             OpCode::Add => value_add!($lhs, $rhs),
//             OpCode::Substract => value_arithmetic!($lhs, $rhs, -),
//             OpCode::Multiply => value_arithmetic!($lhs, $rhs, *),
//             OpCode::Divide => value_arithmetic!($lhs, $rhs, /),
//             OpCode::Equal => value_comparison!($lhs, $rhs, ==),
//             OpCode::Less => value_comparison!($lhs, $rhs, <),
//             OpCode::Greater => value_comparison!($lhs, $rhs, >),
//             _ => Err(RuntimeError::Error(format!("invalid binary token {:?}", $op))),
//         }
//     };
// }
