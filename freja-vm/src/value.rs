use std::fmt;
use std::rc::Rc;

pub type ValuePtr = Rc<Value>;

pub enum Value {
    Double(f64),
    Integer(i64),
    String(String),
    Boolean(bool),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Double(d) => write!(f, "{}", d),
            Value::Integer(i) => write!(f, "{}", i),
            Value::String(s) => write!(f, "{}", s),
            Value::Boolean(b) => write!(f, "{}", b),
            _ => write!(f, "Unknown"),
        }
    }
}
