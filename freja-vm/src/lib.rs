pub mod chunk;
pub mod compiler;
pub mod error;
pub mod objects;
pub mod value;
pub mod vm;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}