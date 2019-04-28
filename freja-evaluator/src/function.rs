use freja_ast::{Argument, FuncStmt};
use freja_runtime::*;
use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

pub struct Function {
    inner: FuncStmt,
    closure: EnvPtr<Rc<Value>>,
}

impl fmt::Debug for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Function")
    }
}

impl Function {
    pub fn new(func: FuncStmt, closure: EnvPtr<Rc<Value>>) -> Function {
        Function {
            inner: func,
            closure,
        }
    }

    pub fn name(&self) -> &str {
        self.inner.name.value.as_str()
    }
}

impl FrejaCallable for Function {
    fn call(&self, vm: &mut VM, args: Vec<Rc<Value>>) -> RuntimeResult<Rc<Value>> {
        let mut env = Env::with_parent(self.closure.clone());
        for (i, v) in args.iter().enumerate() {
            match &self.inner.parameters[i] {
                Argument::Regular(arg) => {
                    env.define(arg, v.clone())?;
                }
                Argument::Rest(rest) => {
                    let a = args.iter().skip(i).map(|m| m.clone()).collect::<Vec<_>>();
                    env.define(rest, Rc::new(Value::Array(a)))?;
                    break;
                }
            }
        }

        let env = Rc::new(RefCell::new(env));
        match vm.execute_block(&vec![&self.inner.body], env) {
            Ok(m) => Ok(Rc::new(Value::Null)),
            Err(RuntimeError::Return(v)) => Ok(v),
            Err(e) => Err(e),
        }

        //Ok(Value::Null)
    }
    fn arity(&self) -> u8 {
        self.inner.parameters.len() as u8
    }

    fn bind(&self, instance: ValuePtr) -> Box<dyn FrejaCallable> {
        let mut env = Env::with_parent(self.closure.clone());
        env.define("this", instance);
        Box::new(Function::new(
            self.inner.clone(),
            EnvPtr::new(RefCell::new(env)),
        ))
    }
}
