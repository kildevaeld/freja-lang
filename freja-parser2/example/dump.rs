use freja_parser2::*;
use std::env;
use std::fs;


fn main() {
    let mut args = env::args().skip(1).collect::<Vec<_>>();
    if args.is_empty() {
        println!("uage: dump (-f) <path>");
        return;
    }

    let data = fs::read_to_string(&args[0]).unwrap();

    let ast = parser::program(data.as_str()).expect("parser");
    
    println!("{:?}", ast);
}
