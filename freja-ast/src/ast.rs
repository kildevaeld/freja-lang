use std::fmt;

#[derive(Serialize, Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
pub struct Location(pub usize, pub usize);

#[derive(Serialize, Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
pub enum TokenType {
    This,
    Identifier,
    OpAdditive,
    OpMultiplicative,
    ShiftOperator,
    EqualityOperator,
    RelationalOperator,
    BitwiseOrOperator,
    BitwiseAndOperator,
    LogicalOrOperator,
    LogicalAndOperator,
}

#[derive(Serialize, Debug, PartialEq, Eq, Hash, Clone, Deserialize)]
#[serde(tag = "type")]
pub struct Token {
    pub kind: TokenType,
    pub location: Location,
    pub value: String,
}

impl Token {
    pub fn new(location: Location, kind: TokenType, value: &str) -> Token {
        Token {
            location,
            value: value.to_owned(),
            kind,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Serialize, Debug, Clone, PartialEq, PartialOrd)]
#[serde(tag = "type", content = "value")]
pub enum Number {
    Double(f64),
    Integer(i64),
}

pub trait StmtVisitor<R> {
    fn visit_program_stmt(&mut self, e: &ProgramStmt) -> R;
    fn visit_var_stmt(&mut self, e: &VarStmt) -> R;
    fn visit_varlist_stmt(&mut self, e: &VarListStmt) -> R;
    fn visit_expr_stmt(&mut self, e: &ExprStmt) -> R;
    fn visit_func_stmt(&mut self, e: &FuncStmt) -> R;
    fn visit_class_stmt(&mut self, e: &ClassStmt) -> R;
    fn visit_block_stmt(&mut self, e: &BlockStmt) -> R;
    fn visit_if_stmt(&mut self, e: &IfStmt) -> R;
    fn visit_for_stmt(&mut self, e: &ForStmt) -> R;
    fn visit_return_stmt(&mut self, e: &ReturnStmt) -> R;
    fn visit_continue_stmt(&mut self, e: &ContinueStmt) -> R;
    fn visit_break_stmt(&mut self, e: &BreakStmt) -> R;
}

pub trait ExprVisitor<R> {
    fn visit_assign_expr(&mut self, e: &AssignExpr) -> R;
    fn visit_call_expr(&mut self, e: &CallExpr) -> R;
    fn visit_literal_expr(&mut self, e: &LiteralExpr) -> R;
    fn visit_binary_expr(&mut self, e: &BinaryExpr) -> R;
    fn visit_member_expr(&mut self, e: &MemberExpr) -> R;
    fn visit_lookup_expr(&mut self, e: &LookupExpr) -> R;
    fn visit_arguments_expr(&mut self, e: &ArgumentsExpr) -> R;
    fn visit_logical_expr(&mut self, e: &LogicalExpr) -> R;
}

#[derive(Serialize, Debug, Clone, PartialEq)]
#[serde(tag = "type", content = "value")]
pub enum Literal {
    String(String),
    Number(Number),
    Boolean(bool),
}

#[derive(Serialize, Debug, Clone, PartialEq)]
#[serde(tag = "type", content = "value")]
pub enum Argument {
    Regular(String),
    Rest(String),
}

#[derive(Serialize, Debug, Clone, PartialEq)]
#[serde(tag = "type")]
pub enum Stmt {
    Program(ProgramStmt),
    Var(VarStmt),
    VarList(VarListStmt),
    Expr(ExprStmt),
    Func(FuncStmt),
    Class(ClassStmt),
    Block(BlockStmt),
    If(IfStmt),
    For(ForStmt),
    Return(ReturnStmt),
    Continue(ContinueStmt),
    Break(BreakStmt),
}

impl Stmt {
    pub fn accept<R>(&self, visitor: &mut StmtVisitor<R>) -> R {
        match self {
            Stmt::Program(s) => visitor.visit_program_stmt(&s),
            Stmt::Var(s) => visitor.visit_var_stmt(&s),
            Stmt::VarList(s) => visitor.visit_varlist_stmt(&s),
            Stmt::Expr(s) => visitor.visit_expr_stmt(&s),
            Stmt::Func(s) => visitor.visit_func_stmt(&s),
            Stmt::Class(s) => visitor.visit_class_stmt(&s),
            Stmt::Block(s) => visitor.visit_block_stmt(&s),
            Stmt::If(s) => visitor.visit_if_stmt(&s),
            Stmt::For(s) => visitor.visit_for_stmt(&s),
            Stmt::Return(s) => visitor.visit_return_stmt(&s),
            Stmt::Continue(s) => visitor.visit_continue_stmt(&s),
            Stmt::Break(s) => visitor.visit_break_stmt(&s),
        }
    }
}

#[derive(Serialize, Debug, Clone, PartialEq)]
#[serde(tag = "type")]
pub enum Expr {
    Assign(AssignExpr),
    Call(CallExpr),
    Literal(LiteralExpr),
    Binary(BinaryExpr),
    Member(MemberExpr),
    Lookup(LookupExpr),
    Arguments(ArgumentsExpr),
    Logical(LogicalExpr),
}

impl Expr {
    pub fn accept<R>(&self, visitor: &mut ExprVisitor<R>) -> R {
        match self {
            Expr::Assign(s) => visitor.visit_assign_expr(&s),
            Expr::Call(s) => visitor.visit_call_expr(&s),
            Expr::Literal(s) => visitor.visit_literal_expr(&s),
            Expr::Binary(s) => visitor.visit_binary_expr(&s),
            Expr::Member(s) => visitor.visit_member_expr(&s),
            Expr::Lookup(s) => visitor.visit_lookup_expr(&s),
            Expr::Arguments(s) => visitor.visit_arguments_expr(&s),
            Expr::Logical(s) => visitor.visit_logical_expr(&s),
        }
    }
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct ProgramStmt {
    pub location: Location,
    pub statements: Vec<Box<Stmt>>,
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct VarStmt {
    pub location: Location,
    pub name: Token,
    pub initializer: Option<Expr>,
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct VarListStmt {
    pub location: Location,
    pub variables: Vec<VarStmt>,
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct ExprStmt {
    pub location: Location,
    pub expression: Expr,
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct FuncStmt {
    pub location: Location,
    pub name: Token,
    pub body: Box<Stmt>,
    pub parameters: Vec<Argument>,
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct ClassStmt {
    pub location: Location,
    pub name: Token,
    pub members: Vec<Box<Stmt>>,
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct BlockStmt {
    pub location: Location,
    pub statements: Vec<Box<Stmt>>,
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct IfStmt {
    pub location: Location,
    pub test: Expr,
    pub consequent: Box<Stmt>,
    pub alternative: Option<Box<Stmt>>,
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct ForStmt {
    pub location: Location,
    pub element: Token,
    pub index: Option<Token>,
    pub iterator: Expr,
    pub body: Box<Stmt>,
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct ReturnStmt {
    pub location: Location,
    pub expression: Option<Expr>,
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct ContinueStmt {
    pub location: Location,
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct BreakStmt {
    pub location: Location,
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct AssignExpr {
    pub location: Location,
    pub destination: Box<Expr>,
    pub value: Box<Expr>,
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct CallExpr {
    pub location: Location,
    pub member: Box<Expr>,
    pub arguments: Box<Expr>,
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct LiteralExpr {
    pub location: Location,
    pub value: Literal,
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct BinaryExpr {
    pub location: Location,
    pub left: Box<Expr>,
    pub right: Box<Expr>,
    pub operator: Token,
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct MemberExpr {
    pub location: Location,
    pub object: Box<Expr>,
    pub property: Box<Expr>,
    pub computed: bool,
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct LookupExpr {
    pub location: Location,
    pub token: Token,
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct ArgumentsExpr {
    pub location: Location,
    pub expressions: Vec<Box<Expr>>,
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct LogicalExpr {
    pub location: Location,
    pub left: Box<Expr>,
    pub right: Box<Expr>,
    pub operator: Token,
}
