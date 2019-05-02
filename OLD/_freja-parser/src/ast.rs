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

// impl<'a> ToOwned for TokenRef<'a> {
//     type Owned = Token;
//     fn to_owned(&self) -> Self::Owned {
//         Token::new(self.location.clone(), self.value.to_string())
//     }
// }

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct ProgramRef<'a> {
    pub statements: Vec<StmtRef<'a>>,
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
pub enum TokenType {}

#[derive(Serialize, Debug, Clone, PartialEq)]
#[serde(tag = "type")]
pub enum ExprRef<'a> {
    Assign(AssignExprRef<'a>),
    Binary(BinaryExprRef<'a>),
    Call(CallExprRef<'a>),
    Literal(LiteralExprRef<'a>),
    Identifier(IdentifierExprRef<'a>),
}

impl<'a> ExprRef<'a> {
    pub fn accept<R>(&self, visitor: &mut ExprRefVisitor<R>) -> R {
        match self {
            ExprRef::Assign(a) => visitor.visitAssignExpr(&a),
            ExprRef::Identifier(i) => visitor.visitIdentifierExpr(&i),
            ExprRef::Call(c) => visitor.visitCallExpr(&c),
            ExprRef::Literal(l) => visitor.visitLiteralExpr(l),
            _ => unimplemented!("{:?}", self),
        }
    }
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct AssignExprRef<'a> {
    pub location: Location,
    pub token: TokenRef<'a>,
    pub value: Box<ExprRef<'a>>,
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct IdentifierExprRef<'a> {
    pub location: Location,
    pub token: TokenRef<'a>,
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct BinaryExprRef<'a> {
    pub location: Location,
    pub left: Box<ExprRef<'a>>,
    pub right: Box<ExprRef<'a>>,
    pub operator: TokenRef<'a>,
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct CallExprRef<'a> {
    pub location: Location,
    pub callee: Box<ExprRef<'a>>,
    pub parent: Option<Box<ExprRef<'a>>>,
    pub arguments: Vec<Box<ExprRef<'a>>>,
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct LiteralExprRef<'a> {
    pub location: Location,
    pub value: LiteralRef<'a>,
}

#[derive(Serialize, Debug, Clone, PartialEq)]
#[serde(tag = "type")]
pub enum StmtRef<'a> {
    Var(VarStmtRef<'a>),
    Expr(ExprStmtRef<'a>),
    Func(FuncStmtRef<'a>),
    Block(BlockStmtRef<'a>),
    Return(ReturnStmtRef<'a>),
}

impl<'a> StmtRef<'a> {
    pub fn accept<R>(&self, visitor: &mut StmtRefVisitor<R>) -> R {
        match self {
            StmtRef::Var(a) => visitor.visitVarStmt(&a),
            StmtRef::Expr(i) => visitor.visitExprStmt(&i),
            StmtRef::Func(c) => visitor.visitFuncStmt(&c),
            StmtRef::Block(l) => visitor.visitBlockStmt(l),
            StmtRef::Return(r) => visitor.visitReturnStmt(r),
        }
    }
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct VarStmtRef<'a> {
    pub name: TokenRef<'a>,
    pub initializer: ExprRef<'a>,
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct BlockStmtRef<'a> {
    pub statements: Vec<StmtRef<'a>>,
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct FuncStmtRef<'a> {
    pub name: TokenRef<'a>,
    pub body: Box<StmtRef<'a>>,
    pub parameters: Vec<ExprRef<'a>>,
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct ReturnStmtRef<'a> {
    pub expression: Option<ExprRef<'a>>,
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct ExprStmtRef<'a> {
    pub expression: ExprRef<'a>,
}
