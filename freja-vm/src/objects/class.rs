use super::super::error::RuntimeResult;
use super::super::value::{Val, Value};
use super::objects::Closure;
use super::types::Instance;
use std::cell::{RefCell, UnsafeCell};
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
#[derive(Debug, PartialEq)]
pub struct ClassInner {
    pub(crate) methods: HashMap<String, Rc<Value>>,
    pub(crate) super_class: Option<Rc<Class>>,
}

#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
#[derive(PartialEq)]
pub struct Class {
    pub(crate) name: String,
    inner: RefCell<ClassInner>,
}

impl Class {
    pub fn new(name: String) -> Class {
        Class {
            name,
            inner: RefCell::new(ClassInner {
                methods: HashMap::new(),
                super_class: None,
            }),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn add_method(&self, name: String, method: Rc<Value>) {
        self.inner.borrow_mut().methods.insert(name, method);
    }

    pub fn find_method(&self, name: &str) -> Option<Rc<Value>> {
        match self.inner.borrow().methods.iter().find(|m| m.0 == name) {
            Some(s) => Some(s.1.clone()),
            None => match self.inner.borrow().super_class {
                Some(ref s) => s.find_method(name),
                None => None,
            },
        }
    }

    pub fn inherit(&self, class: &Rc<Class>) {
        // let mut b = self.methods.borrow_mut();
        // for m in class.methods.borrow().iter() {
        //     b.insert(m.0.clone(), m.1.clone());
        // }
        self.inner.borrow_mut().super_class = Some(class.clone())
    }
}

impl fmt::Debug for Class {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Class")
    }
}

#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub struct ClassInstance {
    pub(crate) class: Rc<Class>,
    #[cfg_attr(feature = "serde_support", serde(skip))]
    fields: UnsafeCell<HashMap<String, Val>>,
}

impl ClassInstance {
    pub fn new(class: Rc<Class>) -> ClassInstance {
        ClassInstance {
            class,
            fields: UnsafeCell::new(HashMap::new()),
        }
    }

    fn fields(&self) -> &mut HashMap<String, Val> {
        unsafe { &mut *self.fields.get() }
    }
}

impl PartialEq for ClassInstance {
    fn eq(&self, other: &ClassInstance) -> bool {
        self.class == other.class && unsafe { (&*self.fields.get()) == (&*other.fields.get()) }
    }
}

impl Clone for ClassInstance {
    fn clone(&self) -> ClassInstance {
        ClassInstance {
            class: self.class.clone(),
            fields: UnsafeCell::new(self.fields().clone()),
        }
    }
}

impl Instance for ClassInstance {
    fn set_field(&self, name: &str, value: Val) -> RuntimeResult<()> {
        unsafe {
            self.fields().insert(name.to_string(), value);
        }

        Ok(())
    }

    fn get_field(&self, name: &str) -> Option<&Val> {
        self.fields().get(name)
    }

    fn find_method(&self, name: &str) -> Option<Rc<Value>> {
        self.class.find_method(name)
    }
}
