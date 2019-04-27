#[macro_use]
extern crate pest_derive;

pub mod lexer;
pub mod parser;

pub use freja_ast as ast;
pub use pest::iterators::{Pair, Pairs};

pub fn create_ast<'a>(source: &'a str) -> ast::Stmt<'a> {
    let tokens = lexer::tokenize(source).unwrap();
    parser::parse(tokens)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
