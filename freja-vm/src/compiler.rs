use super::chunk::{Chunk, OpCode};
use super::error::{CompileError, CompileResult};
use super::value::Value;
use freja_parser::ast::*;

pub struct Local(i32, String);

pub struct Compiler {
    locals: Vec<Local>,
    scope_depth: i32,
    chunk: Chunk,
}

impl Compiler {
    pub fn new() -> Compiler {
        Compiler { locals: Vec::new(), scope_depth: 0, chunk: Chunk::new() }
    }

    pub fn compile(mut self, ast: &ProgramStmt) -> CompileResult<Chunk> {
        for stmt in &ast.statements {
            stmt.accept(&mut self)?;
        }
        Ok(self.chunk)
    }

    pub fn add_local(&mut self, name: &str) {
        if self.locals.len() > 256 {}
        self.locals.push(Local(-1, name.to_string()))
    }

    pub fn declare_variable(&mut self, name: &str) -> CompileResult<()> {
        if self.scope_depth == 0 {
            return Ok(());
        }
        for i in (0..self.locals.len()).into_iter().rev() {
            if self.locals[i].0 == -1 && self.locals[i].0 < self.scope_depth {
                break;
            }
            if self.locals[i].1.as_str() == name {
                return Err(CompileError::DuplicateVariable);
            }
        }

        self.add_local(name);
        Ok(())
    }

    pub fn mark_initialized(&mut self) {
        self.locals.last_mut().unwrap().0 = self.scope_depth;
    }

    pub fn resolve_local(&self, name: &str) -> Option<usize> {
        match self.locals.iter().enumerate().find(|m| (m.1).1.as_str() == name) {
            Some(m) => Some(m.0),
            None => None,
        }
    }

    pub fn begin_scope(&mut self) {
        self.scope_depth += 1;
    }

    pub fn end_scope(&mut self) {
        while !self.locals.is_empty() && self.locals.last().map(|n| n.0).unwrap_or(0) > self.scope_depth {
            self.emit(OpCode::Pop);
            self.locals.pop();
        }
    }

    pub fn is_local(&self) -> bool {
        self.scope_depth > 0
    }

    fn current_chunk(&mut self) -> &mut Chunk {
        &mut self.chunk
    }

    fn emit_opcode(&mut self, code: OpCode) {
        self.emit(code);
    }

    fn emit_opcode_byte(&mut self, code: OpCode, byte: u8) {
        self.emit(code);
        self.emit(byte);
    }

    fn make_constant(&mut self, value: Value) -> usize {
        let constant = self.current_chunk().add_constant(value);
        constant
    }

    fn emit_constant(&mut self, value: Value) {
        let constant = self.make_constant(value);
        self.emit_opcode_byte(OpCode::Constant, constant as u8)
    }

    fn emit<BS: Into<u8>>(&mut self, code: BS) {
        self.current_chunk().write_byte(code.into());
    }

    fn emit_jump<BS: Into<u8>>(&mut self, code: BS) -> usize {
        self.emit(code);
        self.emit(0xff);
        self.emit(0xff);
        self.current_chunk().len() - 2
    }

    //#[allow(exceeding_bitshifts)]
    fn patch_jump(&mut self, offset: usize) {
        let jump = (self.current_chunk().len() - offset - 2) as u16;

        //   if (jump > UINT16_MAX) {
        //     error("Too much code to jump over.");
        //   }

        self.current_chunk().code[offset] = ((jump >> 8) & 0xff) as u8;
        self.current_chunk().code[offset + 1] = (jump & 0xff) as u8;
    }

    fn end_compile(&mut self) {
        self.emit(OpCode::Return)
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
        let mut global = 0;
        self.declare_variable(e.name.as_str())?;
        if !self.is_local() {
            global = self.make_constant(Value::String(e.name.to_string()));
        }
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
        // self.emit(OpCode::Pop);
        Ok(())
    }
    fn visit_func_stmt(&mut self, e: &FuncStmt) -> CompileResult<()> {
        //
        Ok(())
    }
    fn visit_class_stmt(&mut self, e: &ClassStmt) -> CompileResult<()> {
        //
        Ok(())
    }
    fn visit_interface_stmt(&mut self, e: &InterfaceStmt) -> CompileResult<()> {
        //
        Ok(())
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
        //self.patch_jump(end);
        Ok(())
    }
    fn visit_for_stmt(&mut self, e: &ForStmt) -> CompileResult<()> {
        //
        Ok(())
    }
    fn visit_return_stmt(&mut self, e: &ReturnStmt) -> CompileResult<()> {
        //
        Ok(())
    }
    fn visit_continue_stmt(&mut self, e: &ContinueStmt) -> CompileResult<()> {
        //
        Ok(())
    }
    fn visit_break_stmt(&mut self, e: &BreakStmt) -> CompileResult<()> {
        //
        Ok(())
    }
}

impl ExprVisitor<CompileResult<()>> for Compiler {
    fn visit_assign_expr(&mut self, e: &AssignExpr) -> CompileResult<()> {
        //
        Ok(())
    }

    fn visit_call_expr(&mut self, e: &CallExpr) -> CompileResult<()> {
        //
        Ok(())
    }

    fn visit_literal_expr(&mut self, e: &LiteralExpr) -> CompileResult<()> {
        //
        let val = match &e.value {
            Literal::Number(Number::Double(d)) => Value::Double(*d),
            Literal::Number(Number::Integer(i)) => Value::Integer(*i),
            Literal::Boolean(b) => Value::Boolean(*b),
            Literal::String(s) => Value::String(s.clone()),
            _ => unimplemented!("literal"),
        };
        self.emit_constant(val);
        Ok(())
    }

    fn visit_binary_expr(&mut self, e: &BinaryExpr) -> CompileResult<()> {
        //
        let opc = match &e.operator {
            BinaryOperator::Add => OpCode::Add,
            BinaryOperator::Sub => OpCode::Substract,
            BinaryOperator::Mul => OpCode::Multiply,
            BinaryOperator::Div => OpCode::Divide,
            _ => unimplemented!("binary {:?}", e.operator),
        };

        self.emit(opc);
        e.left.accept(self)?;
        e.right.accept(self)?;
        Ok(())
    }

    fn visit_member_expr(&mut self, e: &MemberExpr) -> CompileResult<()> {
        //
        Ok(())
    }

    fn visit_lookup_expr(&mut self, e: &LookupExpr) -> CompileResult<()> {
        //
        Ok(())
    }

    fn visit_arguments_expr(&mut self, e: &ArgumentsExpr) -> CompileResult<()> {
        //
        Ok(())
    }

    fn visit_logical_expr(&mut self, e: &LogicalExpr) -> CompileResult<()> {
        //
        Ok(())
    }

    fn visit_this_expr(&mut self, e: &ThisExpr) -> CompileResult<()> {
        //
        Ok(())
    }

    fn visit_var_expr(&mut self, e: &VarExpr) -> CompileResult<()> {
        //
        Ok(())
    }

    fn visit_identifier_expr(&mut self, e: &IdentifierExpr) -> CompileResult<()> {
        //
        Ok(())
    }

    fn visit_unary_expr(&mut self, e: &UnaryExpr) -> CompileResult<()> {
        //
        Ok(())
    }

    fn visit_postfix_expr(&mut self, e: &PostfixExpr) -> CompileResult<()> {
        //
        Ok(())
    }
}
