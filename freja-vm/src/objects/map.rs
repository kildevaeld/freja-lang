use super::super::context::Context;
use super::super::error::{RuntimeError, RuntimeResult};
use super::super::stack::SubStack;
use super::super::value::Value;
use super::native::NativeFn;
use super::types::Instance;
use super::types::{Class, ClassInstance};
use std::any::Any;
use std::cell::UnsafeCell;

use std::collections::HashMap;
use std::rc::Rc;
// "map" => NativeFn::value(|ctx| {
//     let array = ctx.get(0).unwrap().as_instance().unwrap().as_any().downcast_ref::<MapInstance>().unwrap();
//     let inner = unsafe { (&mut *array.inner.get()) };
//     let mut out = Vec::new();
//     for (i, v) in inner.iter_mut().enumerate() {
//         ctx.dup(1)?;
//         ctx.stack.push(v.as_ptr())?;
//         ctx.push(Value::Integer(i as i64))?;
//         ctx.call(2)?;
//         let item = ctx.pop().unwrap();
//         out.push(item);
//     }
//     Ok(Value::Array(Rc::new(Array::new(out))))
// }, 1)


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
pub struct Map {
    methods: HashMap<&'static str, Value>,
}

impl Map {
    pub fn new() -> Map {
        let methods = map! {
            "len" => NativeFn::value(
                |ctx| {
                    let object = ctx.get(0).unwrap().as_instance().unwrap().as_any().downcast_ref::<MapInstance>().unwrap();
                    Ok(Value::Integer(object.len() as i64))
                },
                0,
            )
            /*"get" => NativeFn::value(
                |ctx| {
                    let array = ctx.get(0).unwrap().as_instance().unwrap().as_any().downcast_ref::<MapInstance>().unwrap();
                    let idx = match ctx.get(1).unwrap().as_ref() {
                        Value::Integer(i) => *i,
                        Value::Double(d) => *d as i64,
                        _ => unimplemented!("get"),
                    };
                    match array.get(idx as usize) {
                        None => Ok(Value::Null),
                        Some(s) => Ok(s.as_ref().clone()),
                    }
                },
                1,
            ),
            "push" => NativeFn::value(|ctx| {
                let array = ctx.get(0).unwrap().as_instance().unwrap().as_any().downcast_ref::<MapInstance>().unwrap();
                let val = ctx.pop().unwrap();
                array.push(val);
                Ok(ctx.get(0).unwrap().as_ref().clone())
            }, 1),
            "each" => NativeFn::value(|ctx| {

                let array = ctx.get(0).unwrap().as_instance().unwrap().as_any().downcast_ref::<MapInstance>().unwrap();
                let inner = unsafe { (&mut *array.inner.get()) };
                for (i, v) in inner.iter_mut().enumerate() {
                     ctx.dup(1)?;
                     ctx.stack.push(v.as_ptr())?;
                     ctx.push(Value::Integer(i as i64))?;
                     ctx.call(2)?;
                     ctx.pop();
                }
                Ok(ctx.get(0).unwrap().as_ref().clone())
            }, 1)*/

        };
        Map { methods }
    }
}

impl Class for Map {
    fn name(&self) -> &str {
        "Map"
    }

    fn construct(&self, ctx: &Context<SubStack>) -> RuntimeResult<()> {
        let v = &ctx.stack.as_ref()[1..];
        if v.len() % 2 != 0 {
            println!("{}", ctx.dump());
            return Err(RuntimeError::Error(format!("invalid number of arguments: {}", v.len())));
        }

        let class = match ctx.get(0).expect("instance").as_class() {
            None => return Err("invalid instance".into()),
            Some(c) => c,
        };

        let mut i = 0;
        let mut map = HashMap::new();
        while i < v.len() {
            let key = &v[i];
            let val = &v[i + 1];
            i += 2;
            map.insert(key.to_string(), val.clone().into_inner());
        }

        ctx.set(
            0,
            Value::ClassInstance(Rc::new(Box::new(MapInstance::new(class.clone(), map)))),
        );

        Ok(())
    }

    fn as_instance(&self) -> &Instance {
        self
    }
}

impl Instance for Map {
    fn set_field(&self, _name: &str, _value: Value) -> RuntimeResult<()> {
        Ok(())
    }

    fn get_field(&self, name: &str) -> Option<&Value> {
        println!("name {}", name);
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
pub struct MapInstance {
    inner: UnsafeCell<HashMap<String, Value>>,
    class: Rc<Box<Class>>,
}

impl MapInstance {
    pub fn new(class: Rc<Box<Class>>, data: HashMap<String, Value>) -> MapInstance {
        MapInstance {
            inner: UnsafeCell::new(data),
            class,
        }
    }

    pub fn len(&self) -> usize {
        unsafe { (&*self.inner.get()).len() }
    }

    pub fn get(&self, name: &str) -> Option<&Value> {
        unsafe { (&*self.inner.get()).get(name) }
    }

    pub fn insert(&self, name: &str, val: Value) {
        unsafe {
            (&mut *self.inner.get()).insert(name.to_string(), val);
        }
    }

    pub fn is_empty(&self) -> bool {
        unsafe { (&*self.inner.get()).is_empty() }
    }
}


impl ClassInstance for MapInstance {
    fn class(&self) -> &Class {
        self.class.as_ref().as_ref()
    }

    fn as_instance(&self) -> &Instance {
        self
    }
}

impl Instance for MapInstance {
    fn set_field(&self, name: &str, value: Value) -> RuntimeResult<()> {
        self.insert(name, value);
        Ok(())
    }

    fn get_field(&self, name: &str) -> Option<&Value> {
        self.get(name)
    }

    fn find_method(&self, name: &str) -> Option<&Value> {
        self.class.find_method(name)
    }

    fn as_any(&self) -> &Any {
        self
    }
}