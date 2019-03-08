use super::env::{Builtin, Env, Instance};
use super::error::{Error, InterpretResult};
use super::Interpreter;
// use freja_parser2::owned::*;
// use freja_parser::traits::{ExprVisitor, StmtVisitor};
use freja_parser2::ast::*;
use std::rc::Rc;

#[derive(Debug)]
pub struct VM {
    env: Env,
}

impl VM {
    pub fn new() -> VM {
        VM { env: Env::new() }
    }

    fn evaluate(&mut self, expr: &Expr) -> InterpretResult<Instance> {
        expr.accept(self)
    }

    fn execute(&mut self, stmt: &Stmt) -> InterpretResult<()> {
        stmt.accept(self)
    }
}

impl Interpreter for VM {
    fn interpret(&mut self, ast: &Stmt) -> InterpretResult<()> {
        // for stmt in ast.statements {
        //     self.execute(stmt)?;
        // }
        // Ok(())
        self.execute(ast)
    }
}

impl StmtVisitor<InterpretResult<()>> for VM {
    // fn visit_var_stmt(&mut self, stmt: VarStmt) -> InterpretResult<()> {
    //     let val = self.evaluate(stmt.initializer)?;
    //     self.env.define(stmt.name.to_string(), val)?;
    //     Ok(())
    // }
    // fn visitExprStmt(&mut self, stmt: ExprStmt) -> InterpretResult<()> {
    //     self.evaluate(stmt.expression).map(|_| ())
    // }
    // fn visitFuncStmt(&mut self, stmt: FuncStmt) -> InterpretResult<()> {
    //     Err("visit func stmt".into())
    // }
    // fn visitBlockStmt(&mut self, stmt: BlockStmt) -> InterpretResult<()> {
    //     Err("visit block stmt".into())
    // }
    // fn visitReturnStmt(&mut self, stmt: ReturnStmt) -> InterpretResult<()> {
    //     Err("visit return stmt".into())
    // }
    fn visit_program_stmt(&mut self, e: &ProgramStmt) -> InterpretResult<()> {
        for b in &e.statements {
            self.execute(&b)?;
        }
        Ok(())
    }
    fn visit_var_stmt(&mut self, stmt: &VarStmt) -> InterpretResult<()> {
        let v = stmt.initializer.as_ref().unwrap();
        let val = self.evaluate(&v)?;
        self.env.define(stmt.name.to_string(), val)?;
        Ok(())
    }
    fn visit_varlist_stmt(&mut self, e: &VarListStmt) -> InterpretResult<()> {
        for v in &e.variables {
            self.visit_var_stmt(&v)?;
        }
        Ok(())
    }
    fn visit_expr_stmt(&mut self, e: &ExprStmt) -> InterpretResult<()> {
        Err("exprstmt".into())
    }
    fn visit_func_stmt(&mut self, e: &FuncStmt) -> InterpretResult<()> {
        Err("func".into())
    }
    fn visit_class_stmt(&mut self, e: &ClassStmt) -> InterpretResult<()> {
        Err("class".into())
    }
    fn visit_block_stmt(&mut self, e: &BlockStmt) -> InterpretResult<()> {
        Err("block".into())
    }
    fn visit_if_stmt(&mut self, e: &IfStmt) -> InterpretResult<()> {
        Err("if".into())
    }
    fn visit_for_stmt(&mut self, e: &ForStmt) -> InterpretResult<()> {
        Err("for".into())
    }
    fn visit_return_stmt(&mut self, e: &ReturnStmt) -> InterpretResult<()> {
        Err("return".into())
    }
    fn visit_continue_stmt(&mut self, e: &ContinueStmt) -> InterpretResult<()> {
        Err("err".into())
    }
    fn visit_break_stmt(&mut self, e: &BreakStmt) -> InterpretResult<()> {
        Err("break".into())
    }
}

impl ExprVisitor<InterpretResult<Instance>> for VM {
    // fn visitAssignExpr(&mut self, expr: AssignExpr) -> InterpretResult<Instance> {
    //     Err("visit assign expr".into())
    // }
    // fn visitIdentifierExpr(&mut self, expr: IdentifierExpr) -> InterpretResult<Instance> {
    //     Err("visit identifier expr".into())
    // }
    // fn visitCallExpr(&mut self, expr: CallExpr) -> InterpretResult<Instance> {
    //     Err("visit call expr".into())
    // }
    // fn visitLiteralExpr(&mut self, expr: LiteralExpr) -> InterpretResult<Instance> {
    //     match expr.value {
    //         Literal::String(s) => Ok(Instance::Builtin(Builtin::String(s))),
    //         _ => Err("visit literal expr".into()),
    //     }
    // }
    fn visit_assign_expr(&mut self, e: &AssignExpr) -> InterpretResult<Instance> {
        Err("".into())
    }
    fn visit_call_expr(&mut self, e: &CallExpr) -> InterpretResult<Instance> {
        Err("".into())
    }
    fn visit_literal_expr(&mut self, expr: &LiteralExpr) -> InterpretResult<Instance> {
        match expr.value {
            Literal::String(s) => Ok(Instance::Builtin(Builtin::String(s.to_owned()))),
            _ => Err("visit literal expr".into()),
        }
    }
    fn visit_binary_expr(&mut self, e: &BinaryExpr) -> InterpretResult<Instance> {
        Err("".into())
    }
    fn visit_member_expr(&mut self, e: &MemberExpr) -> InterpretResult<Instance> {
        Err("".into())
    }
    fn visit_lookup_expr(&mut self, e: &LookupExpr) -> InterpretResult<Instance> {
        Err("".into())
    }
    fn visit_arguments_expr(&mut self, e: &ArgumentsExpr) -> InterpretResult<Instance> {
        Err("".into())
    }
    fn visit_logical_expr(&mut self, e: &LogicalExpr) -> InterpretResult<Instance> {
        Err("".into())
    }
}
