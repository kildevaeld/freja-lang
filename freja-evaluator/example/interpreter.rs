use freja_evaluator::VM;
use freja_parser::create_ast;

fn main() {
    let ast = create_ast(
        r#"
        let test = "main", mig = 200.0;
        let test2 = 10 + 2 - 11 * 2 + 8;

        print(test2)

        fn fib(num) {
            if num <= 1 {
                return 1;
            }
            return fib(num - 1) + fib(num - 2);
        }
    "#,
    );

    let mut vm = VM::new();
    vm.interpret(&ast).unwrap();

    println!("{:?}", vm);
}
