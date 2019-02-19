use super::Interpreter;
use super::env::Instance;

pub struct Callable {
    type Interpreter: Interpreter;
    fn arity() -> i32;
    fn call(vm: &mut I) -> Instance;
}