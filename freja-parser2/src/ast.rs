use std::fmt;

#[derive(Serialize, Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
pub struct Location(pub usize, pub usize);

#[derive(Serialize, Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
pub enum TokenType {
    This, Identifier,
    OpAdditive, OpMultiplicative
}

#[derive(Serialize, Debug, PartialEq, Eq, Hash, Clone)]
#[serde(tag = "type")]
pub struct Token<'a> {
    pub kind: TokenType,
    pub location: Location,
    pub value: &'a str,
}

impl<'a> Token<'a> {
    pub fn new(location: Location, kind: TokenType, value: &'a str) -> Token<'a> {
        Token { location, value, kind }
    }
}

impl<'a> fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

pub trait StmtVisitor<R> {
    fn visit_program_stmt(&mut self, e:&ProgramStmt) -> R;
    fn visit_var_stmt(&mut self, e:&VarStmt) -> R;
    fn visit_varlist_stmt(&mut self, e:&VarListStmt) -> R;
    fn visit_expr_stmt(&mut self, e:&ExprStmt) -> R;
    fn visit_func_stmt(&mut self, e:&FuncStmt) -> R;
    fn visit_class_stmt(&mut self, e:&ClassStmt) -> R;
    fn visit_block_stmt(&mut self, e:&BlockStmt) -> R;
    fn visit_if_stmt(&mut self, e:&IfStmt) -> R;
    fn visit_for_stmt(&mut self, e:&ForStmt) -> R;
    fn visit_return_stmt(&mut self, e:&ReturnStmt) -> R;
    fn visit_continue_stmt(&mut self, e:&ContinueStmt) -> R;
    fn visit_break_stmt(&mut self, e:&BreakStmt) -> R;
}

pub trait ExprVisitor<R> {
    fn visit_assign_expr(&mut self, e:&AssignExpr) -> R;
    fn visit_call_expr(&mut self, e:&CallExpr) -> R;
    fn visit_literal_expr(&mut self, e:&LiteralExpr) -> R;
    fn visit_binary_expr(&mut self, e:&BinaryExpr) -> R;
    fn visit_member_expr(&mut self, e:&MemberExpr) -> R;
    fn visit_lookup_expr(&mut self, e:&LookupExpr) -> R;
    fn visit_arguments_expr(&mut self, e:&ArgumentsExpr) -> R;
}


#[derive(Serialize, Debug, Clone, PartialEq)]
#[serde(tag = "type", content = "value")]
pub enum Literal<'a> {
    String(&'a str),
    Number(Number<'a>),
    Boolean(bool),
}

#[derive(Serialize, Debug, Clone, PartialEq)]
#[serde(tag = "type", content = "value")]
pub enum Number<'a> {
    Double(&'a str),
    Integer(&'a str),
}

#[derive(Serialize, Debug, Clone, PartialEq)]
#[serde(tag = "type", content = "value")]
pub enum Argument<'a> {
    Regular(&'a str),
    Rest(&'a str),
}

#[derive(Serialize, Debug, Clone, PartialEq)]
#[serde(tag = "type")]
pub enum Stmt<'a> {
    Program(ProgramStmt<'a>),
    Var(VarStmt<'a>),
    VarList(VarListStmt<'a>),
    Expr(ExprStmt<'a>),
    Func(FuncStmt<'a>),
    Class(ClassStmt<'a>),
    Block(BlockStmt<'a>),
    If(IfStmt<'a>),
    For(ForStmt<'a>),
    Return(ReturnStmt<'a>),
    Continue(ContinueStmt),
    Break(BreakStmt),
}

impl<'a> Stmt<'a> {
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
pub enum Expr<'a> {
    Assign(AssignExpr<'a>),
    Call(CallExpr<'a>),
    Literal(LiteralExpr<'a>),
    Binary(BinaryExpr<'a>),
    Member(MemberExpr<'a>),
    Lookup(LookupExpr<'a>),
    Arguments(ArgumentsExpr<'a>),
}

impl<'a> Expr<'a> {
    pub fn accept<R>(&self, visitor: &mut ExprVisitor<R>) -> R {
        match self {
            Expr::Assign(s) => visitor.visit_assign_expr(&s),
            Expr::Call(s) => visitor.visit_call_expr(&s),
            Expr::Literal(s) => visitor.visit_literal_expr(&s),
            Expr::Binary(s) => visitor.visit_binary_expr(&s),
            Expr::Member(s) => visitor.visit_member_expr(&s),
            Expr::Lookup(s) => visitor.visit_lookup_expr(&s),
            Expr::Arguments(s) => visitor.visit_arguments_expr(&s),
        }
    }
}


#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct ProgramStmt<'a> {
    pub location: Location,
    pub statements: Vec<Box<Stmt<'a>>>,
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct VarStmt<'a> {
    pub location: Location,
    pub name: Token<'a>,
    pub initializer: Option<Expr<'a>>,
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct VarListStmt<'a> {
    pub location: Location,
    pub variables: Vec<VarStmt<'a>>,
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct ExprStmt<'a> {
    pub location: Location,
    pub expression: Expr<'a>,
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct FuncStmt<'a> {
    pub location: Location,
    pub name: Token<'a>,
    pub body: Box<Stmt<'a>>,
    pub parameters: Vec<Argument<'a>>,
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct ClassStmt<'a> {
    pub location: Location,
    pub name: Token<'a>,
    pub members: Vec<Box<Stmt<'a>>>,
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct BlockStmt<'a> {
    pub location: Location,
    pub statements: Vec<Box<Stmt<'a>>>,
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct IfStmt<'a> {
    pub location: Location,
    pub test: Expr<'a>,
    pub consequent: Box<Stmt<'a>>,
    pub alternative: Option<Expr<'a>>,
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct ForStmt<'a> {
    pub location: Location,
    pub element: Token<'a>,
    pub index: Option<Expr<'a>>,
    pub body: Box<Stmt<'a>>,
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct ReturnStmt<'a> {
    pub location: Location,
    pub expressions: Option<Expr<'a>>,
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
pub struct AssignExpr<'a> {
    pub location: Location,
    pub token: Token<'a>,
    pub value: Box<Expr<'a>>,
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct CallExpr<'a> {
    pub location: Location,
    pub member: Box<Expr<'a>>,
    pub arguments: Box<Expr<'a>>,
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct LiteralExpr<'a> {
    pub location: Location,
    pub value: Literal<'a>,
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct BinaryExpr<'a> {
    pub location: Location,
    pub left: Box<Expr<'a>>,
    pub right: Box<Expr<'a>>,
    pub operator: Token<'a>,
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct MemberExpr<'a> {
    pub location: Location,
    pub object: Box<Expr<'a>>,
    pub property: Box<Expr<'a>>,
    pub computed: bool,
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct LookupExpr<'a> {
    pub location: Location,
    pub token: Token<'a>,
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct ArgumentsExpr<'a> {
    pub location: Location,
    pub expressions: Vec<Box<Expr<'a>>>,
}

