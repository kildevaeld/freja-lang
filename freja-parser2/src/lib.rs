#[macro_use]
extern crate pest_derive;

pub mod lexer;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
