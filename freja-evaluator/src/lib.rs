use freja_parser::ast::*;

// pub mod env;
// pub mod error;
mod function;
pub mod vm;

pub use vm::VM;

// pub trait Interpreter {
//     fn interpret(&mut self, ast: &Stmt) -> error::InterpretResult<()>;
// }

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
