mod callable;
mod class;
mod error;
//mod function;
mod scope;
mod value;
mod vm;

pub use self::callable::*;
pub use self::class::*;
pub use self::error::*;
pub use self::scope::*;
pub use self::value::*;
pub use self::vm::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
