use freja_parser::*;
use std::env;
use std::fs;
use serde_json;

fn main() {
    let mut args = env::args().skip(1).collect::<Vec<_>>();
    if args.is_empty() {
        println!("uage: dump (-f) <path>");
        return;
    }

    let data = fs::read_to_string(&args[0]).unwrap();

    let ast = parser::program(data.as_str()).expect("parser");
    let json = serde_json::to_string_pretty(&ast).unwrap();
    println!("{}", json);
}
