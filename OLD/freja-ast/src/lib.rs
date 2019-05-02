#[macro_use]
extern crate serde;

mod ast;
mod ast_ext;

pub use ast::*;
pub use ast_ext::*;
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
