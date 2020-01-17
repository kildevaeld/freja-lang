use std::alloc::System;
#[global_allocator]
static ALLOCATOR: System = System;


use freja_compiler::*;
use freja_parser::*;
use freja_vm::*;
use getopts::Options;
#[cfg(feature = "serializing")]
use serde_json;
use std::env;
use std::fs;

#[cfg(feature = "serializing")]
fn print_ast(input: &str) {
    let data = fs::read_to_string(input).unwrap();
    let ast = parser::program(&data).expect("could not parse");
    let json = serde_json::to_string_pretty(&ast).unwrap();
    println!("{}", json);
}

#[cfg(not(feature = "serializing"))]
fn print_ast(_input: &str) {}

fn print_bytecode(input: &str) {
    let data = fs::read_to_string(input).unwrap();
    let ast = parser::program(&data).expect("could not parse");
    let ret = Compiler::new().compile(&ast).expect("compile");
    println!("{}", ret.chunk().dissamble(true));
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

fn run(input: &str) {
    let data = fs::read_to_string(input).unwrap();

    let vm = vm::VM::new();

    vm.eval_script(data).expect("eval script");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("o", "", "set output file name", "NAME")
        .optflag("h", "help", "print this help menu")
        .optflag("b", "bytecode", "print bytecode");

    if cfg!(feature = "serializing") {
        opts.optflag("a", "ast", "print ast");
    }

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    let _output = matches.opt_str("o");
    let input = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        print_usage(&program, opts);
        return;
    };

    if cfg!(feature = "serializing") && matches.opt_present("a") {
        print_ast(&input);
    } else if matches.opt_present("b") {
        print_bytecode(&input);
    } else {
        run(&input);
    }
}
