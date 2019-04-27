// use super::env::{Builtin, Env, Value};
// use super::error::{Error, RuntimeResult};
// use super::Interpreter;
// use freja_parser2::owned::*;
// use freja_parser::traits::{ExprVisitor, StmtVisitor};
use freja_parser::ast::*;
use freja_runtime::{value_binary, Env, FrejaCallable, RuntimeResult, Value, VM as VMBase};
use std::fmt;
use std::rc::Rc;

pub struct NativeFunction<F: 'static> {
    inner: F,
}

impl<F: 'static> FrejaCallable for NativeFunction<F>
where
    F: Fn(&mut VMBase, Vec<&Value>) -> RuntimeResult<Value>,
{
    fn call(&self, vm: &mut VMBase, args: Vec<&Value>) -> RuntimeResult<Value> {
        (self.inner)(vm, args)
    }

    fn arity(&self) -> u8 {
        1
    }
}

impl<F: 'static> fmt::Debug for NativeFunction<F> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Callable")
    }
}

macro_rules! native_fn {
    ($closure: expr) => {
        Value::Function(Box::new(NativeFunction { inner: $closure }))
    };
}

#[derive(Debug)]
pub struct VM {
    env: Env<Rc<Value>>,
}

impl VM {
    pub fn new() -> VM {
        let mut vm = VM { env: Env::new() };
        let f = |vm: &mut VMBase, args: Vec<&Value>| {
            println!("{}", args[0]);
            Ok(Value::Null)
        };
        vm.env.define(
            "print",
            Rc::new(Value::Function(Box::new(NativeFunction { inner: f }))),
        );
        vm
    }

    pub fn interpret(&mut self, ast: &Stmt) -> RuntimeResult<()> {
        self.execute(ast)
    }

    fn evaluate(&mut self, expr: &Expr) -> RuntimeResult<Rc<Value>> {
        expr.accept(self)
    }

    fn execute(&mut self, stmt: &Stmt) -> RuntimeResult<()> {
        stmt.accept(self)
    }
}

impl VMBase for VM {
    // fn interpret(&mut self, ast: &Stmt) -> RuntimeResult<()> {
    //     // for stmt in ast.statements {
    //     //     self.execute(stmt)?;
    //     // }
    //     // Ok(())
    //     self.execute(ast)
    // }
}

impl StmtVisitor<RuntimeResult<()>> for VM {
    fn visit_program_stmt(&mut self, e: &ProgramStmt) -> RuntimeResult<()> {
        for b in &e.statements {
            self.execute(&b)?;
        }
        Ok(())
    }
    fn visit_var_stmt(&mut self, stmt: &VarStmt) -> RuntimeResult<()> {
        let v = stmt.initializer.as_ref().expect("initializer");
        let val = self.evaluate(&v)?;
        self.env.define(stmt.name.to_string(), val)?;
        Ok(())
    }
    fn visit_varlist_stmt(&mut self, e: &VarListStmt) -> RuntimeResult<()> {
        for v in &e.variables {
            self.visit_var_stmt(&v)?;
        }
        Ok(())
    }
    fn visit_expr_stmt(&mut self, e: &ExprStmt) -> RuntimeResult<()> {
        self.evaluate(&e.expression)?;
        Ok(())
    }
    fn visit_func_stmt(&mut self, e: &FuncStmt) -> RuntimeResult<()> {
        Err("func".into())
    }
    fn visit_class_stmt(&mut self, e: &ClassStmt) -> RuntimeResult<()> {
        Err("class".into())
    }
    fn visit_block_stmt(&mut self, e: &BlockStmt) -> RuntimeResult<()> {
        Err("block".into())
    }
    fn visit_if_stmt(&mut self, e: &IfStmt) -> RuntimeResult<()> {
        Err("if".into())
    }
    fn visit_for_stmt(&mut self, e: &ForStmt) -> RuntimeResult<()> {
        Err("for".into())
    }
    fn visit_return_stmt(&mut self, e: &ReturnStmt) -> RuntimeResult<()> {
        Err("return".into())
    }
    fn visit_continue_stmt(&mut self, e: &ContinueStmt) -> RuntimeResult<()> {
        Err("err".into())
    }
    fn visit_break_stmt(&mut self, e: &BreakStmt) -> RuntimeResult<()> {
        Err("break".into())
    }
}

impl ExprVisitor<RuntimeResult<Rc<Value>>> for VM {
    // fn visitAssignExpr(&mut self, expr: AssignExpr) -> RuntimeResult<Rc<Value>> {
    //     Err("visit assign expr".into())
    // }
    // fn visitIdentifierExpr(&mut self, expr: IdentifierExpr) -> RuntimeResult<Rc<Value>> {
    //     Err("visit identifier expr".into())
    // }
    // fn visitCallExpr(&mut self, expr: CallExpr) -> RuntimeResult<Rc<Value>> {
    //     Err("visit call expr".into())
    // }
    // fn visitLiteralExpr(&mut self, expr: LiteralExpr) -> RuntimeResult<Rc<Value>> {
    //     match expr.value {
    //         Literal::String(s) => Ok(Value::Builtin(Builtin::String(s))),
    //         _ => Err("visit literal expr".into()),
    //     }
    // }
    fn visit_assign_expr(&mut self, e: &AssignExpr) -> RuntimeResult<Rc<Value>> {
        Err("assign".into())
    }
    fn visit_call_expr(&mut self, e: &CallExpr) -> RuntimeResult<Rc<Value>> {
        let callee = self.evaluate(&e.member)?;

        let args = match e.arguments.as_ref() {
            Expr::Arguments(args) => args,
            _ => unreachable!("should be arggs"),
        };

        let args = args
            .expressions
            .iter()
            .map(|m| self.evaluate(m).unwrap())
            .collect::<Vec<_>>();

        let refed = args.iter().map(|m| m.as_ref()).collect::<Vec<_>>();

        match callee.as_ref() {
            Value::Function(f) => f.call(self, refed).map(|m| Rc::new(m)),
            _ => Err("not a function".into()),
        }
        //Err("call".into())
    }

    fn visit_literal_expr(&mut self, expr: &LiteralExpr) -> RuntimeResult<Rc<Value>> {
        let val = match &expr.value {
            Literal::String(s) => Value::String((*s).to_owned()),
            Literal::Number(n) => Value::Number(n.clone()),
            _ => return Err("visit literal expr".into()),
        };
        Ok(Rc::new(val))
    }
    fn visit_binary_expr(&mut self, e: &BinaryExpr) -> RuntimeResult<Rc<Value>> {
        let left = self.evaluate(&e.left)?;
        let right = self.evaluate(&e.right)?;
        Ok(Rc::new(value_binary(&left, &right, &e.operator)?))
    }
    fn visit_member_expr(&mut self, e: &MemberExpr) -> RuntimeResult<Rc<Value>> {
        Err("member".into())
    }
    fn visit_lookup_expr(&mut self, e: &LookupExpr) -> RuntimeResult<Rc<Value>> {
        let lookup = match &e.token.kind {
            TokenType::Identifier => e.token.value,
            _ => return Err("invalid lookup token".into()),
        };

        let val = self.env.get(lookup).unwrap();

        Ok(val.clone())
    }
    fn visit_arguments_expr(&mut self, e: &ArgumentsExpr) -> RuntimeResult<Rc<Value>> {
        Err("arguments".into())
    }
    fn visit_logical_expr(&mut self, e: &LogicalExpr) -> RuntimeResult<Rc<Value>> {
        Err("logical".into())
    }
}
