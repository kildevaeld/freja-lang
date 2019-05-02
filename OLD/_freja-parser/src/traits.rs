use super::ast::*;
use super::owned::*;

pub trait StmtRefVisitor<R> {
    fn visitVarStmt(&mut self, stmt: &VarStmtRef) -> R;
    fn visitExprStmt(&mut self, stmt: &ExprStmtRef) -> R;
    fn visitFuncStmt(&mut self, stmt: &FuncStmtRef) -> R;
    fn visitBlockStmt(&mut self, stmt: &BlockStmtRef) -> R;
    fn visitReturnStmt(&mut self, stmt: &ReturnStmtRef) -> R;
    //fn visitProgram(&mut self, stmt: ProgramRef) -> R;
}

pub trait ExprRefVisitor<R> {
    fn visitAssignExpr(&mut self, expr: &AssignExprRef) -> R;
    fn visitIdentifierExpr(&mut self, expr: &IdentifierExprRef) -> R;
    fn visitCallExpr(&mut self, expr: &CallExprRef) -> R;
    fn visitLiteralExpr(&mut self, expr: &LiteralExprRef) -> R;
}

pub trait StmtVisitor<R> {
    fn visitVarStmt(&mut self, stmt: VarStmt) -> R;
    fn visitExprStmt(&mut self, stmt: ExprStmt) -> R;
    fn visitFuncStmt(&mut self, stmt: FuncStmt) -> R;
    fn visitBlockStmt(&mut self, stmt: BlockStmt) -> R;
    fn visitReturnStmt(&mut self, stmt: ReturnStmt) -> R;
    //fn visitProgram(&mut self, stmt: ProgramRef) -> R;
}

pub trait ExprVisitor<R> {
    fn visitAssignExpr(&mut self, expr: AssignExpr) -> R;
    fn visitIdentifierExpr(&mut self, expr: IdentifierExpr) -> R;
    fn visitCallExpr(&mut self, expr: CallExpr) -> R;
    fn visitLiteralExpr(&mut self, expr: LiteralExpr) -> R;
}
