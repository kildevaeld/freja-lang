use freja_parser::*;

fn main() {
    let source = r#"
        fn function(arg) {
            return 200
        }
        let test = "Hello, World"
        function(test);
        let ret = function("Hello", test);
    "#;

    let ast = create_ast(source);

    let ast = serde_json::to_string_pretty(&ast).unwrap();
    println!("{}", ast);
}
