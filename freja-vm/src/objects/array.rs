use super::super::context::Context;
use super::super::error::RuntimeResult;
use super::super::stack::{Stack, SubStack};
use super::super::utils::Pointer;
use super::super::value::Value;
use super::native::NativeFn;
use super::types::Instance;
use super::types::{Class, ClassInstance};
use std::any::Any;
use std::cell::UnsafeCell;
use std::collections::HashMap;
use std::rc::Rc;


macro_rules! map(
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = ::std::collections::HashMap::new();
            $(
                m.insert($key, $value);
            )+
            m
        }
     };
);


#[derive(Debug)]
pub struct Array {
    methods: HashMap<&'static str, Value>,
}

impl Array {
    pub fn new() -> Array {
        let methods = map! {
            "len" => NativeFn::value(
                |ctx| {
                    let array = ctx.get(0).unwrap().as_instance().unwrap().as_any().downcast_ref::<ArrayInstance>().unwrap();
                    Ok(Value::Integer(array.len() as i64))
                },
                0,
            ),
            "get" => NativeFn::value(
                |ctx| {
                    let array = ctx.get(0).unwrap().as_instance().unwrap().as_any().downcast_ref::<ArrayInstance>().unwrap();
                    let idx = match ctx.get(1).unwrap().as_ref() {
                        Value::Integer(i) => *i,
                        Value::Double(d) => *d as i64,
                        _ => unimplemented!("get"),
                    };
                    match array.get(idx as usize) {
                        None => Ok(Value::Null),
                        Some(s) => Ok(s.clone()),
                    }
                },
                1,
            ),
            "push" => NativeFn::value(|ctx| {
                let array = ctx.get(0).unwrap().as_instance().unwrap().as_any().downcast_ref::<ArrayInstance>().unwrap();
                let val = ctx.pop().unwrap();
                array.push(val.into_inner());
                Ok(ctx.get(0).unwrap().as_ref().clone())
            }, 1),
            "each" => NativeFn::value(|ctx| {
                let array = ctx.get(0).unwrap().as_instance().unwrap().as_any().downcast_ref::<ArrayInstance>().unwrap();
                let inner = unsafe { (&mut *array.inner.get()) };
                for (i, v) in inner.iter_mut().enumerate() {
                     ctx.dup(1)?;
                     ctx.stack.push(Pointer::Ref(v as *const Value))?;
                     ctx.push(Value::Integer(i as i64))?;
                     ctx.call(2)?;
                     ctx.pop();
                }
                Ok(ctx.get(0).unwrap().as_ref().clone())
            }, 1)

        };
        Array { methods }
    }
}

impl Class for Array {
    fn name(&self) -> &str {
        "Array"
    }

    fn construct(&self, ctx: &Context<SubStack>) -> RuntimeResult<()> {

        let v = (&ctx.stack.as_ref()[1..])
            .iter()
            .map(|m| m.clone().into_inner())
            .collect();
        let class = match ctx.get(0).unwrap().as_class() {
            None => return Err("invalid instance".into()),
            Some(c) => c,
        };

        ctx.set(
            0,
            Value::ClassInstance(Rc::new(Box::new(ArrayInstance::new(class.clone(), v)))),
        );

        Ok(())
    }

    fn as_instance(&self) -> &Instance {
        self
    }
}

impl Instance for Array {
    fn set_field(&self, _name: &str, _value: Value) -> RuntimeResult<()> {
        Ok(())
    }

    fn get_field(&self, _name: &str) -> Option<&Value> {
        None
    }

    fn find_method(&self, name: &str) -> Option<&Value> {
        self.methods.get(name)
    }

    fn as_any(&self) -> &Any {
        self
    }
}


#[derive(Debug)]
pub struct ArrayInstance {
    inner: UnsafeCell<Vec<Value>>,
    class: Rc<Box<Class>>,
}

impl ArrayInstance {
    pub fn new(class: Rc<Box<Class>>, data: Vec<Value>) -> ArrayInstance {
        ArrayInstance {
            inner: UnsafeCell::new(data),
            class,
        }
    }

    pub fn len(&self) -> usize {
        unsafe { (&*self.inner.get()).len() }
    }

    pub fn get(&self, idx: usize) -> Option<&Value> {
        unsafe { (&*self.inner.get()).get(idx) }
    }

    pub fn push(&self, val: Value) {
        unsafe { (&mut *self.inner.get()).push(val) }
    }

    pub fn is_empty(&self) -> bool {
        unsafe { (&*self.inner.get()).is_empty() }
    }
}


impl ClassInstance for ArrayInstance {
    fn class(&self) -> &Class {
        self.class.as_ref().as_ref()
    }

    fn as_instance(&self) -> &Instance {
        self
    }
}

impl Instance for ArrayInstance {
    fn set_field(&self, _name: &str, _value: Value) -> RuntimeResult<()> {
        Ok(())
    }

    fn get_field(&self, name: &str) -> Option<&Value> {
        println!("name {}", name);
        None
    }

    fn find_method(&self, name: &str) -> Option<&Value> {
        self.class.find_method(name)
    }

    fn as_any(&self) -> &Any {
        self
    }
}