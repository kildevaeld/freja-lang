use super::ast::*;
use super::traits::{ExprRefVisitor, ExprVisitor, StmtRefVisitor, StmtVisitor};
use std::borrow::Borrow;
use std::fmt;
use std::ops::Deref;

#[derive(Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Token {
    pub location: Location,
    pub value: String,
}

impl Token {
    pub fn new(location: Location, value: String) -> Token {
        Token { location, value }
    }
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct Program {
    pub statements: Vec<Stmt>,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Serialize, Debug, Clone, PartialEq, Deserialize)]
#[serde(tag = "type", content = "value")]
pub enum Literal {
    String(String),
    Number(Number),
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Literal::String(s) => write!(f, "{}", s),
            Literal::Number(n) => <fmt::Display>::fmt(n, f),
        }
    }
}

#[derive(Serialize, Debug, Clone, PartialEq, Deserialize)]
#[serde(tag = "type", content = "value")]
pub enum Number {
    Integer(String),
    Float(String),
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Number::Integer(s) => s,
            Number::Float(n) => n,
        };
        write!(f, "{}", s)
    }
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub enum TokenType {}

#[derive(Serialize, Debug, Clone, PartialEq)]
#[serde(tag = "type")]
pub enum Expr {
    Assign(AssignExpr),
    Binary(BinaryExpr),
    Call(CallExpr),
    Literal(LiteralExpr),
    Identifier(IdentifierExpr),
}

impl Expr {
    pub fn accept<R>(self, visitor: &mut ExprVisitor<R>) -> R {
        match self {
            Expr::Assign(a) => visitor.visitAssignExpr(a),
            Expr::Identifier(i) => visitor.visitIdentifierExpr(i),
            Expr::Call(c) => visitor.visitCallExpr(c),
            Expr::Literal(l) => visitor.visitLiteralExpr(l),
            _ => unimplemented!("{:?}", self),
        }
    }
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct AssignExpr {
    pub location: Location,
    pub token: Token,
    pub value: Box<Expr>,
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct IdentifierExpr {
    pub location: Location,
    pub token: Token,
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct BinaryExpr {
    pub location: Location,
    pub left: Box<Expr>,
    pub right: Box<Expr>,
    pub operator: Token,
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct CallExpr {
    pub location: Location,
    pub callee: Box<Expr>,
    pub parent: Option<Box<Expr>>,
    pub arguments: Vec<Box<Expr>>,
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct LiteralExpr {
    pub location: Location,
    pub value: Literal,
}

#[derive(Serialize, Debug, Clone, PartialEq)]
#[serde(tag = "type")]
pub enum Stmt {
    Var(VarStmt),
    Expr(ExprStmt),
    Func(FuncStmt),
    Block(BlockStmt),
    Return(ReturnStmt),
}

impl Stmt {
    pub fn accept<R>(self, visitor: &mut StmtVisitor<R>) -> R {
        match self {
            Stmt::Var(a) => visitor.visitVarStmt(a),
            Stmt::Expr(i) => visitor.visitExprStmt(i),
            Stmt::Func(c) => visitor.visitFuncStmt(c),
            Stmt::Block(l) => visitor.visitBlockStmt(l),
            Stmt::Return(r) => visitor.visitReturnStmt(r),
        }
    }
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct VarStmt {
    pub name: Token,
    pub initializer: Expr,
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct BlockStmt {
    pub statements: Vec<Stmt>,
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct FuncStmt {
    pub name: Token,
    pub body: Box<Stmt>,
    pub parameters: Vec<Expr>,
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct ReturnStmt {
    pub expression: Option<Expr>,
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct ExprStmt {
    pub expression: Expr,
}

pub struct OwnedVisitor;

impl StmtRefVisitor<Stmt> for OwnedVisitor {
    fn visitVarStmt(&mut self, stmt: &VarStmtRef) -> Stmt {
        Stmt::Var(VarStmt {
            name: Token::new(stmt.name.location.clone(), stmt.name.value.to_owned()),
            initializer: stmt.initializer.accept(self),
        })
    }
    fn visitExprStmt(&mut self, stmt: &ExprStmtRef) -> Stmt {
        Stmt::Expr(ExprStmt {
            expression: stmt.expression.accept(self),
        })
    }
    fn visitFuncStmt(&mut self, stmt: &FuncStmtRef) -> Stmt {
        Stmt::Func(FuncStmt {
            name: Token::new(stmt.name.location.clone(), stmt.name.value.to_owned()),
            parameters: stmt.parameters.iter().map(|m| m.accept(self)).collect(),
            body: Box::new(stmt.body.accept(self)),
        })
    }
    fn visitBlockStmt(&mut self, stmt: &BlockStmtRef) -> Stmt {
        Stmt::Block(BlockStmt {
            statements: stmt.statements.iter().map(|m| m.accept(self)).collect(),
        })
    }
    fn visitReturnStmt(&mut self, stmt: &ReturnStmtRef) -> Stmt {
        match &stmt.expression {
            Some(m) => Stmt::Return(ReturnStmt {
                expression: Some(m.accept(self)),
            }),
            None => Stmt::Return(ReturnStmt { expression: None }),
        }
    }
}

impl ExprRefVisitor<Expr> for OwnedVisitor {
    fn visitAssignExpr(&mut self, expr: &AssignExprRef) -> Expr {
        Expr::Assign(AssignExpr {
            location: expr.location.clone(),
            token: Token::new(expr.token.location.clone(), expr.token.value.to_owned()),
            value: Box::new(expr.value.accept(self)),
        })
    }
    fn visitIdentifierExpr(&mut self, expr: &IdentifierExprRef) -> Expr {
        Expr::Identifier(IdentifierExpr {
            location: expr.location.clone(),
            token: Token::new(expr.token.location.clone(), expr.token.value.to_owned()),
        })
    }
    fn visitCallExpr(&mut self, expr: &CallExprRef) -> Expr {
        match &expr.parent {
            Some(p) => Expr::Call(CallExpr {
                location: expr.location.clone(),
                parent: Some(Box::new(p.accept(self))),
                callee: Box::new(expr.callee.accept(self)),
                arguments: expr
                    .arguments
                    .iter()
                    .map(|m| Box::new(m.accept(self)))
                    .collect(),
            }),
            None => Expr::Call(CallExpr {
                location: expr.location.clone(),
                parent: None,
                callee: Box::new(expr.callee.accept(self)),
                arguments: expr
                    .arguments
                    .iter()
                    .map(|m| Box::new(m.accept(self)))
                    .collect(),
            }),
        }
    }
    fn visitLiteralExpr(&mut self, expr: &LiteralExprRef) -> Expr {
        Expr::Literal(LiteralExpr {
            location: expr.location.clone(),
            value: match &expr.value {
                LiteralRef::String(s) => Literal::String(s.to_string()),
                r => unimplemented!("{:?}", r),
            },
        })
    }
}
