use freja_parser2::*;
use std::fs;
use pest::iterators::Pair;


fn format_pair<'a>(pair: Pair<'a, lexer::Rule>, indent_level: usize, is_newline: bool) -> String {
    let indent = if is_newline {
        "  ".repeat(indent_level)
    } else {
        "".to_string()
    };

    let children: Vec<_> = pair.clone().into_inner().collect();
    let len = children.len();
    let children: Vec<_> = children.into_iter().map(|pair| {
        format_pair(pair, if len > 1 { indent_level + 1 } else { indent_level }, len > 1)
    }).collect();

    let dash = if is_newline {
        "- "
    } else {
        ""
    };

    match len {
        0 => format!("{}{}{:?}: {:?}", indent, dash, pair.as_rule(), pair.into_span().as_str()),
        1 => format!("{}{}{:?} > {}", indent, dash, pair.as_rule(), children[0]),
        _ => format!("{}{}{:?}\n{}", indent, dash, pair.as_rule(), children.join("\n"))
    }
}

fn main () {
    let data = fs::read_to_string("test.fr").unwrap();
    let tokens = lexer::tokenize(data.as_str()).unwrap();
    let lines: Vec<_> = tokens.map(|pair| {
                    format_pair(pair, 0, true)
                }).collect();
                let lines = lines.join("\n");

                println!("{}", lines);
}