#[macro_use]
extern crate pest_derive;
#[macro_use]
extern crate serde_derive;

pub mod ast;
pub mod lexer;
pub mod parser;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
