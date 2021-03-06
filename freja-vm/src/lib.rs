
// pub mod chunk;
// pub mod compiler;
pub mod error;
pub mod objects;
#[macro_use]
pub mod value;
pub mod context;
pub mod frames;
pub mod globals;
pub mod runner;
pub mod stack;
pub mod vm;
pub mod utils;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
