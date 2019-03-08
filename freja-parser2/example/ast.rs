use freja_parser2::*;
use pest::iterators::Pair;
use serde_json;
use std::env;
use std::fs;

fn main() {
    let mut args = env::args().skip(1).collect::<Vec<_>>();
    if args.is_empty() {
        println!("uage: dump (-f) <path>");
        return;
    }
    let full = args[0] == "-f";
    if full {
        args = args.into_iter().skip(1).collect();
    }

    let data = fs::read_to_string(&args[0]).unwrap();
    let tokens = lexer::tokenize(data.as_str()).unwrap();

    let ast = parser::parse(tokens);

    let json = serde_json::to_string_pretty(&ast).unwrap();

    println!("{}", json);
}
