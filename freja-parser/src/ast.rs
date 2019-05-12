/*! WARNING: AUTO GENERATED  - DO NOT EDIT **/

use std::fmt;

#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Location(pub usize, pub usize);

#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
#[cfg_attr(feature = "serde_support", serde(tag = "type"))]
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

#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
#[derive(Debug, PartialEq, Clone)]
pub struct Object {
    entries: Vec<ObjectEntry>,
}

impl Object {
    pub fn new(entries: Vec<ObjectEntry>) -> Object {
        Object { entries }
    }

    pub fn entries(&self) -> &[ObjectEntry] {
        &self.entries
    }
}

#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
#[derive(Debug, PartialEq, Clone)]
pub struct ObjectEntry {
    pub key: Expr,
    pub value: Expr,
}

impl ObjectEntry {
    pub fn new(key: Expr, value: Expr) -> ObjectEntry {
        ObjectEntry { key, value }
    }
}

#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
// #[serde(tag = "type", content = "value")]
#[cfg_attr(feature = "serde_support", serde(tag = "type", content = "value"))]
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
    fn visit_interface_stmt(&mut self, e: &InterfaceStmt) -> R;
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
    fn visit_this_expr(&mut self, e: &ThisExpr) -> R;
    fn visit_var_expr(&mut self, e: &VarExpr) -> R;
    fn visit_identifier_expr(&mut self, e: &IdentifierExpr) -> R;
    fn visit_unary_expr(&mut self, e: &UnaryExpr) -> R;
    fn visit_postfix_expr(&mut self, e: &PostfixExpr) -> R;
    fn visit_closure_expr(&mut self, e: &ClosureExpr) -> R;
}

#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
// #[serde(tag = "type", content = "value")]
#[cfg_attr(feature = "serde_support", serde(tag = "type", content = "value"))]
pub enum UnaryOperator {
    Plus,
    Minus,
    Increment,
    Decrement,
    Not,
}

#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
// #[serde(tag = "type", content = "value")]
#[cfg_attr(feature = "serde_support", serde(tag = "type", content = "value"))]
pub enum PostfixOperator {
    Increment,
    Decrement,
}

#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
// #[serde(tag = "type", content = "value")]
#[cfg_attr(feature = "serde_support", serde(tag = "type", content = "value"))]
pub enum BinaryOperator {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    BitwiseXor,
    BitwiseAnd,
    BitwiseOr,
    ShiftLeft,
    ShiftRight,
    Eq,
    Neq,
    Lt,
    Lte,
    Gt,
    Gte,
    Is,
}

#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
// #[serde(tag = "type", content = "value")]
#[cfg_attr(feature = "serde_support", serde(tag = "type", content = "value"))]
pub enum ComparisonOperator {
    Eq,
    Neq,
    Lt,
    Lte,
    Gt,
    Gte,
}

#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
// #[serde(tag = "type", content = "value")]
#[cfg_attr(feature = "serde_support", serde(tag = "type", content = "value"))]
pub enum LogicalOperator {
    And,
    Or,
}

#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
// #[serde(tag = "type", content = "value")]
#[cfg_attr(feature = "serde_support", serde(tag = "type", content = "value"))]
pub enum AssignmentOperator {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    ShiftLeft,
    ShiftRight,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    Assign,
}

#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
// #[serde(tag = "type", content = "value")]
#[cfg_attr(feature = "serde_support", serde(tag = "type", content = "value"))]
pub enum Literal {
    String(String),
    Number(Number),
    Boolean(bool),
    Array(Vec<Expr>),
    Object(Object),
    Null,
}

#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
// #[serde(tag = "type", content = "value")]
#[cfg_attr(feature = "serde_support", serde(tag = "type", content = "value"))]
pub enum Argument {
    Regular(String),
    Rest(String),
}

#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde_support", serde(tag = "type"))]
pub enum Stmt {
    Program(ProgramStmt),
    Var(VarStmt),
    VarList(VarListStmt),
    Expr(ExprStmt),
    Func(FuncStmt),
    Class(ClassStmt),
    Interface(InterfaceStmt),
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
            Stmt::Interface(s) => visitor.visit_interface_stmt(&s),
            Stmt::Block(s) => visitor.visit_block_stmt(&s),
            Stmt::If(s) => visitor.visit_if_stmt(&s),
            Stmt::For(s) => visitor.visit_for_stmt(&s),
            Stmt::Return(s) => visitor.visit_return_stmt(&s),
            Stmt::Continue(s) => visitor.visit_continue_stmt(&s),
            Stmt::Break(s) => visitor.visit_break_stmt(&s),
        }
    }
}

#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde_support", serde(tag = "type"))]
pub enum Expr {
    Assign(AssignExpr),
    Call(CallExpr),
    Literal(LiteralExpr),
    Binary(BinaryExpr),
    Member(MemberExpr),
    Lookup(LookupExpr),
    Arguments(ArgumentsExpr),
    Logical(LogicalExpr),
    This(ThisExpr),
    Var(VarExpr),
    Identifier(IdentifierExpr),
    Unary(UnaryExpr),
    Postfix(PostfixExpr),
    Closure(ClosureExpr),
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
            Expr::This(s) => visitor.visit_this_expr(&s),
            Expr::Var(s) => visitor.visit_var_expr(&s),
            Expr::Identifier(s) => visitor.visit_identifier_expr(&s),
            Expr::Unary(s) => visitor.visit_unary_expr(&s),
            Expr::Postfix(s) => visitor.visit_postfix_expr(&s),
            Expr::Closure(s) => visitor.visit_closure_expr(&s),
        }
    }
}

#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct ProgramStmt {
    pub statements: Vec<Box<Stmt>>,
}

impl ProgramStmt {
    pub fn new(statements: Vec<Box<Stmt>>) -> ProgramStmt {
        ProgramStmt { statements }
    }
}

#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct VarStmt {
    pub name: String,
    pub initializer: Option<Expr>,
}

impl VarStmt {
    pub fn new(name: String, initializer: Option<Expr>) -> VarStmt {
        VarStmt { name, initializer }
    }
}

#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct VarListStmt {
    pub variables: Vec<VarStmt>,
}

impl VarListStmt {
    pub fn new(variables: Vec<VarStmt>) -> VarListStmt {
        VarListStmt { variables }
    }
}

#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct ExprStmt {
    pub expression: Expr,
}

impl ExprStmt {
    pub fn new(expression: Expr) -> ExprStmt {
        ExprStmt { expression }
    }
}

#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct FuncStmt {
    pub name: String,
    pub body: Box<Stmt>,
    pub parameters: Vec<Argument>,
}

impl FuncStmt {
    pub fn new(name: String, body: Box<Stmt>, parameters: Vec<Argument>) -> FuncStmt {
        FuncStmt { name, body, parameters }
    }
}

#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct ClassStmt {
    pub name: String,
    pub members: Vec<Box<Stmt>>,
    pub extends: Option<String>,
    pub implements: Vec<String>,
}

impl ClassStmt {
    pub fn new(name: String, members: Vec<Box<Stmt>>, extends: Option<String>, implements: Vec<String>) -> ClassStmt {
        ClassStmt {
            name,
            members,
            extends,
            implements,
        }
    }
}

#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct InterfaceStmt {
    pub name: String,
    pub extends: Option<String>,
    pub members: Vec<Box<Stmt>>,
}

impl InterfaceStmt {
    pub fn new(name: String, extends: Option<String>, members: Vec<Box<Stmt>>) -> InterfaceStmt {
        InterfaceStmt { name, extends, members }
    }
}

#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct BlockStmt {
    pub statements: Vec<Box<Stmt>>,
}

impl BlockStmt {
    pub fn new(statements: Vec<Box<Stmt>>) -> BlockStmt {
        BlockStmt { statements }
    }
}

#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct IfStmt {
    pub test: Expr,
    pub consequent: Box<Stmt>,
    pub alternative: Option<Box<Stmt>>,
}

impl IfStmt {
    pub fn new(test: Expr, consequent: Box<Stmt>, alternative: Option<Box<Stmt>>) -> IfStmt {
        IfStmt {
            test,
            consequent,
            alternative,
        }
    }
}

#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct ForStmt {
    pub initializer: Option<Box<Stmt>>,
    pub condition: Option<Expr>,
    pub increment: Option<Expr>,
    pub body: Box<Stmt>,
}

impl ForStmt {
    pub fn new(
        initializer: Option<Box<Stmt>>,
        condition: Option<Expr>,
        increment: Option<Expr>,
        body: Box<Stmt>,
    ) -> ForStmt {
        ForStmt {
            initializer,
            condition,
            increment,
            body,
        }
    }
}

#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct ReturnStmt {
    pub expression: Option<Expr>,
}

impl ReturnStmt {
    pub fn new(expression: Option<Expr>) -> ReturnStmt {
        ReturnStmt { expression }
    }
}

#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct ContinueStmt {}

impl ContinueStmt {
    pub fn new() -> ContinueStmt {
        ContinueStmt {}
    }
}

#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct BreakStmt {}

impl BreakStmt {
    pub fn new() -> BreakStmt {
        BreakStmt {}
    }
}

#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct AssignExpr {
    pub destination: Box<Expr>,
    pub value: Box<Expr>,
    pub operator: AssignmentOperator,
}

impl AssignExpr {
    pub fn new(destination: Box<Expr>, value: Box<Expr>, operator: AssignmentOperator) -> AssignExpr {
        AssignExpr {
            destination,
            value,
            operator,
        }
    }
}

#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct CallExpr {
    pub member: Box<Expr>,
    pub arguments: Vec<Expr>,
}

impl CallExpr {
    pub fn new(member: Box<Expr>, arguments: Vec<Expr>) -> CallExpr {
        CallExpr { member, arguments }
    }
}

#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct LiteralExpr {
    pub value: Literal,
}

impl LiteralExpr {
    pub fn new(value: Literal) -> LiteralExpr {
        LiteralExpr { value }
    }
}

#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct BinaryExpr {
    pub left: Box<Expr>,
    pub right: Box<Expr>,
    pub operator: BinaryOperator,
}

impl BinaryExpr {
    pub fn new(left: Box<Expr>, right: Box<Expr>, operator: BinaryOperator) -> BinaryExpr {
        BinaryExpr { left, right, operator }
    }
}

#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct MemberExpr {
    pub object: Box<Expr>,
    pub property: Box<Expr>,
    pub computed: bool,
}

impl MemberExpr {
    pub fn new(object: Box<Expr>, property: Box<Expr>, computed: bool) -> MemberExpr {
        MemberExpr {
            object,
            property,
            computed,
        }
    }
}

#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct LookupExpr {
    pub token: Token,
}

impl LookupExpr {
    pub fn new(token: Token) -> LookupExpr {
        LookupExpr { token }
    }
}

#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct ArgumentsExpr {
    pub expressions: Vec<Box<Expr>>,
}

impl ArgumentsExpr {
    pub fn new(expressions: Vec<Box<Expr>>) -> ArgumentsExpr {
        ArgumentsExpr { expressions }
    }
}

#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct LogicalExpr {
    pub left: Box<Expr>,
    pub right: Box<Expr>,
    pub operator: LogicalOperator,
}

impl LogicalExpr {
    pub fn new(left: Box<Expr>, right: Box<Expr>, operator: LogicalOperator) -> LogicalExpr {
        LogicalExpr { left, right, operator }
    }
}

#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct ThisExpr {}

impl ThisExpr {
    pub fn new() -> ThisExpr {
        ThisExpr {}
    }
}

#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct VarExpr {
    pub name: Token,
}

impl VarExpr {
    pub fn new(name: Token) -> VarExpr {
        VarExpr { name }
    }
}

#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct IdentifierExpr {
    pub value: String,
}

impl IdentifierExpr {
    pub fn new(value: String) -> IdentifierExpr {
        IdentifierExpr { value }
    }
}

#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct UnaryExpr {
    pub value: Box<Expr>,
    pub operator: UnaryOperator,
}

impl UnaryExpr {
    pub fn new(value: Box<Expr>, operator: UnaryOperator) -> UnaryExpr {
        UnaryExpr { value, operator }
    }
}

#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct PostfixExpr {
    pub value: Box<Expr>,
    pub operator: PostfixOperator,
}

impl PostfixExpr {
    pub fn new(value: Box<Expr>, operator: PostfixOperator) -> PostfixExpr {
        PostfixExpr { value, operator }
    }
}

#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct ClosureExpr {
    pub arguments: Vec<Argument>,
    pub body: Box<Stmt>,
}

impl ClosureExpr {
    pub fn new(arguments: Vec<Argument>, body: Box<Stmt>) -> ClosureExpr {
        ClosureExpr { arguments, body }
    }
}
