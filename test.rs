
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

#[derive(Serialize, Debug, PartialEq, Eq, Hash, Clone)]
pub struct Token {
    pub location: Location,
    pub value: String,
}

impl Token {
    pub fn new<S:AsRef<str>>(location: Location, value: S) -> Token{
        TokenRef { location, value: value.as_ref().to_string() }
    }
}

impl<'a> fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}


#[derive(Serialize, Debug, Clone, PartialEq, Deserialize)]
#[serde(tag = "type", content = "value")]
pub enum Literal {
  String(String),
  Number(Number),
  Boolean(bool)
}
#[derive(Serialize, Debug, Clone, PartialEq, Deserialize)]
#[serde(tag = "type", content = "value")]
pub enum Number {
  Double(f64),
  Integer(i64)
}
#[derive(Serialize, Debug, Clone, PartialEq, Deserialize)]
#[serde(tag = "type", content = "value")]
pub enum LiteralRef<'a> {
  String(&'a str),
  Number(NumberRef<'a>),
  Boolean(bool)
}
#[derive(Serialize, Debug, Clone, PartialEq, Deserialize)]
#[serde(tag = "type", content = "value")]
pub enum NumberRef<'a> {
  Double(&'a str),
  Integer(&'a str)
}
pub trait ExprRefVisitor<R> {
  fn visitAssignExpr<'a>(expr: AssignExprExprRef<'a> );
  fn visitCallExpr<'a>(expr: CallExprExprRef<'a> );
  fn visitLiteralExpr<'a>(expr: LiteralExprExprRef<'a> );
  fn visitBinaryExpr<'a>(expr: BinaryExprExprRef<'a> );
  fn visitLiteralExpr<'a>(expr: LiteralExprExprRef<'a> );

pub trait StmtRefVisitor<R> {
  fn visitVarStmt<'a>(stmt: VarStmtStmtRef<'a> );
  fn visitExprStmt<'a>(stmt: ExprStmtStmtRef<'a> );
  fn visitFuncStmt<'a>(stmt: FuncStmtStmtRef<'a> );
  fn visitBlockStmt<'a>(stmt: BlockStmtStmtRef<'a> );
  fn visitIfStmt<'a>(stmt: IfStmtStmtRef<'a> );
  fn visitReturnStmt<'a>(stmt: ReturnStmtStmtRef<'a> );
  fn visitContinueStmt<'a>(stmt: ContinueStmtStmtRef<'a> );
  fn visitBreakStmt<'a>(stmt: BreakStmtStmtRef<'a> );

pub trait ExprVisitor<R> {
  fn visitAssignExpr(expr: AssignExprExpr );
  fn visitCallExpr(expr: CallExprExpr );
  fn visitLiteralExpr(expr: LiteralExprExpr );
  fn visitBinaryExpr(expr: BinaryExprExpr );
  fn visitLiteralExpr(expr: LiteralExprExpr );

pub trait StmtVisitor<R> {
  fn visitVarStmt(stmt: VarStmtStmt );
  fn visitExprStmt(stmt: ExprStmtStmt );
  fn visitFuncStmt(stmt: FuncStmtStmt );
  fn visitBlockStmt(stmt: BlockStmtStmt );
  fn visitIfStmt(stmt: IfStmtStmt );
  fn visitReturnStmt(stmt: ReturnStmtStmt );
  fn visitContinueStmt(stmt: ContinueStmtStmt );
  fn visitBreakStmt(stmt: BreakStmtStmt );

#[derive(Serialize, Debug, Clone, PartialEq)]
pub enum Expr {
  Assign(AssignExpr )
  Call(CallExpr )
  Literal(LiteralExpr )
  Binary(BinaryExpr )
  Literal(LiteralExpr )
}
impl Expr  {
  pub fn accept<R>(&self, visitor: &mut ExprVisitor<R>) -> R {
    match self {
      StmtRef::Assign(e) => visitor.visitAssignStmt(&e),
      StmtRef::Call(e) => visitor.visitCallStmt(&e),
      StmtRef::Literal(e) => visitor.visitLiteralStmt(&e),
      StmtRef::Binary(e) => visitor.visitBinaryStmt(&e),
      StmtRef::Literal(e) => visitor.visitLiteralStmt(&e)
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
pub struct LiteralExpr {
  location: Location,
  value: Literal
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub enum Stmt {
  Var(VarStmt )
  Expr(ExprStmt )
  Func(FuncStmt )
  Block(BlockStmt )
  If(IfStmt )
  Return(ReturnStmt )
  Continue(ContinueStmt )
  Break(BreakStmt )
}
impl Stmt  {
  pub fn accept<R>(&self, visitor: &mut StmtVisitor<R>) -> R {
    match self {
      StmtRef::Var(e) => visitor.visitVarStmt(&e),
      StmtRef::Expr(e) => visitor.visitExprStmt(&e),
      StmtRef::Func(e) => visitor.visitFuncStmt(&e),
      StmtRef::Block(e) => visitor.visitBlockStmt(&e),
      StmtRef::If(e) => visitor.visitIfStmt(&e),
      StmtRef::Return(e) => visitor.visitReturnStmt(&e),
      StmtRef::Continue(e) => visitor.visitContinueStmt(&e),
      StmtRef::Break(e) => visitor.visitBreakStmt(&e)
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
pub struct IfStmt {
  location: Location,
  test: Expr,
  consequent: Stmt,
  alternative: Option<Expr>
}
#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct ReturnStmt {
  location: Location,
  expressions: Option<Expr>
}
#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct ContinueStmt {
  location: Location
}
#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct BreakStmt {
  location: Location
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub enum Expr {
  Assign(AssignExpr )
  Call(CallExpr )
  Literal(LiteralExpr )
  Binary(BinaryExpr )
  Literal(LiteralExpr )
}
impl Expr  {
  pub fn accept<R>(&self, visitor: &mut ExprVisitor<R>) -> R {
    match self {
      StmtRef::Assign(e) => visitor.visitAssignStmt(&e),
      StmtRef::Call(e) => visitor.visitCallStmt(&e),
      StmtRef::Literal(e) => visitor.visitLiteralStmt(&e),
      StmtRef::Binary(e) => visitor.visitBinaryStmt(&e),
      StmtRef::Literal(e) => visitor.visitLiteralStmt(&e)
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
pub struct LiteralExpr {
  location: Location,
  value: Literal
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub enum Stmt {
  Var(VarStmt )
  Expr(ExprStmt )
  Func(FuncStmt )
  Block(BlockStmt )
  If(IfStmt )
  Return(ReturnStmt )
  Continue(ContinueStmt )
  Break(BreakStmt )
}
impl Stmt  {
  pub fn accept<R>(&self, visitor: &mut StmtVisitor<R>) -> R {
    match self {
      StmtRef::Var(e) => visitor.visitVarStmt(&e),
      StmtRef::Expr(e) => visitor.visitExprStmt(&e),
      StmtRef::Func(e) => visitor.visitFuncStmt(&e),
      StmtRef::Block(e) => visitor.visitBlockStmt(&e),
      StmtRef::If(e) => visitor.visitIfStmt(&e),
      StmtRef::Return(e) => visitor.visitReturnStmt(&e),
      StmtRef::Continue(e) => visitor.visitContinueStmt(&e),
      StmtRef::Break(e) => visitor.visitBreakStmt(&e)
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
pub struct IfStmt {
  location: Location,
  test: Expr,
  consequent: Stmt,
  alternative: Option<Expr>
}
#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct ReturnStmt {
  location: Location,
  expressions: Option<Expr>
}
#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct ContinueStmt {
  location: Location
}
#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct BreakStmt {
  location: Location
}
