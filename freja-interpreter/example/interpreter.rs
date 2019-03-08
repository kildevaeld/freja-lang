use freja_interpreter::{Interpreter, VM};
use freja_parser2::create_ast;

fn main() {
    let ast = create_ast(
        r#"
        let test = "main";
        fn testFn(args) {
            
        }
    "#,
    );

    let mut vm = freja_interpreter::VM::new();
    vm.interpret(&ast).unwrap();

    println!("{:?}", vm);
}
