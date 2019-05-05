use super::value::{Value, ValuePtr};
use std::fmt;
use std::rc::Rc;

macro_rules! byte_instruction {
    ($name:expr, $chunk:expr, $offset: expr, $fmt:expr) => {{
        let code = $chunk.get_code($offset + 1);
        write!($fmt, "{:24}{:4}", $name, code as u8)?;
        $offset + 2
    }};
}

macro_rules! constant_instruction {
    ($name:expr, $chunk:expr, $offset: expr, $fmt:expr) => {{
        let code = $chunk.get_code($offset + 1) as u8;
        let constant = $chunk.get_constant(code as usize).unwrap();
        write!($fmt, "{:24}{:4} '{}'", $name, code, constant)?;
        $offset + 2
    }};
}

macro_rules! jump_instruction {
    ($name:expr, $chunk:expr,$offset: expr, $sign:expr, $fmt:expr) => {{
        let mut jump = ($chunk.code[$offset + 1] as u16) << 8;
        jump |= $chunk.code[$offset + 2] as u16;

        write!($fmt, "{:24}{:4} -> {}", $name, $offset, $offset + 3 + $sign * (jump as usize))?;
        $offset + 3
    }};
}

macro_rules! simple_instruction {
    ($name:expr, $offset: expr, $fmt:expr) => {{
        write!($fmt, "{}", $name)?;
        $offset + 1
    }};
}

macro_rules! simple_instruction_n {
    ($name:expr, $offset: expr, $n: expr, $fmt:expr) => {{
        write!($fmt, "{}_{}", $name, $n)?;
        $offset + 1
    }};
}

#[repr(u8)]
#[derive(Debug, Clone, PartialEq)]
pub enum OpCode {
    Constant,
    Nil,
    True,
    False,
    Pop,
    GetLocal,
    SetLocal,
    GetGlobal,
    SetGlobal,
    DefineGlobal,
    GetUpValue,
    SetUpValue,
    Equal,
    Greater,
    Less,
    Add,
    Substract,
    Multiply,
    Divide,
    Not,
    Negate,
    Return,
    Jump,
    JumpIfFalse,
    Array,
    Property,
    Closure,
    Call0,
    Call1,
    Call2,
    Call3,
    Call4,
    Call5,
    Call6,
    Call7,
    Call8,
}

impl From<u8> for OpCode {
    fn from(i: u8) -> OpCode {
        if i <= (OpCode::Call8 as u8) {
            return unsafe { std::mem::transmute::<_, OpCode>(i) };
        }
        panic!("invalid repo {}", i);
    }
}

impl From<OpCode> for u8 {
    fn from(op: OpCode) -> u8 {
        op as u8
    }
}

#[derive(PartialEq)]
pub struct Chunk {
    pub(crate) code: Vec<u8>,
    constants: Vec<ValuePtr>,
    lines: Vec<i32>,
}

impl Chunk {
    pub fn new() -> Chunk {
        Chunk { code: Vec::new(), constants: Vec::new(), lines: Vec::new() }
    }

    pub fn len(&self) -> usize {
        self.code.len()
    }

    pub fn get_code(&self, offset: usize) -> OpCode {
        OpCode::from(self.code[offset])
    }

    pub fn get_constant(&self, constant: usize) -> Option<&ValuePtr> {
        self.constants.get(constant)
    }

    pub fn get_line(&self, line: usize) -> Option<&i32> {
        self.lines.get(line)
    }

    pub fn write_byte(&mut self, byte: u8) {
        self.code.push(byte);
    }

    pub fn write_opcode(&mut self, code: OpCode) {
        self.code.push(code as u8);
        //self.lines.push(line)
    }

    pub fn add_constant(&mut self, value: Value) -> usize {
        for kv in self.constants.iter().enumerate() {
            if kv.1.as_ref() == &value {
                return kv.0;
            }
        }
        self.constants.push(Rc::new(value));
        self.constants.len() - 1
    }

    fn disamble(&self, offset: usize, f: &mut fmt::Formatter) -> Result<usize, fmt::Error> {
        write!(f, "{:04}    ", offset)?;

        let opcode = OpCode::from(self.code[offset]);
        let m = match opcode {
            OpCode::Constant => constant_instruction!("OP_CONSTANT", self, offset, f),
            OpCode::Nil => simple_instruction!("OP_NIL", offset, f),
            OpCode::True => simple_instruction!("OP_TRUE", offset, f),
            OpCode::False => simple_instruction!("OP_FALSE", offset, f),
            OpCode::Pop => simple_instruction!("OP_POP", offset, f),
            OpCode::GetUpValue => byte_instruction!("OP_GET_UPVALUE", self, offset, f),
            OpCode::SetUpValue => byte_instruction!("OP_SET_UPVALUE", self, offset, f),
            OpCode::GetLocal => byte_instruction!("OP_GET_LOCAL", self, offset, f),
            OpCode::GetGlobal => constant_instruction!("OP_GET_GLOBAL", self, offset, f),
            OpCode::DefineGlobal => constant_instruction!("OP_DEFINE_GLOBAL", self, offset, f),
            OpCode::SetLocal => byte_instruction!("OP_SET_LOCAL", self, offset, f),
            OpCode::SetGlobal => constant_instruction!("OP_SET_GLOBAL", self, offset, f),
            OpCode::Equal => simple_instruction!("OP_EQUAL", offset, f),
            OpCode::Greater => simple_instruction!("OP_GREATER", offset, f),
            OpCode::Less => simple_instruction!("OP_LESS", offset, f),
            OpCode::Add => simple_instruction!("OP_ADD", offset, f),
            OpCode::Substract => simple_instruction!("OP_SUBTRACT", offset, f),
            OpCode::Multiply => simple_instruction!("OP_MULTIPLY", offset, f),
            OpCode::Divide => simple_instruction!("OP_DIVIDE", offset, f),
            OpCode::Not => simple_instruction!("OP_NOT", offset, f),
            OpCode::Negate => simple_instruction!("OP_NEGATE", offset, f),
            OpCode::Return => simple_instruction!("OP_RETURN", offset, f),
            OpCode::Jump => jump_instruction!("OP_JUMP", self, offset, 1, f),
            OpCode::JumpIfFalse => jump_instruction!("OP_JUMP_iF_FALSE", self, offset, 1, f),
            OpCode::Array => byte_instruction!("OP_ARRAY", self, offset, f),
            OpCode::Property => simple_instruction!("OP_PROPERTY", offset, f),
            OpCode::Closure => {
                let mut offset = offset + 1;
                let constant = self.code[offset];
                offset += 1;
                let value = &self.constants[constant as usize];
                writeln!(f, "{:24} {:4} {}", "OP_CLOSURE", constant, value)?;

                if let Some(fu) = value.as_function() {
                    for u in 0..fu.up_value_count {
                        let local = self.code.get(offset);
                        offset += 1;
                        let index = self.code[offset];
                        offset += 1;
                        writeln!(f, "{:04}   | {:16} {}", offset - 2, if local.is_some() { "local" } else { "upvalue" }, index)?;
                    }

                    writeln!(f, "{:}", fu.chunk)?;
                };

                offset
            }
            OpCode::Call0 | OpCode::Call1 | OpCode::Call2 | OpCode::Call3 | OpCode::Call4 | OpCode::Call5 | OpCode::Call6 | OpCode::Call7 | OpCode::Call8 => simple_instruction_n!("OP_CALL", offset, ((opcode as u8) - (OpCode::Call0 as u8)), f),
            _ => unimplemented!("unknown code {:?}", opcode),
        };
        Ok(m)
    }
}

impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut i = 0;

        while i < self.code.len() {
            i = self.disamble(i, f)?;
            write!(f, "\n")?;
        }

        Ok(())
    }
}