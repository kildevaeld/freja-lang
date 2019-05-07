use super::chunk::OpCode;
use super::error::{CompileError, CompileResult};
use super::objects::Function;
use super::value::Value;
use freja_parser::ast::*;
use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

#[derive(PartialEq, Debug)]
pub enum FunctionType {
    Function,
    Method,
    Initializer,
    TopLevel,
}

pub struct Local(i32, String, bool);

struct UpValue {
    index: u8,
    is_local: bool,
}

pub struct CompilerState {
    enclosing: Option<CompilerStatePtr>,
    locals: Vec<Local>,
    up_values: Vec<UpValue>,
    scope_depth: i32,
    function: Function,
    function_type: FunctionType,
}

type CompilerStatePtr = Rc<RefCell<CompilerState>>;

impl CompilerState {
    pub fn new(enclosing: Option<CompilerStatePtr>, scope_depth: i32, function_type: FunctionType) -> CompilerStatePtr {
        let local = if function_type != FunctionType::Function {
            // In a method, it holds the receiver, "this".
            Local(scope_depth, "this".to_string(), false)
        } else {
            // In a function, it holds the function, but cannot be referenced,
            // so has no name.
            Local(scope_depth, "".to_string(), false)
        };

        let state = CompilerState {
            enclosing,
            locals: vec![local],
            scope_depth,
            function_type,
            function: Function::new(),
            up_values: Vec::new(),
        };

        Rc::new(RefCell::new(state))
    }
}

type ClassCompilerStatePtr = Rc<RefCell<ClassCompilerState>>;

pub struct ClassCompilerState {
    enclosing: Option<ClassCompilerStatePtr>,
    name: String,
    has_super_class: bool,
}

impl ClassCompilerState {
    pub fn new(enclosing: Option<ClassCompilerStatePtr>, name: String, has_super_class: bool) -> ClassCompilerStatePtr {
        Rc::new(RefCell::new(ClassCompilerState {
            enclosing,
            name,
            has_super_class,
        }))
    }
}

pub struct Compiler {
    state: CompilerStatePtr,
    class: Option<ClassCompilerStatePtr>,
}

impl Compiler {
    pub fn new() -> Compiler {
        Compiler {
            state: CompilerState::new(None, 0, FunctionType::TopLevel),
            class: None,
        }
    }

    pub fn compile(&mut self, ast: &ProgramStmt) -> CompileResult<Function> {
        for stmt in &ast.statements {
            stmt.accept(self)?;
        }

        let function = self.end_compile();
        Ok(function)
    }

    pub fn reset_state(&mut self) {
        self.state = CompilerState::new(None, 0, FunctionType::TopLevel);
    }

    fn state(&self) -> Ref<CompilerState> {
        self.state.borrow()
    }

    fn state_mut(&self) -> RefMut<CompilerState> {
        self.state.borrow_mut()
    }

    pub fn add_local(&mut self, name: &str) {
        if self.state().locals.len() > 256 {}
        self.state_mut().locals.push(Local(-1, name.to_string(), false))
    }

    pub fn add_upvalue(&mut self, index: usize, is_local: bool) -> usize {
        for kv in self.state().up_values.iter().enumerate() {
            if kv.1.index == (index as u8) && kv.1.is_local == is_local {
                return kv.0;
            }
        }

        self.state().up_values.len()
    }

    fn _add_upvalue(&self, state: &CompilerStatePtr, index: usize, is_local: bool) -> usize {
        for kv in state.borrow().up_values.iter().enumerate() {
            if kv.1.index == (index as u8) && kv.1.is_local == is_local {
                return kv.0;
            }
        }

        state.borrow_mut().up_values.push(UpValue {
            index: index as u8,
            is_local,
        });

        state.borrow().up_values.len()
    }

    pub fn declare_variable(&mut self, name: &str) -> CompileResult<()> {
        if self.state().scope_depth == 0 {
            return Ok(());
        }
        for i in (0..self.state().locals.len()).into_iter().rev() {
            if self.state().locals[i].0 == -1 && self.state().locals[i].0 < self.state().scope_depth {
                break;
            }
            if self.state().locals[i].1.as_str() == name {
                return Err(CompileError::DuplicateVariable);
            }
        }

        self.add_local(name);
        Ok(())
    }

    pub fn define_variable(&mut self, global: usize) {
        if self.state().scope_depth > 0 {
            self.mark_initialized();
            return;
        }

        self.emit_opcode_byte(OpCode::DefineGlobal, global as u8);
    }

    pub fn mark_initialized(&mut self) {
        let scope = self.state().scope_depth;
        if let Some(last) = self.state_mut().locals.last_mut() {
            last.0 = scope;
        }
    }

    pub fn resolve_local(&self, name: &str) -> Option<usize> {
        match self
            .state()
            .locals
            .iter()
            .enumerate()
            .find(|m| (m.1).1.as_str() == name)
        {
            Some(m) => Some(m.0),
            None => None,
        }
    }

    fn _resolve_local(&self, state: &CompilerStatePtr, name: &str) -> Option<usize> {
        match state
            .borrow()
            .locals
            .iter()
            .enumerate()
            .find(|m| (m.1).1.as_str() == name)
        {
            Some(m) => Some(m.0),
            None => None,
        }
    }

    fn _resolve_upvalue(&self, state: &CompilerStatePtr, name: &str) -> Option<usize> {
        match &state.borrow().enclosing {
            Some(s) => {
                if let Some(local) = self._resolve_local(&s, name) {
                    {
                        s.borrow_mut().locals[local].2 = true;
                    }
                    return None;
                    //return Some(self._add_upvalue(&s, local, true));
                }

                return self._resolve_upvalue(&s, name);
            }
            None => {}
        };

        None
    }

    pub fn resolve_upvalue(&self, name: &str) -> Option<usize> {
        self._resolve_upvalue(&self.state, name)
        // if self.state().enclosing.is_none() {
        //     return None;
        // }
        // match &mut self.state_mut().enclosing {
        //     Some(s) => {
        //         if let Some(local) = self._resolve_local(s, name) {
        //             s.borrow_mut().locals[local].2 = true;
        //             return Some(self._add_upvalue(&s, local, true));
        //         }
        //     }
        //     None => {}
        // };

        // self._re
        // // let b = &self.state().enclosing.unwrap();
        // // let state = b.borrow();
        // // let local = self._resolve(&state, name);
        // None
    }

    pub fn begin_scope(&mut self) {
        self.state_mut().scope_depth += 1;
    }

    pub fn end_scope(&mut self) {
        self.state_mut().scope_depth -= 1;

        while !self.state().locals.is_empty()
            && self.state().locals.last().map(|n| n.0).unwrap_or(0) > self.state().scope_depth
        {
            self.emit(OpCode::Pop);
            self.state_mut().locals.pop();
        }
    }

    pub fn is_local(&self) -> bool {
        self.state().scope_depth > 0
    }

    // fn chunk(&self) -> &Chunk {
    //     &self.state().function.chunk
    // }

    // fn chunk_mut(&mut self) -> &mut Chunk {
    //     &mut self.state_mut().function.chunk
    // }

    fn emit_opcode(&mut self, code: OpCode) {
        self.emit(code);
    }

    fn emit_opcode_byte(&mut self, code: OpCode, byte: u8) {
        self.emit(code);
        self.emit(byte);
    }

    fn make_constant(&mut self, value: Value) -> usize {
        let constant = self.state_mut().function.chunk.add_constant(value);
        constant
    }

    fn parse_var(&mut self, name: &str) -> usize {
        self.declare_variable(name);
        if self.state().scope_depth > 0 {
            0
        } else {
            self.make_constant(Value::String(name.to_string()))
        }
    }

    fn emit_constant(&mut self, value: Value) {
        let constant = self.make_constant(value);
        self.emit_opcode_byte(OpCode::Constant, constant as u8)
    }

    fn emit<BS: Into<u8>>(&mut self, code: BS) {
        self.state_mut().function.chunk.write_byte(code.into());
    }

    fn emit_jump<BS: Into<u8>>(&mut self, code: BS) -> usize {
        self.emit(code);
        self.emit(0xff);
        self.emit(0xff);
        self.state().function.chunk.len() - 2
    }

    fn emit_return(&mut self) {
        if self.state().function_type == FunctionType::Initializer {
            self.emit_opcode_byte(OpCode::GetGlobal, 0);
        } else {
            self.emit_opcode(OpCode::Nil);
        }
        self.emit(OpCode::Return);
    }

    //#[allow(exceeding_bitshifts)]
    fn patch_jump(&mut self, offset: usize) {
        let jump = (self.state().function.chunk.len() - offset - 2) as u16;

        //   if (jump > UINT16_MAX) {
        //     error("Too much code to jump over.");
        //   }

        self.state_mut().function.chunk.code[offset] = ((jump >> 8) & 0xff) as u8;
        self.state_mut().function.chunk.code[offset + 1] = (jump & 0xff) as u8;
    }

    fn end_compile(&mut self) -> Function {
        self.emit_return();
        let function = std::mem::replace(&mut self.state_mut().function, Function::new());
        let state = self.state.clone();

        if let Some(e) = &state.borrow().enclosing {
            self.state = e.clone();
        } else {
            self.state = CompilerState::new(None, 0, FunctionType::TopLevel);
        }
        function
    }

    fn function(&mut self, e: &FuncStmt, type_: FunctionType) -> CompileResult<()> {
        let state = CompilerState::new(Some(self.state.clone()), 1, type_);
        {
            let mut s = state.borrow_mut();
            s.function.arity = e.parameters.len() as i32;
            s.function.name = Some(e.name.clone());
        }
        self.state = state.clone();

        for p in &e.parameters {
            match p {
                Argument::Regular(m) => {
                    let global = self.parse_var(m.as_str());
                    self.define_variable(global);
                }
                Argument::Rest(_) => unimplemented!("rest not implemented"),
            };
        }

        match e.body.as_ref() {
            Stmt::Block(b) => {
                for bb in &b.statements {
                    bb.accept(self)?;
                }
            }
            _ => unimplemented!("should be block"),
        };

        self.end_scope();

        let function = self.end_compile();

        let constant = self.make_constant(Value::Function(Rc::new(function))) as u8;
        self.emit_opcode_byte(OpCode::Closure, constant);

        for i in state.borrow().up_values.iter() {
            self.emit(if i.is_local { 1 } else { 0 });
            self.emit(i.index);
        }

        Ok(())
    }

    fn variable(&mut self, name: &str) {
        let (a, get, _set) = if let Some(a) = self.resolve_local(name) {
            (a, OpCode::GetLocal, OpCode::SetLocal)
        } else if let Some(a) = self.resolve_upvalue(name) {
            (a, OpCode::GetUpValue, OpCode::SetUpValue)
        } else {
            let a = self.make_constant(Value::String(name.to_owned()));
            (a, OpCode::GetGlobal, OpCode::SetGlobal)
        };

        self.emit_opcode_byte(get, a as u8);
    }
}

impl StmtVisitor<CompileResult<()>> for Compiler {
    fn visit_program_stmt(&mut self, e: &ProgramStmt) -> CompileResult<()> {
        //
        for stmt in &e.statements {
            stmt.accept(self)?;
        }
        Ok(())
    }
    fn visit_var_stmt(&mut self, e: &VarStmt) -> CompileResult<()> {
        let global = self.parse_var(e.name.as_str());

        match &e.initializer {
            Some(init) => init.accept(self)?,
            None => self.emit(OpCode::Nil),
        };
        if self.is_local() {
            self.mark_initialized();
        } else {
            self.emit_opcode_byte(OpCode::DefineGlobal, global as u8);
        }
        Ok(())
    }
    fn visit_varlist_stmt(&mut self, e: &VarListStmt) -> CompileResult<()> {
        for v in &e.variables {
            self.visit_var_stmt(v)?;
        }
        Ok(())
    }
    fn visit_expr_stmt(&mut self, e: &ExprStmt) -> CompileResult<()> {
        e.expression.accept(self)?;
        self.emit(OpCode::Pop);
        Ok(())
    }
    fn visit_func_stmt(&mut self, e: &FuncStmt) -> CompileResult<()> {
        let global = self.parse_var(e.name.as_str());
        self.mark_initialized();

        self.function(e, FunctionType::Function)?;

        self.define_variable(global as usize);

        // TODO: Upvalues

        Ok(())
    }
    fn visit_class_stmt(&mut self, e: &ClassStmt) -> CompileResult<()> {
        let global = self.make_constant(Value::String(e.name.to_string()));
        self.declare_variable(&e.name)?;
        self.emit_opcode_byte(OpCode::Class, global as u8);
        self.define_variable(global);

        self.class = Some(ClassCompilerState::new(
            self.class.as_ref().map(|m| m.clone()),
            e.name.clone(),
            e.extends.is_some(),
        ));

        if let Some(su) = &e.extends {
            self.class.as_ref().unwrap().borrow_mut().has_super_class = true;
            self.begin_scope();
            self.variable(su.as_str());
            self.add_local("super");
            self.define_variable(0);
            self.variable(e.name.as_str());
            self.emit(OpCode::Inherit);
        }

        for m in &e.members {
            match m.as_ref() {
                Stmt::Func(stmt) => {
                    self.variable(&e.name);
                    let name = self.make_constant(Value::String(stmt.name.to_string()));
                    let ty = if stmt.name.as_str() == "init" {
                        FunctionType::Initializer
                    } else {
                        FunctionType::Method
                    };

                    self.function(stmt, ty)?;

                    self.emit_opcode_byte(OpCode::Method, name as u8);
                }
                _ => unimplemented!("invalid class member {:?}", m),
            }
        }

        if e.extends.is_some() {
            self.end_scope();
        }

        let enc = self.class.as_ref().unwrap().borrow().enclosing.clone();
        self.class = enc;

        Ok(())
    }
    fn visit_interface_stmt(&mut self, _e: &InterfaceStmt) -> CompileResult<()> {
        //
        unimplemented!("interface");
    }
    fn visit_block_stmt(&mut self, e: &BlockStmt) -> CompileResult<()> {
        self.begin_scope();
        for b in &e.statements {
            b.accept(self)?;
        }
        self.end_scope();
        Ok(())
    }
    fn visit_if_stmt(&mut self, e: &IfStmt) -> CompileResult<()> {
        e.test.accept(self)?;
        let elsejump = self.emit_jump(OpCode::JumpIfFalse);

        self.emit(OpCode::Pop);
        e.consequent.accept(self)?;

        let end = self.emit_jump(OpCode::Jump);

        self.patch_jump(elsejump);
        self.emit(OpCode::Pop);

        if e.alternative.is_some() {
            e.alternative.as_ref().unwrap().accept(self)?;
        }

        self.patch_jump(end);

        Ok(())
    }
    fn visit_for_stmt(&mut self, _e: &ForStmt) -> CompileResult<()> {
        unimplemented!("for loop");
    }
    fn visit_return_stmt(&mut self, e: &ReturnStmt) -> CompileResult<()> {
        if self.state().function_type == FunctionType::TopLevel {
            return Err(CompileError::ReturnTopLevel);
        }
        match &e.expression {
            Some(s) => {
                s.accept(self)?;
                self.emit(OpCode::Return);
            }
            None => {
                self.emit_return();
            }
        };
        Ok(())
    }
    fn visit_continue_stmt(&mut self, _e: &ContinueStmt) -> CompileResult<()> {
        //
        unimplemented!("continue");
    }
    fn visit_break_stmt(&mut self, _e: &BreakStmt) -> CompileResult<()> {
        //
        unimplemented!("break");
    }
}

impl ExprVisitor<CompileResult<()>> for Compiler {
    fn visit_assign_expr(&mut self, _e: &AssignExpr) -> CompileResult<()> {
        //
        unimplemented!("assign");
    }

    fn visit_call_expr(&mut self, e: &CallExpr) -> CompileResult<()> {
        let c = e.arguments.len() as u8;
        match e.member.as_ref() {
            Expr::Member(mem) => {
                mem.object.accept(self)?;
                let name = match mem.property.as_ref() {
                    Expr::Identifier(i) => i.value.as_str(),
                    _ => unimplemented!("invalid call member"),
                };
                let name = self.make_constant(Value::String(name.to_string()));
                for a in &e.arguments {
                    a.accept(self)?;
                }
                self.emit_opcode_byte(OpCode::from((OpCode::Invoke0 as u8) + c), name as u8);
            }
            _ => {
                e.member.accept(self)?;
                for a in &e.arguments {
                    a.accept(self)?;
                }

                self.emit(OpCode::from((OpCode::Call0 as u8) + c));
            }
        }

        //unimplemented!("call");
        Ok(())
    }

    fn visit_literal_expr(&mut self, e: &LiteralExpr) -> CompileResult<()> {
        //
        let val = match &e.value {
            Literal::Number(n) => Value::Number(n.clone()),
            //Literal::Number(Number::Double(d)) => Value::Double(*d),
            //Literal::Number(Number::Integer(i)) => Value::Integer(*i),
            Literal::Boolean(b) => Value::Boolean(*b),
            Literal::String(s) => Value::String(s.clone()),
            Literal::Array(arr) => {
                for a in arr.iter() {
                    a.accept(self)?;
                }
                self.emit_opcode_byte(OpCode::Array, arr.len() as u8);
                return Ok(());
            }
            _ => unimplemented!("literal"),
        };
        self.emit_constant(val);
        Ok(())
    }

    fn visit_binary_expr(&mut self, e: &BinaryExpr) -> CompileResult<()> {
        //

        e.left.accept(self)?;
        e.right.accept(self)?;

        let opc = match &e.operator {
            BinaryOperator::Add => OpCode::Add,
            BinaryOperator::Sub => OpCode::Substract,
            BinaryOperator::Mul => OpCode::Multiply,
            BinaryOperator::Div => OpCode::Divide,
            BinaryOperator::Eq => OpCode::Equal,
            BinaryOperator::Lte => {
                self.emit(OpCode::Greater);
                OpCode::Not
            }
            BinaryOperator::Lt => OpCode::Less,
            BinaryOperator::Gt => OpCode::Greater,
            BinaryOperator::Gte => {
                self.emit(OpCode::Less);
                OpCode::Not
            }
            BinaryOperator::Neq => {
                self.emit(OpCode::Equal);
                OpCode::Not
            }
            _ => unimplemented!("binary {:?}", e.operator),
        };

        self.emit(opc);

        Ok(())
    }

    fn visit_member_expr(&mut self, e: &MemberExpr) -> CompileResult<()> {
        //

        e.object.accept(self)?;

        // match &e.property {
        //     Expr::Identifier(i) => {}
        //     _ => unimplemented!("member lookup: {:?}", e.property),
        // };
        e.property.accept(self)?;
        self.emit(OpCode::Property);
        //unimplemented!("member");
        Ok(())
    }

    fn visit_lookup_expr(&mut self, _e: &LookupExpr) -> CompileResult<()> {
        unimplemented!("lookup");
    }

    fn visit_arguments_expr(&mut self, _e: &ArgumentsExpr) -> CompileResult<()> {
        unimplemented!("arguments");
    }

    fn visit_logical_expr(&mut self, e: &LogicalExpr) -> CompileResult<()> {
        //
        e.left.accept(self)?;

        let jump = match e.operator {
            LogicalOperator::And => {
                let jump = self.emit_jump(OpCode::JumpIfFalse);
                self.emit(OpCode::Pop);
                jump
            }
            LogicalOperator::Or => {
                let else_jump = self.emit_jump(OpCode::JumpIfFalse);
                let end_jump = self.emit_jump(OpCode::Jump);
                self.patch_jump(else_jump);
                self.emit(OpCode::Pop);
                end_jump
            }
        };
        e.right.accept(self)?;
        self.patch_jump(jump);
        Ok(())
    }

    fn visit_this_expr(&mut self, _e: &ThisExpr) -> CompileResult<()> {
        self.variable("this");
        Ok(())
    }

    fn visit_var_expr(&mut self, _e: &VarExpr) -> CompileResult<()> {
        unimplemented!("expr");
    }

    fn visit_identifier_expr(&mut self, e: &IdentifierExpr) -> CompileResult<()> {
        //
        let (a, get, _) = if let Some(a) = self.resolve_local(e.value.as_str()) {
            (a, OpCode::GetLocal, OpCode::SetLocal)
        } else if let Some(a) = self.resolve_upvalue(e.value.as_str()) {
            (a, OpCode::GetUpValue, OpCode::SetUpValue)
        } else {
            let a = self.make_constant(Value::String(e.value.clone()));
            (a, OpCode::GetGlobal, OpCode::SetGlobal)
        };
        //unimplemented!("idenfier");
        self.emit_opcode_byte(get, a as u8);

        Ok(())
    }

    fn visit_unary_expr(&mut self, e: &UnaryExpr) -> CompileResult<()> {
        e.value.accept(self)?;
        let op = match &e.operator {
            UnaryOperator::Not => OpCode::Not,
            UnaryOperator::Minus => OpCode::Negate,
            _ => unimplemented!("unary operator {:?}", e.operator),
        };
        self.emit(op);
        Ok(())
    }

    fn visit_postfix_expr(&mut self, _e: &PostfixExpr) -> CompileResult<()> {
        //
        unimplemented!("postfix");
    }
}
