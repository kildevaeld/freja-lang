use super::ast::*;
use pest::error::Error;
use pest::iterators::Pair;
use pest::Parser;

macro_rules! location {
    ($ctx:expr) => {
        Location($ctx.as_span().start(), $ctx.as_span().end())
    };
}

#[derive(Parser)]
#[grammar = "grammar2.pest"]
pub struct FrejaLexer2;







pub fn parse_stmt<'a>(pair:Pair<'a, Rule>) -> StmtRef<'a> {
    
}