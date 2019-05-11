use std::alloc::System;
#[global_allocator]
static ALLOCATOR: System = System;

use freja_parser::*;
use freja_vm::compiler::*;
use freja_vm::value::Value;
use freja_vm::vm::VM;
use getopts::Options;
#[cfg(feature = "serializing")]
use serde_cbor;
#[cfg(feature = "serializing")]
use serde_json;
use std::env;
use std::fs;
use std::io::BufRead;

#[cfg(feature = "serializing")]
fn print_ast(input: &str) {
    let data = fs::read_to_string(input).unwrap();
    let ast = parser::program(&data).expect("could not parse");
    let json = serde_json::to_string_pretty(&ast).unwrap();
    println!("{}", json);
}

#[cfg(not(feature = "serializing"))]
fn print_ast(_input: &str) {}

#[cfg(feature = "serializing")]
fn dump(input: &str, output: Option<String>) {
    let data = fs::read_to_string(input).unwrap();
    let ast = parser::program(&data).expect("could not parse");
    let ret = Compiler::new().compile(&ast).expect("compile");
    if output.is_some() {
        let mut file = fs::File::create(output.unwrap()).expect("open file");
        serde_cbor::to_writer(&mut file, ret.chunk()).unwrap();
    } else {
        let json = serde_json::to_string_pretty(ret.chunk()).unwrap();
        println!("{}", json);
    }
}

#[cfg(not(feature = "serializing"))]
fn dump(_input: &str, _output: Option<String>) {}

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
    let ast = parser::program(&data).expect("could not parse");
    //let ret = Compiler::new().compile(&ast).expect("compile");
    let mut vm = VM::new();
    vm.push_native("print", |vals| {
        let v = vals.iter().map(|m| m.to_string()).collect::<Vec<_>>().join(", ");
        print!("{}", v);
        Ok(Value::Null)
    });
    vm.push_native("println", |vals| {
        let v = vals.iter().map(|m| m.to_string()).collect::<Vec<_>>().join(", ");
        println!("{}", v);
        Ok(Value::Null)
    });
    vm.push_native("readinput", |vals| {
        let mut out = String::new();
        std::io::stdin().lock().read_line(&mut out).unwrap();
        Ok(Value::Null)
    });
    vm.interpret_ast(&ast).unwrap();

    //println!("{}", vm.dump());
    //println!("{}", vm.dump_global());
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
        opts.optflag("d", "dump", "dump bytecode");
    }

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

    if cfg!(feature = "serializing") && matches.opt_present("a") {
        print_ast(&input);
    } else if cfg!(feature = "serializing") && matches.opt_present("d") {
        dump(&input, output);
    } else if matches.opt_present("b") {
        print_bytecode(&input);
    } else {
        run(&input);
    }
}
