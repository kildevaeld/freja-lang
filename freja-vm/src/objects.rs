use super::chunk::Chunk;
use super::value::Val;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
#[derive(PartialEq)]
pub struct Function {
    #[cfg_attr(feature = "serde_support", serde(rename = "c"))]
    pub(crate) chunk: Chunk,
    #[cfg_attr(feature = "serde_support", serde(rename = "u"))]
    pub(crate) up_value_count: i32,
    #[cfg_attr(feature = "serde_support", serde(rename = "n"))]
    pub(crate) name: Option<String>,
    #[cfg_attr(feature = "serde_support", serde(rename = "a"))]
    pub(crate) arity: i32,
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.chunk)
    }
}

impl fmt::Debug for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Function")
    }
}

impl Function {
    pub fn new() -> Function {
        Function {
            chunk: Chunk::new(),
            up_value_count: 0,
            name: None,
            arity: 0,
        }
    }

    pub fn chunk(&self) -> &Chunk {
        &self.chunk
    }
}

#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
#[derive(PartialEq, Debug)]
pub struct Closure {
    pub(crate) function: Rc<Function>,
}

impl Closure {
    pub fn new(function: Rc<Function>) -> Closure {
        Closure { function }
    }

    pub fn chunk(&self) -> &Chunk {
        &self.function.chunk
    }
}

pub enum CloseurePtr {
    Stack(Rc<Closure>),
    Ref(*const Closure),
}

impl AsRef<Closure> for CloseurePtr {
    fn as_ref(&self) -> &Closure {
        match self {
            CloseurePtr::Ref(r) => unsafe { &**r },
            CloseurePtr::Stack(r) => r.as_ref(),
        }
    }
}

impl fmt::Debug for CloseurePtr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Closure as fmt::Debug>::fmt(self.as_ref(), f)
    }
}

pub struct Native {
    pub(crate) function: Box<Fn(&[Val])>,
}

impl PartialEq for Native {
    fn eq(&self, _other: &Native) -> bool {
        false
    }
}

#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
#[derive(Debug, PartialEq)]
pub struct Class {
    pub(crate) name: String,
    pub(crate) methods: RefCell<HashMap<String, Rc<Closure>>>,
}

impl Class {
    pub fn new(name: String) -> Class {
        Class {
            name,
            methods: RefCell::new(HashMap::new()),
        }
    }

    pub fn add_method(&self, name: String, method: Rc<Closure>) {
        self.methods.borrow_mut().insert(name, method);
    }

    pub fn inherit(&self, class: &Class) {
        let mut b = self.methods.borrow_mut();
        for m in class.methods.borrow().iter() {
            b.insert(m.0.clone(), m.1.clone());
        }
    }
}

#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
#[derive(Debug, PartialEq, Clone)]
pub struct ClassInstance {
    pub(crate) class: Rc<Class>,
}

impl ClassInstance {
    pub fn new(class: Rc<Class>) -> ClassInstance {
        ClassInstance { class }
    }
}

impl fmt::Debug for Native {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Native")
    }
}

#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
#[derive(PartialEq, Debug, Clone, Default)]
pub struct Array {
    inner: Vec<Val>,
}

impl Array {
    pub fn new(inner: Vec<Val>) -> Array {
        Array { inner }
    }
    pub fn is_empty(&self) -> bool {
        true
    }
}

impl fmt::Display for Array {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = self.inner.iter().map(|m| m.to_string()).collect::<Vec<_>>().join(", ");
        write!(f, "[{}]", s)
    }
}
