
mod chunk;
mod compiler_state;
mod compiler;
mod constants;
mod error;
mod objects;

pub use self::error::*;
pub use self::chunk::*;
pub use self::constants::*;
pub use self::objects::*;
pub use self::compiler::Compiler;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
