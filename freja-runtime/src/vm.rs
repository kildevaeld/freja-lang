use super::error::RuntimeResult;
use super::scope::EnvPtr;
use super::value::Value;
use freja_ast::Stmt;
use std::rc::Rc;

pub trait VM {
    fn execute_block(&mut self, stmts: &Vec<&Stmt>, env: EnvPtr<Rc<Value>>) -> RuntimeResult<()>;
}
