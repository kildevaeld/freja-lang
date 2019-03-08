use pest::error::Error;
use pest::iterators::Pairs;
use pest::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct FrejaLexer;

pub fn tokenize<'a>(data: &'a str) -> Result<Pairs<'a, Rule>, Error<Rule>> {
    Ok(FrejaLexer::parse(Rule::program, data)?)
}
