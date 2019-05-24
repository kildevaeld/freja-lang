use super::chunk::{Chunk, OpCode};
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

#[derive(Debug)]
pub struct Local {
    depth: i32,
    name: String,
    is_upvalue: bool,
}

impl Local {
    pub fn new(depth: i32, name: String, is_upvalue: bool) -> Local {
        Local {
            depth,
            name,
            is_upvalue,
        }
    }
}

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
    member_depth: i32,
    assign: bool,
}

type CompilerStatePtr = Rc<RefCell<CompilerState>>;

impl CompilerState {
    pub fn new(enclosing: Option<CompilerStatePtr>, scope_depth: i32, function_type: FunctionType) -> CompilerStatePtr {
        let local = if function_type != FunctionType::Function && function_type != FunctionType::TopLevel {
            // In a method, it holds the receiver, "this".
            Local::new(scope_depth, "this".to_string(), false)
        } else {
            // In a function, it holds the function, but cannot be referenced,
            // so has no name.
            Local::new(scope_depth, "".to_string(), false)
        };

        let state = CompilerState {
            enclosing,
            locals: vec![local],
            scope_depth,
            function_type,
            function: Function::new(),
            up_values: Vec::new(),
            member_depth: 0,
            assign: false,
        };

        Rc::new(RefCell::new(state))
    }

    pub fn chunk(&self) -> &Chunk {
        &self.function.chunk()
    }

    pub fn chunk_mut(&mut self) -> &mut Chunk {
        &mut self.function.chunk
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
        self.state_mut().locals.push(Local::new(-1, name.to_string(), false))
    }

    pub fn add_upvalue(&mut self, index: usize, is_local: bool) -> usize {
        let mut state = self.state_mut();
        self._add_upvalue(&mut state, index, is_local)
    }

    fn _add_upvalue(&self, state: &mut CompilerState, index: usize, is_local: bool) -> usize {
        for kv in state.up_values.iter().enumerate() {
            if kv.1.index == (index as u8) && kv.1.is_local == is_local {
                return kv.0;
            }
        }

        state.up_values.push(UpValue {
            index: index as u8,
            is_local,
        });

        state.function.up_value_count += 1;
        state.up_values.len() - 1
    }

    pub fn declare_variable(&mut self, name: &str) -> CompileResult<()> {
        if self.state().scope_depth == 0 {
            return Ok(());
        }
        for i in (0..self.state().locals.len()).into_iter().rev() {
            if self.state().locals[i].depth == -1 && self.state().locals[i].depth < self.state().scope_depth {
                break;
            }
            if self.state().locals[i].name.as_str() == name {
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

        self.emit_bytes(OpCode::DefineGlobal, global as u8);
    }

    pub fn mark_initialized(&mut self) {
        let scope = self.state().scope_depth;
        if let Some(last) = self.state_mut().locals.last_mut() {
            last.depth = scope;
        }
    }

    pub fn resolve_local(&self, name: &str) -> Option<usize> {
        match self
            .state()
            .locals
            .iter()
            .enumerate()
            .find(|m| (m.1).name.as_str() == name)
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
            .find(|m| (m.1).name.as_str() == name)
        {
            Some(m) => Some(m.0),
            None => None,
        }
    }

    fn _resolve_upvalue(&self, state: &mut CompilerState, name: &str) -> Option<usize> {
        match &state.enclosing {
            Some(s) => {
                if let Some(local) = self._resolve_local(&s, name) {
                    {
                        s.borrow_mut().locals[local].is_upvalue = true;
                    }

                    return Some(self._add_upvalue(state, local, true));
                }

                let mut ret = {
                    let mut s = s.borrow_mut();
                    self._resolve_upvalue(&mut s, name)
                };

                if let Some(upvalue) = ret.take() {
                    return Some(self._add_upvalue(state, upvalue, false));
                }
            }
            None => {}
        };

        None
    }

    pub fn resolve_upvalue(&self, name: &str) -> Option<usize> {
        let mut b = self.state_mut();
        self._resolve_upvalue(&mut b, name)
    }

    pub fn begin_scope(&mut self) {
        self.state_mut().scope_depth += 1;
    }

    pub fn end_scope(&mut self, pop_locals: bool) {
        self.state_mut().scope_depth -= 1;

        if pop_locals {
            self.pop_locals();
        }
    }

    fn pop_locals(&mut self) {
        while !self.state().locals.is_empty()
            && self.state().locals.last().map(|n| n.depth).unwrap_or(0) > self.state().scope_depth
        {
            if self.state().locals[self.state().locals.len() - 1].is_upvalue {
                self.emit(OpCode::CloseUpValue);
            } else {
                self.emit(OpCode::Pop);
            }
            self.state_mut().locals.pop();
        }
    }

    pub fn is_local(&self) -> bool {
        self.state().scope_depth > 0
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
        self.emit_bytes(OpCode::Constant, constant as u8)
    }

    fn emit<BS: Into<u8>>(&mut self, code: BS) {
        self.state_mut().function.chunk.write_byte(code.into());
    }

    fn emit_bytes<BS: Into<u8>, BS2: Into<u8>>(&mut self, code: BS, byte: BS2) {
        self.emit(code);
        self.emit(byte);
    }

    fn emit_jump<BS: Into<u8>>(&mut self, code: BS) -> usize {
        self.emit(code);
        self.emit(0xff);
        self.emit(0xff);
        self.state().function.chunk.len() - 2
    }

    fn emit_return(&mut self) {
        if self.state().function_type == FunctionType::Initializer {
            self.emit_bytes(OpCode::GetLocal, 0);
        } else {
            self.emit(OpCode::Nil);
        }
        self.emit(OpCode::Return);
    }

    fn emit_loop(&mut self, start: usize) {
        self.emit(OpCode::Loop);
        let offset = self.state().chunk().len() - start + 2;
        self.emit(((offset >> 8) & 0xff) as u8);
        self.emit((offset & 0xff) as u8);
    }

    fn patch_jump(&mut self, offset: usize) {
        let jump = (self.state().function.chunk.len() - offset - 2) as u16;

        self.state_mut().function.chunk.code[offset] = ((jump >> 8) & 0xff) as u8;
        self.state_mut().function.chunk.code[offset + 1] = (jump & 0xff) as u8;
    }

    fn end_compile(&mut self) -> Function {
        self.emit_return();
        self.pop_locals();
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
        self.function2(&e.parameters, &e.body, Some(e.name.as_str()), type_)
    }

    fn function2(
        &mut self,
        params: &[Argument],
        body: &Stmt,
        name: Option<&str>,
        type_: FunctionType,
    ) -> CompileResult<()> {
        let state = CompilerState::new(Some(self.state.clone()), 1, type_);
        {
            let mut s = state.borrow_mut();
            s.function.arity = params.len() as i32;
            s.function.name = name.map(|m| m.to_string());
        }
        self.state = state.clone();

        for p in params {
            match p {
                Argument::Regular(m) => {
                    let global = self.parse_var(m.as_str());
                    self.define_variable(global);
                }
                Argument::Rest(_) => unimplemented!("rest not implemented"),
            };
        }

        match body {
            Stmt::Block(b) => {
                for bb in &b.statements {
                    bb.accept(self)?;
                }
            }
            Stmt::Expr(e) => {
                self.visit_expr_stmt(e)?;
            }
            _ => unimplemented!("should be block or expression statement, was: {:?}", body),
        };

        self.end_scope(false);

        let function = self.end_compile();

        let constant = self.make_constant(Value::Function(Rc::new(function))) as u8;
        self.emit_bytes(OpCode::Closure, constant);

        for i in state.borrow().up_values.iter() {
            self.emit(if i.is_local { 1 } else { 0 });
            self.emit(i.index);
        }

        Ok(())
    }

    fn variable(&mut self, name: &str) {
        let (a, get, set) = if let Some(a) = self.resolve_local(name) {
            (a, OpCode::GetLocal, OpCode::SetLocal)
        } else if let Some(a) = self.resolve_upvalue(name) {
            (a, OpCode::GetUpValue, OpCode::SetUpValue)
        } else {
            let a = self.make_constant(Value::String(name.to_owned()));
            (a, OpCode::GetGlobal, OpCode::SetGlobal)
        };

        if self.state().assign {
            self.emit_bytes(set, a as u8)
        } else {
            self.emit_bytes(get, a as u8);
        }
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
            self.emit_bytes(OpCode::DefineGlobal, global as u8);
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

        Ok(())
    }
    fn visit_class_stmt(&mut self, e: &ClassStmt) -> CompileResult<()> {
        let global = self.make_constant(Value::String(e.name.to_string()));
        self.declare_variable(&e.name)?;
        self.emit_bytes(OpCode::Class, global as u8);
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

                    self.emit_bytes(OpCode::Method, name as u8);
                }
                _ => unimplemented!("invalid class member {:?}", m),
            }
        }

        if e.extends.is_some() {
            self.end_scope(true);
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
        self.end_scope(true);
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
    fn visit_for_stmt(&mut self, e: &ForStmt) -> CompileResult<()> {
        self.begin_scope();

        if let Some(i) = e.initializer.as_ref() {
            i.accept(self)?;
        }

        let mut loop_start = self.state().chunk().len();
        let mut exit_jump = None;
        if let Some(i) = &e.condition {
            i.accept(self)?;
            exit_jump = Some(self.emit_jump(OpCode::JumpIfFalse));
            self.emit(OpCode::Pop);
        }

        if let Some(i) = &e.increment {
            let jump_body = self.emit_jump(OpCode::Jump);
            let increment_start = self.state().chunk().len();

            i.accept(self)?;
            self.emit(OpCode::Pop);

            self.emit_loop(loop_start);

            loop_start = increment_start;

            self.patch_jump(jump_body);
        }

        e.body.accept(self)?;

        self.emit_loop(loop_start);

        if let Some(exit_jump) = exit_jump {
            self.patch_jump(exit_jump);
            self.emit(OpCode::Pop);
        }

        self.end_scope(true);

        Ok(())
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
    fn visit_assign_expr(&mut self, e: &AssignExpr) -> CompileResult<()> {
        match e.operator {
            AssignmentOperator::Assign => {}
            _ => unimplemented!("assigment operator: {:?}", e.operator),
        }

        match e.destination.as_ref() {
            Expr::Member(mem) => {
                mem.object.accept(self)?;
                e.value.accept(self)?;
                self.state_mut().assign = true;
                self.state_mut().member_depth += 1;
                mem.property.accept(self)?;
                self.state_mut().member_depth -= 1;
            }
            i => {
                e.value.accept(self)?;
                self.state_mut().assign = true;
                i.accept(self)?;
            }
        };

        self.state_mut().assign = false;

        Ok(())
    }

    fn visit_call_expr(&mut self, e: &CallExpr) -> CompileResult<()> {
        let c = e.arguments.len() as u8;

        match e.member.as_ref() {
            Expr::Member(mem) => {
                //handle super
                let code = match mem.object.as_ref() {
                    Expr::Super(_) => {
                        //
                        //self.variable("this");

                        OpCode::Super0
                    }
                    _ => {
                        mem.object.accept(self)?;
                        OpCode::Invoke0
                    }
                };

                let name = match mem.property.as_ref() {
                    Expr::Identifier(i) => i.value.as_str(),
                    _ => unimplemented!("invalid call member"),
                };
                let name = self.make_constant(Value::String(name.to_string()));
                for a in &e.arguments {
                    a.accept(self)?;
                }
                if code == OpCode::Super0 {
                    self.variable("super");
                }

                self.emit_bytes(OpCode::from((code as u8) + c), name as u8);
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
            Literal::Number(Number::Double(d)) => Value::Double(*d),
            Literal::Number(Number::Integer(i)) => Value::Integer(*i),
            Literal::Boolean(b) => Value::Boolean(*b),
            Literal::String(s) => Value::String(s.clone()),
            Literal::Array(arr) => {
                for a in arr.iter() {
                    a.accept(self)?;
                }
                self.emit_bytes(OpCode::Array, arr.len() as u8);
                return Ok(());
            }
            Literal::Object(obj) => {
                for kv in obj.entries().iter() {
                    kv.key.accept(self)?;
                    kv.value.accept(self)?;
                }

                self.emit_bytes(OpCode::Map, obj.entries().len() as u8);
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

        self.state_mut().member_depth += 1;

        e.property.accept(self)?;

        self.state_mut().member_depth -= 1;

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

    fn visit_super_expr(&mut self, e: &SuperExpr) -> CompileResult<()> {
        Ok(())
    }

    fn visit_var_expr(&mut self, _e: &VarExpr) -> CompileResult<()> {
        unimplemented!("expr");
    }

    fn visit_identifier_expr(&mut self, e: &IdentifierExpr) -> CompileResult<()> {
        if self.state().member_depth > 0 {
            let global = self.make_constant(Value::String(e.value.to_string()));
            if self.state().assign {
                self.emit_bytes(OpCode::SetProperty, global as u8);
            } else {
                self.emit_bytes(OpCode::GetProperty, global as u8);
            }
        } else {
            self.variable(e.value.as_str());
        }

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

    fn visit_postfix_expr(&mut self, e: &PostfixExpr) -> CompileResult<()> {
        let is_local = match e.value.as_ref() {
            Expr::Identifier(_) => true,
            _ => false,
        };
        e.value.accept(self)?;

        self.emit_constant(Value::Integer(1));
        self.emit(OpCode::Add);
        if is_local {
            let op = self.state().chunk().len() - 5;
            let idx = self.state().chunk().len() - 4;
            let local_code = self.state().chunk().get_code(op);
            let idx = self.state().chunk().get(idx);

            match local_code {
                OpCode::GetGlobal => self.emit_bytes(OpCode::SetGlobal, idx),
                OpCode::GetLocal => self.emit_bytes(OpCode::SetLocal, idx),
                _ => {}
            };
        }

        Ok(())
    }

    fn visit_closure_expr(&mut self, e: &ClosureExpr) -> CompileResult<()> {
        //unimplemented!("closure");
        self.function2(e.arguments.as_slice(), e.body.as_ref(), None, FunctionType::Function)
    }
}
