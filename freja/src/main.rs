use freja_parser::*;
use freja_vm::compiler::*;
use freja_vm::vm::VM;
use getopts::Options;
use serde_json;
use std::env;
use std::fs;

fn print_ast(input: &str) {
    let data = fs::read_to_string(input).unwrap();
    let ast = parser::program(&data).expect("could not parse");
    let json = serde_json::to_string_pretty(&ast).unwrap();
    println!("{}", json);
}

fn print_bytecode(input: &str) {
    let data = fs::read_to_string(input).unwrap();
    let ast = parser::program(&data).expect("could not parse");
    let ret = Compiler::new().compile(&ast).expect("compile");
    println!("{}", ret);
    //println!("{}", vm.dump());
    //println!("{}", vm.dump_global());
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

fn run(input: &str) {
    let data = fs::read_to_string(input).unwrap();
    let ast = parser::program(&data).expect("could not parse");
    //let ret = Compiler::new().compile(&ast).expect("compile");
    let mut vm = VM::new();
    vm.push_native("print", |vals| {
        let v = vals.iter().map(|m| m.to_string()).collect::<Vec<_>>().join(", ");
        println!("{}", v);
    });
    vm.interpret_ast(&ast).unwrap();

    //println!("{}", vm.dump());
    //println!("{}", vm.dump_global());
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("o", "", "set output file name", "NAME").optflag("h", "help", "print this help menu").optflag("b", "bytecode", "print bytecode").optflag("a", "ast", "print ast");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    let output = matches.opt_str("o");
    let input = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        print_usage(&program, opts);
        return;
    };

    if matches.opt_present("a") {
        print_ast(&input);
    } else if matches.opt_present("b") {
        print_bytecode(&input);
    } else {
        run(&input);
    }
}