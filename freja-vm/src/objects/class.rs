use super::super::error::RuntimeResult;
use super::super::value::{Val, Value};
use super::types::Instance;
use std::cell::{RefCell, UnsafeCell};
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub struct ClassInner {
    pub(crate) methods: HashMap<String, Value>,
    pub(crate) super_class: Option<Rc<Class>>,
}

pub struct Class {
    pub(crate) name: String,
    inner: UnsafeCell<ClassInner>,
}

impl Class {
    pub fn new(name: String) -> Class {
        Class {
            name,
            inner: UnsafeCell::new(ClassInner {
                methods: HashMap::new(),
                super_class: None,
            }),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn add_method(&self, name: String, method: Value) {
        unsafe { (&mut *self.inner.get()).methods.insert(name, method) };
    }

    pub fn inherit(&self, class: &Rc<Class>) {
        // let mut b = self.methods.borrow_mut();
        // for m in class.methods.borrow().iter() {
        //     b.insert(m.0.clone(), m.1.clone());
        // }
        unsafe { (&mut *self.inner.get()).super_class = Some(class.clone()) }
    }
}

impl fmt::Debug for Class {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Class")
    }
}

impl PartialEq for Class {
    fn eq(&self, other: &Class) -> bool {
        self.inner.get() == other.inner.get()
    }
}

impl Instance for Class {
    fn find_method(&self, name: &str) -> Option<&Value> {
        unsafe {
            let borrow = &*self.inner.get();
            match borrow.methods.iter().find(|m| m.0 == name) {
                Some(s) => Some(s.1),
                None => match borrow.super_class {
                    Some(ref s) => s.find_method(name),
                    None => None,
                },
            }
        }
    }

    fn set_field(&self, _name: &str, _value: Val) -> RuntimeResult<()> {
        Ok(())
    }

    fn get_field(&self, name: &str) -> Option<&Val> {
        println!("name {}", name);
        None
    }
}

#[derive(Debug)]
pub struct ClassInstance {
    pub(crate) class: Rc<Class>,
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

    fn find_method(&self, name: &str) -> Option<&Value> {
        self.class.find_method(name)
    }
}
