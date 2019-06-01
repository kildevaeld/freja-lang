use super::super::context::Context;
use super::super::error::RuntimeResult;
use super::super::stack::*;
use super::super::value::Value;
use super::types::*;
use std::any::Any;
use std::cell::UnsafeCell;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

// pub trait Class: Instance {
//     fn name(&self) -> &str;
//     fn construct(&self, ctx: &Context<SubStack>) -> RuntimeResult<()>;
//     fn as_instance(&self) -> &Instance;
//     //fn find_method(&self, name: &str) -> Option<&Value>;
// }

// impl<'a> PartialEq for Class + 'a {
//     fn eq(&self, other: &Self) -> bool {
//         false
//     }
// }


// pub trait ClassInstance: Instance {
//     fn class(&self) -> &Class;
//     fn as_instance(&self) -> &Instance;
//     //fn find_method(&self, name: &str) -> Option<&Value>;
// }

// impl<'a> PartialEq for ClassInstance + 'a {
//     fn eq(&self, other: &Self) -> bool {
//         false
//     }
// }

#[derive(Debug, PartialEq)]
pub struct ClassInner {
    pub(crate) methods: HashMap<String, Value>,
    pub(crate) super_class: Option<Rc<Box<Class>>>,
}

pub struct SourceClass {
    pub(crate) name: String,
    inner: UnsafeCell<ClassInner>,
}

impl SourceClass {
    pub fn new(name: String) -> SourceClass {
        SourceClass {
            name,
            inner: UnsafeCell::new(ClassInner {
                methods: HashMap::new(),
                super_class: None,
            }),
        }
    }

    pub fn add_method(&self, name: String, method: Value) {
        unsafe { (&mut *self.inner.get()).methods.insert(name, method) };
    }

    pub fn inherit(&self, class: &Rc<Box<Class>>) {
        unsafe { (&mut *self.inner.get()).super_class = Some(class.clone()) }
    }
}

impl Class for SourceClass {
    fn name(&self) -> &str {
        &self.name
    }
    fn construct(&self, _ctx: &Context<SubStack>) -> RuntimeResult<()> {
        unreachable!("don't construct source class directly")
    }

    fn as_instance(&self) -> &Instance {
        self
    }

    // fn find_method(&self, name: &str) -> Option<&Value> {
    //     unsafe {
    //         let borrow = &*self.inner.get();
    //         match borrow.methods.iter().find(|m| m.0 == name) {
    //             Some(s) => Some(s.1),
    //             None => match borrow.super_class {
    //                 Some(ref s) => s.find_method(name),
    //                 None => None,
    //             },
    //         }
    //     }
    // }
}

impl fmt::Debug for SourceClass {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Class")
    }
}

impl PartialEq for SourceClass {
    fn eq(&self, other: &SourceClass) -> bool {
        self.inner.get() == other.inner.get()
    }
}

impl Instance for SourceClass {
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

    fn set_field(&self, _name: &str, _value: Value) -> RuntimeResult<()> {
        Ok(())
    }

    fn get_field(&self, name: &str) -> Option<&Value> {
        println!("name {}", name);
        None
    }

    fn as_any(&self) -> &Any {
        self
    }
}

#[derive(Debug)]
pub struct SourceClassInstance {
    pub(crate) class: Rc<Box<Class>>,
    fields: UnsafeCell<HashMap<String, Value>>,
}

impl SourceClassInstance {
    pub fn new(class: Rc<Box<Class>>) -> SourceClassInstance {
        SourceClassInstance {
            class,
            fields: UnsafeCell::new(HashMap::new()),
        }
    }

    fn fields(&self) -> &mut HashMap<String, Value> {
        unsafe { &mut *self.fields.get() }
    }
}

impl ClassInstance for SourceClassInstance {
    fn class(&self) -> &Class {
        self.class.as_ref().as_ref()
    }
    fn as_instance(&self) -> &Instance {
        self
    }
}

impl PartialEq for SourceClassInstance {
    fn eq(&self, other: &SourceClassInstance) -> bool {
        self.class == other.class && unsafe { (&*self.fields.get()) == (&*other.fields.get()) }
    }
}

impl Clone for SourceClassInstance {
    fn clone(&self) -> SourceClassInstance {
        SourceClassInstance {
            class: self.class.clone(),
            fields: UnsafeCell::new(self.fields().clone()),
        }
    }
}

impl Instance for SourceClassInstance {
    fn set_field(&self, name: &str, value: Value) -> RuntimeResult<()> {
        self.fields().insert(name.to_string(), value);
        Ok(())
    }

    fn get_field(&self, name: &str) -> Option<&Value> {
        self.fields().get(name)
    }

    fn find_method(&self, name: &str) -> Option<&Value> {
        self.class.find_method(name)
    }

    fn as_any(&self) -> &Any {
        self
    }
}
