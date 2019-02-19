use freja_parser::ast::ProgramRef;
use freja_parser::owned::*;

pub mod env;
pub mod error;
pub mod vm;

pub use vm::VM;

pub trait Interpreter {
    fn interpret(&mut self, ast: Program) -> error::InterpretResult<()>;
    fn interpret_ref<'a>(&mut self, ast: ProgramRef<'a>) -> error::InterpretResult<()> {
        let mut visitor = OwnedVisitor;
        let program = Program {
            statements: ast
                .statements
                .iter()
                .map(|m| m.accept(&mut visitor))
                .collect(),
        };
        self.interpret(program)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
