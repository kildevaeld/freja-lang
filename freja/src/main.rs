#[macro_use]
extern crate clap;

use freja_parser::*;
use serde_json;

use std::fs;

fn print_ast(input: &str, full: bool) {
    let data = fs::read_to_string(input).unwrap();
    let tokens = lexer::tokenize(data.as_str()).unwrap();
    let ast = parser::parse(tokens);
    let json = serde_json::to_string_pretty(&ast).unwrap();
    println!("{}", json);
}

fn format_pair<'a>(
    pair: Pair<'a, lexer::Rule>,
    indent_level: usize,
    is_newline: bool,
    full: bool,
) -> String {
    let indent = if is_newline {
        "  ".repeat(indent_level)
    } else {
        "".to_string()
    };

    let children: Vec<_> = pair.clone().into_inner().collect();
    let len = children.len();
    let children: Vec<_> = children
        .into_iter()
        .map(|pair| {
            format_pair(
                pair,
                if len > 1 {
                    indent_level + 1
                } else {
                    indent_level
                },
                len > 1,
                full,
            )
        })
        .collect();

    let dash = if is_newline { "- " } else { "" };

    match len {
        0 => format!(
            "{}{}{:?}: {:?}",
            indent,
            dash,
            pair.as_rule(),
            pair.into_span().as_str()
        ),
        1 => {
            if full {
                format!("{}{}{:?} > {}", indent, dash, pair.as_rule(), children[0])
            } else {
                format!("{}{}{}", indent, dash, children[0])
            }
        }
        _ => format!(
            "{}{}{:?}\n{}",
            indent,
            dash,
            pair.as_rule(),
            children.join("\n")
        ),
    }
}

fn print_dump(input: &str, full: bool) {
    let data = fs::read_to_string(input).unwrap();
    let tokens = lexer::tokenize(data.as_str()).unwrap();
    let lines: Vec<_> = tokens
        .map(|pair| format_pair(pair, 0, true, full))
        .collect();
    let lines = lines.join("\n");
    println!("{}", lines);
}

fn main() {
    let matches = clap_app!(freja =>
        (@subcommand ast =>
            (@arg full: -f "full")
            (@arg input: *  "input")
        )
        (@subcommand dump =>
            (@arg full: -f "full")
            (@arg input: *  "input")
        )
    )
    .get_matches();

    if let Some(matches) = matches.subcommand_matches("ast") {
        let input = matches.value_of("input").unwrap();
        print_ast(input, matches.is_present("full"));
    } else if let Some(matches) = matches.subcommand_matches("dump") {
        let input = matches.value_of("input").unwrap();
        print_dump(input, matches.is_present("full"));
    }
}
