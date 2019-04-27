#[macro_use]
extern crate pest_derive;

#[macro_use]
extern crate serde_derive;

pub mod ast;
pub mod owned;
pub mod parser;
pub mod parser2;
pub mod traits;

pub fn create_ast<'a>(source: &'a str) -> ast::ProgramRef<'a> {
    let token = parser::lexer(source).unwrap();
    parser::parse(token)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
