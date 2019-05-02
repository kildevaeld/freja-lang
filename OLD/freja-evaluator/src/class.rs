use freja_ast::{Argument, ClassStmt};
use freja_runtime::*;
use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

struct FrejaClassInner {
    name: String,
    methods: Rc<Vec<(String, Box<dyn FrejaCallable>)>>,
    closure: EnvPtr<Rc<Value>>,
}

#[derive(Clone)]
pub struct FrejaClass {
    inner: Rc<FrejaClassInner>,
}

impl fmt::Debug for FrejaClass {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<Class {}>", self.inner.name)
    }
}

impl FrejaClass {
    pub fn new(
        name: String,
        methods: Vec<(String, Box<dyn FrejaCallable>)>,
        closure: EnvPtr<Rc<Value>>,
    ) -> FrejaClass {
        FrejaClass {
            inner: Rc::new(FrejaClassInner {
                name,
                methods: Rc::new(methods),
                // .into_iter()
                // .map(|m| (m.0, Rc::new(Value::Function(m.1))))
                // .collect::<Vec<_>>(),
                closure,
            }),
        }
    }
}

impl FrejaCallable for FrejaClass {
    fn call(&self, _vm: &mut VM, _args: Vec<ValuePtr>) -> RuntimeResult<ValuePtr> {
        Ok(ValuePtr::new(Value::Instance(Box::new(
            FrejaClassInstance::new(self.clone()),
        ))))
    }
    fn arity(&self) -> u8 {
        0
    }

    fn bind(&self, instance: ValuePtr) -> Box<dyn FrejaCallable> {
        let mut env = Env::with_parent(self.inner.closure.clone());
        env.define("this", instance);
        Box::new(FrejaClass {
            inner: Rc::new(FrejaClassInner {
                name: self.inner.name.clone(),
                closure: EnvPtr::new(RefCell::new(env)),
                methods: self.inner.methods.clone(),
            }),
        })
    }
}

#[derive(Debug, Clone)]
pub struct FrejaClassInstance {
    class: FrejaClass,
}

impl FrejaClassInstance {
    pub fn new(class: FrejaClass) -> FrejaClassInstance {
        FrejaClassInstance { class }
    }
}

impl Instance for FrejaClassInstance {
    fn get(&self, name: &str) -> RuntimeResult<ValuePtr> {
        match self
            .class
            .inner
            .methods
            .iter()
            .find(|m| m.0.as_str() == name)
        {
            Some(s) => Ok(ValuePtr::new(Value::Function(
                s.1.bind(Rc::new(Value::Instance(Box::new(self.clone())))),
            ))),
            None => Err("invalid property".into()),
        }
    }
}
