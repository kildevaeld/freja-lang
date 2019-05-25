#[cfg(feature = "serde_support")]
#[macro_use]
extern crate serde;

#[macro_use]
extern crate lazy_static;

pub mod chunk;
pub mod compiler;
pub mod error;
pub mod objects;
#[macro_use]
pub mod value;
pub mod context;
pub mod frames;
mod runner;
pub mod stack;
pub mod utils;
pub mod vm;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
