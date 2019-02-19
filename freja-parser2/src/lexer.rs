// use super::ast::*;
use pest::error::Error;
use pest::iterators::Pair;
use pest::Parser;

// macro_rules! location {
//     ($ctx:expr) => {
//         Location($ctx.as_span().start(), $ctx.as_span().end())
//     };
// }

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct FrejaLexer;


