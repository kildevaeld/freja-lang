use super::env::{Builtin, Env, Instance};
use super::error::{Error, InterpretResult};
use super::Interpreter;
use freja_parser::owned::*;
use freja_parser::traits::{ExprVisitor, StmtVisitor};
use std::rc::Rc;

#[derive(Debug)]
pub struct VM {
    env: Env,
}

impl VM {
    pub fn new() -> VM {
        VM { env: Env::new() }
    }

    fn evaluate(&mut self, expr: Expr) -> InterpretResult<Instance> {
        expr.accept(self)
    }

    fn execute(&mut self, stmt: Stmt) -> InterpretResult<()> {
        stmt.accept(self)
    }
}

impl Interpreter for VM {
    fn interpret(&mut self, ast: Program) -> InterpretResult<()> {
        for stmt in ast.statements {
            self.execute(stmt)?;
        }
        Ok(())
    }
}

impl StmtVisitor<InterpretResult<()>> for VM {
    fn visitVarStmt(&mut self, stmt: VarStmt) -> InterpretResult<()> {
        let val = self.evaluate(stmt.initializer)?;
        self.env.define(stmt.name.to_string(), val)?;
        Ok(())
    }
    fn visitExprStmt(&mut self, stmt: ExprStmt) -> InterpretResult<()> {
        self.evaluate(stmt.expression).map(|_| ())
    }
    fn visitFuncStmt(&mut self, stmt: FuncStmt) -> InterpretResult<()> {
        Err("visit func stmt".into())
    }
    fn visitBlockStmt(&mut self, stmt: BlockStmt) -> InterpretResult<()> {
        Err("visit block stmt".into())
    }
    fn visitReturnStmt(&mut self, stmt: ReturnStmt) -> InterpretResult<()> {
        Err("visit return stmt".into())
    }
}

impl ExprVisitor<InterpretResult<Instance>> for VM {
    fn visitAssignExpr(&mut self, expr: AssignExpr) -> InterpretResult<Instance> {
        Err("visit assign expr".into())
    }
    fn visitIdentifierExpr(&mut self, expr: IdentifierExpr) -> InterpretResult<Instance> {
        Err("visit identifier expr".into())
    }
    fn visitCallExpr(&mut self, expr: CallExpr) -> InterpretResult<Instance> {
        Err("visit call expr".into())
    }
    fn visitLiteralExpr(&mut self, expr: LiteralExpr) -> InterpretResult<Instance> {
        match expr.value {
            Literal::String(s) => Ok(Instance::Builtin(Builtin::String(s))),
            _ => Err("visit literal expr".into()),
        }
    }
}
