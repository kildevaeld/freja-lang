// use super::ast::*;
use pest::error::Error;
use pest::iterators::Pairs;
use pest::Parser;

// macro_rules! location {
//     ($ctx:expr) => {
//         Location($ctx.as_span().start(), $ctx.as_span().end())
//     };
// }

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct FrejaLexer;

pub fn tokenize<'a>(data: &'a str) -> Result<Pairs<'a, Rule>, Error<Rule>> {
    Ok(FrejaLexer::parse(Rule::program, data)?)
}
