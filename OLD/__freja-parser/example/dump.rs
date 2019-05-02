use freja_parser::*;
use std::fs;
use pest::iterators::Pair;
use std::env;

fn format_pair<'a>(pair: Pair<'a, lexer::Rule>, indent_level: usize, is_newline: bool, full: bool) -> String {
    let indent = if is_newline {
        "  ".repeat(indent_level)
    } else {
        "".to_string()
    };

    let children: Vec<_> = pair.clone().into_inner().collect();
    let len = children.len();
    let children: Vec<_> = children.into_iter().map(|pair| {
        format_pair(pair, if len > 1 { indent_level + 1 } else { indent_level }, len > 1, full)
    }).collect();

    let dash = if is_newline {
        "- "
    } else {
        ""
    };

    match len {
        0 => format!("{}{}{:?}: {:?}", indent, dash, pair.as_rule(), pair.into_span().as_str()),
        1 => if full {
            format!("{}{}{:?} > {}", indent, dash, pair.as_rule(), children[0])
        } else {
            format!("{}{}{}", indent,dash,children[0])
        },
        _ => format!("{}{}{:?}\n{}", indent, dash, pair.as_rule(), children.join("\n"))
    }
}

fn main () {

    let mut args = env::args().skip(1).collect::<Vec<_>>();
    if (args.is_empty()) {
        println!("uage: dump (-f) <path>");
        return;
    }

    let full = args[0] == "-f";
    if full {
        args = args.into_iter().skip(1).collect();
    }
    
   
    let data = fs::read_to_string(&args[0]).unwrap();
    let tokens = lexer::tokenize(data.as_str()).unwrap();
    let lines: Vec<_> = tokens.map(|pair| {
                    format_pair(pair, 0, true, full)
                }).collect();
                let lines = lines.join("\n");

                println!("{}", lines);
}