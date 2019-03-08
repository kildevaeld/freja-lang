#[macro_use]
extern crate pest_derive;
#[macro_use]
extern crate serde_derive;

pub mod ast;
pub mod lexer;
pub mod parser;

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
