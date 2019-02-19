#[derive(Serialize, Debug, Clone, PartialEq, Deserialize)]
#[serde(tag = "type", content = "value")]
pub enum Literal
  String(String),
  Number(Number),
  Boolean(bool)
}
#[derive(Serialize, Debug, Clone, PartialEq, Deserialize)]
#[serde(tag = "type", content = "value")]
pub enum Number
  Double(f64),
  Integer(i64)
}
#[derive(Serialize, Debug, Clone, PartialEq, Deserialize)]
#[serde(tag = "type", content = "value")]
pub enum LiteralRef<'a>
  String(&'a str),
  Number(Number<'a>),
  Boolean(bool),
}
#[derive(Serialize, Debug, Clone, PartialEq, Deserialize)]
#[serde(tag = "type", content = "value")]
pub enum NumberRef<'a>
  Double(&'a str),
  Integer(&'a str)
}

use super::owned::*;
use super::traits::{ExprRefVisitor, StmtRefVisitor};
use std::fmt;

#[derive(Serialize, Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
pub struct Location(pub usize, pub usize);

#[derive(Serialize, Debug, PartialEq, Eq, Hash, Clone)]
pub struct TokenRef<'a> {
    pub location: Location,
    pub value: &'a str,
}

impl<'a> TokenRef<'a> {
    pub fn new(location: Location, value: &'a str) -> TokenRef<'a> {
        TokenRef { location, value }
    }
}

impl<'a> fmt::Display for TokenRef<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Serialize, Debug, Clone, PartialEq, Deserialize)]
#[serde(tag = "type", content = "value")]
pub enum LiteralRef<'a> {
    String(&'a str),
    Number(NumberRef<'a>),
}

impl<'a> fmt::Display for LiteralRef<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LiteralRef::String(s) => write!(f, "{}", s),
            LiteralRef::Number(n) => <fmt::Display>::fmt(n, f),
        }
    }
}

#[derive(Serialize, Debug, Clone, PartialEq, Deserialize)]
#[serde(tag = "type", content = "value")]
pub enum NumberRef<'a> {
    Integer(&'a str),
    Float(&'a str),
}

impl<'a> fmt::Display for NumberRef<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match *self {
            NumberRef::Integer(s) => s,
            NumberRef::Float(n) => n,
        };
        write!(f, "{}", s)
    }
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub enum Expr {
  Assign(AssignExpr)
  Call(CallExpr)
  Literal(LiteralExpr)
  Binary(BinaryExpr)
}
impl<'a> StmtRef<'a> {
  pub fn accept<R>(&self, visitor: &mut ExprVisitor<R>) -> R {
    match self {
      StmtRef::Assign(e) => visitor.visitAssignStmt(&e),
      StmtRef::Call(e) => visitor.visitCallStmt(&e),
      StmtRef::Literal(e) => visitor.visitLiteralStmt(&e),
      StmtRef::Binary(e) => visitor.visitBinaryStmt(&e)
    }
  }
}
#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct AssignExpr {
  location: Location,
  token: Token,
  value: Expr
}
#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct CallExpr {
  location: Location,
  callee: Expr,
  parent: Option<Expr>,
  arguments: Vec<Expr>
}
#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct LiteralExpr {
  location: Location,
  value: Literal
}
#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct BinaryExpr {
  location: Location,
  left: Expr,
  right: Expr,
  operator: Token
}
#[derive(Serialize, Debug, Clone, PartialEq)]
pub enum Stmt {
  Var(VarStmt)
  Expr(ExprStmt)
  Func(FuncStmt)
  Block(BlockStmt)
  Return(ReturnStmt)
}
impl<'a> StmtRef<'a> {
  pub fn accept<R>(&self, visitor: &mut StmtVisitor<R>) -> R {
    match self {
      StmtRef::Var(e) => visitor.visitVarStmt(&e),
      StmtRef::Expr(e) => visitor.visitExprStmt(&e),
      StmtRef::Func(e) => visitor.visitFuncStmt(&e),
      StmtRef::Block(e) => visitor.visitBlockStmt(&e),
      StmtRef::Return(e) => visitor.visitReturnStmt(&e)
    }
  }
}
#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct VarStmt {
  location: Location,
  name: Token,
  initializer: Expr
}
#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct ExprStmt {
  location: Location,
  expression: Expr
}
#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct FuncStmt {
  location: Location,
  name: Token,
  body: Stmt,
  parameters: Vec<Expr>
}
#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct BlockStmt {
  location: Location,
  statements: Vec<Stmt>
}
#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct ReturnStmt {
  location: Location,
  expressions: Option<Expr>
}
#[derive(Serialize, Debug, Clone, PartialEq)]
pub enum Expr {
  Assign(AssignExpr)
  Call(CallExpr)
  Literal(LiteralExpr)
  Binary(BinaryExpr)
}
impl<'a> StmtRef<'a> {
  pub fn accept<R>(&self, visitor: &mut ExprVisitor<R>) -> R {
    match self {
      StmtRef::Assign(e) => visitor.visitAssignStmt(&e),
      StmtRef::Call(e) => visitor.visitCallStmt(&e),
      StmtRef::Literal(e) => visitor.visitLiteralStmt(&e),
      StmtRef::Binary(e) => visitor.visitBinaryStmt(&e)
    }
  }
}
#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct AssignExpr {
  location: Location,
  token: Token,
  value: Expr
}
#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct CallExpr {
  location: Location,
  callee: Expr,
  parent: Option<Expr>,
  arguments: Vec<Expr>
}
#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct LiteralExpr {
  location: Location,
  value: Literal
}
#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct BinaryExpr {
  location: Location,
  left: Expr,
  right: Expr,
  operator: Token
}
#[derive(Serialize, Debug, Clone, PartialEq)]
pub enum Stmt {
  Var(VarStmt)
  Expr(ExprStmt)
  Func(FuncStmt)
  Block(BlockStmt)
  Return(ReturnStmt)
}
impl<'a> StmtRef<'a> {
  pub fn accept<R>(&self, visitor: &mut StmtVisitor<R>) -> R {
    match self {
      StmtRef::Var(e) => visitor.visitVarStmt(&e),
      StmtRef::Expr(e) => visitor.visitExprStmt(&e),
      StmtRef::Func(e) => visitor.visitFuncStmt(&e),
      StmtRef::Block(e) => visitor.visitBlockStmt(&e),
      StmtRef::Return(e) => visitor.visitReturnStmt(&e)
    }
  }
}
#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct VarStmt {
  location: Location,
  name: Token,
  initializer: Expr
}
#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct ExprStmt {
  location: Location,
  expression: Expr
}
#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct FuncStmt {
  location: Location,
  name: Token,
  body: Stmt,
  parameters: Vec<Expr>
}
#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct BlockStmt {
  location: Location,
  statements: Vec<Stmt>
}
#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct ReturnStmt {
  location: Location,
  expressions: Option<Expr>
}
