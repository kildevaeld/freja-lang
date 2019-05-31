use super::chunk::Chunk;
use super::objects::Function;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(PartialEq, Debug)]
pub enum FunctionType {
    Function,
    Method,
    Initializer,
    TopLevel,
}

#[derive(Debug)]
pub(crate) struct Local {
    pub(crate) depth: i32,
    pub(crate) name: String,
    pub(crate) is_upvalue: bool,
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

pub(crate) struct UpValue {
    pub(crate) index: u8,
    pub(crate) is_local: bool,
}

pub struct CompilerState {
    pub(crate) enclosing: Option<CompilerStatePtr>,
    pub(crate) locals: Vec<Local>,
    pub(crate) up_values: Vec<UpValue>,
    pub(crate) scope_depth: i32,
    pub(crate) function: Function,
    pub(crate) function_type: FunctionType,
    pub(crate) member_depth: i32,
    pub(crate) assign: bool,
}

pub type CompilerStatePtr = Rc<RefCell<CompilerState>>;

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

    // pub fn chunk_mut(&mut self) -> &mut Chunk {
    //     &mut self.function.chunk
    // }
}

pub type ClassCompilerStatePtr = Rc<RefCell<ClassCompilerState>>;

pub struct ClassCompilerState {
    pub(crate) enclosing: Option<ClassCompilerStatePtr>,
    _name: String,
    pub(crate) has_super_class: bool,
}

impl ClassCompilerState {
    pub fn new(enclosing: Option<ClassCompilerStatePtr>, name: String, has_super_class: bool) -> ClassCompilerStatePtr {
        Rc::new(RefCell::new(ClassCompilerState {
            enclosing,
            _name: name,
            has_super_class,
        }))
    }
}
