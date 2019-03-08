use super::Interpreter;
use super::env::Instance;
use super::vm::VM
pub trait Callable {
    type Interpreter: Interpreter;
    fn arity() -> i32;
    fn call(vm: &mut I) -> Instance;
}


pub struct Function {
    type
}