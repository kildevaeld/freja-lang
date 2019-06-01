
mod chunk;
mod compiler_state;
mod compiler;
mod constant;
mod error;
mod objects;
//mod utils;

pub use self::error::*;
pub use self::chunk::*;
pub use self::constant::*;
pub use self::objects::*;
pub use self::compiler::Compiler;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
