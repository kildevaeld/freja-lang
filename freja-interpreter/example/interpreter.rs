use freja_interpreter::{Interpreter, VM};
use freja_parser::create_ast;

fn main() {
    let ast = create_ast(
        r#"
        let test = "main";
    "#,
    );

    let mut vm = freja_interpreter::VM::new();

    vm.interpret_ref(ast).unwrap();

    println!("{:?}", vm);
}
