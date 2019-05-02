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
#[grammar = "grammar.pest"]
pub struct FrejaLexer;

pub fn lexer<'a>(data: &'a str) -> Result<Pair<'a, Rule>, Error<Rule>> {
    let tokens = FrejaLexer::parse(Rule::program, data)?.next().unwrap();
    Ok(tokens)
}

fn parse_variable_declaration<'a>(pair: Pair<'a, Rule>) -> StmtRef<'a> {
    if pair.as_rule() != Rule::variable_declaration {
        panic!("not a variable declation");
    }

    let mut inner = pair.into_inner();
    let assign = inner.next().unwrap();
    if assign.as_rule() != Rule::assignment {
        panic!("not a assignment");
    }

    let mut inner = assign.into_inner();

    let ident = inner.next().unwrap();
    let expr = parse_expr(inner.next().unwrap());

    StmtRef::Var(VarStmtRef {
        name: TokenRef::new(location!(ident), ident.as_str()),
        initializer: expr,
    })
}

fn parse_func_decl<'a>(pair: Pair<'a, Rule>) -> StmtRef<'a> {
    if pair.as_rule() != Rule::fun_decl {
        panic!("not a function declation {:?}", pair.as_rule());
    }
    let mut inner = pair.into_inner();
    let next = inner.next().unwrap();

    StmtRef::Func(parse_function(next))
}

fn parse_string<'a>(pair: Pair<'a, Rule>) -> ExprRef<'a> {
    if pair.as_rule() != Rule::string_literal && pair.as_rule() != Rule::multi_line_string_literal {
        panic!(format!("not a string literal: {:?}", pair.as_rule()));
    }
    let s = pair.as_str();
    ExprRef::Literal(LiteralExprRef {
        location: location!(pair),
        value: LiteralRef::String(&pair.as_str()[1..s.len() - 1]),
    })
}

fn parse_argument<'a>(pair: Pair<'a, Rule>) -> Vec<Box<ExprRef<'a>>> {
    if pair.as_rule() != Rule::arguments {
        panic!(format!("not a string literal: {:?}", pair.as_rule()));
    }
    pair.into_inner().map(|m| Box::new(parse_expr(m))).collect()
}

fn parse_function<'a>(pair: Pair<'a, Rule>) -> FuncStmtRef<'a> {
    if pair.as_rule() != Rule::function {
        panic!(format!("not a function: {:?}", pair.as_rule()));
    }

    // let span = pair.clone().into_span();
    // // A pair is a combination of the rule which matched and a span of input
    // println!("Rule:    {:?}", pair.as_rule());
    // println!("Span:    {:?}", span);
    // println!("Text:    {}", span.as_str());

    // // A pair can be converted to an iterator of the tokens which make it up:
    // for inner_pair in pair.into_inner() {
    //     let inner_span = inner_pair.clone().into_span();
    //     println!("  Rule:    {:?}", inner_pair.as_rule());
    //     println!("  Span:    {:?}", inner_span);
    //     println!("  Text:    {}", inner_span.as_str());
    // }

    // panic!("none");
    let mut inner = pair.into_inner();
    let name = inner.next().unwrap();
    let name = TokenRef::new(location!(name), name.as_str());

    let next = inner.next().unwrap();
    let (parameters, body) = if next.as_rule() == Rule::parameters {
        (
            parse_parameters(next),
            parse_block_stmt(inner.next().unwrap()),
        )
    } else {
        (Vec::new(), parse_block_stmt(next))
    };

    FuncStmtRef {
        name,
        body: Box::new(StmtRef::Block(body)),
        parameters,
    }
}

fn parse_identifier_expr<'a>(pair: Pair<'a, Rule>) -> ExprRef<'a> {
    if pair.as_rule() != Rule::identifier {
        panic!(format!("not a identifier: {:?}", pair.as_rule()));
    }

    ExprRef::Identifier(IdentifierExprRef {
        location: location!(pair),
        token: TokenRef::new(location!(pair), pair.as_str()),
    })
}

fn parse_call<'a>(pair: Pair<'a, Rule>) -> ExprRef<'a> {
    let location = location!(pair);
    let mut inner = pair.into_inner();
    let mut calle = Box::new(parse_expr(inner.next().unwrap()));
    let arguments = inner.next().unwrap();
    let (args, mut parent) = match arguments.as_rule() {
        Rule::arguments => (parse_argument(arguments), None),
        _ => (
            parse_argument(inner.next().unwrap()),
            Some(Box::new(parse_expr(arguments))),
        ),
    };

    if parent.is_some() {
        let tmp = calle;
        calle = parent.unwrap();
        parent = Some(tmp);
    }

    ExprRef::Call(CallExprRef {
        location: location,
        callee: calle,
        arguments: args,
        parent: parent,
    })
}

pub fn parse_block_stmt<'a>(pair: Pair<'a, Rule>) -> BlockStmtRef<'a> {
    if pair.as_rule() != Rule::block {
        panic!(format!("not a block: {:?}", pair.as_rule()));
    }

    let body = pair.into_inner().map(parse_stmt).collect();

    BlockStmtRef { statements: body }
}

pub fn parse_parameters<'a>(pair: Pair<'a, Rule>) -> Vec<ExprRef<'a>> {
    if pair.as_rule() != Rule::parameters {
        panic!(format!("not parameters: {:?}", pair.as_rule()));
    }

    pair.into_inner().map(parse_expr).collect()
}

pub fn parse_number_expr<'a>(pair: Pair<'a, Rule>) -> ExprRef<'a> {
    let n = match pair.as_rule() {
        Rule::float => NumberRef::Float(pair.as_str()),
        _ => panic!("invalid number"),
    };
    ExprRef::Literal(LiteralExprRef {
        location: location!(pair),
        value: LiteralRef::Number(n),
    })
}

pub fn parse_expr<'a>(pair: Pair<'a, Rule>) -> ExprRef<'a> {
    match pair.as_rule() {
        Rule::multi_line_string_literal | Rule::string_literal => parse_string(pair),
        Rule::float => parse_number_expr(pair),
        Rule::call => parse_call(pair),
        Rule::identifier => parse_identifier_expr(pair),
        _ => unimplemented!("parse_expr: {:?}", pair.as_rule()),
    }
}

pub fn parse_statement_stmt<'a>(pair: Pair<'a, Rule>) -> StmtRef<'a> {
    let mut inner = pair.into_inner();
    parse_stmt(inner.next().unwrap())
}

pub fn parse_expr_stmt<'a>(pair: Pair<'a, Rule>) -> ExprStmtRef<'a> {
    if pair.as_rule() != Rule::expr_stmt {
        panic!(format!("not parameters: {:?}", pair.as_rule()));
    }
    ExprStmtRef {
        expression: parse_expr(pair.into_inner().next().unwrap()),
    }
}

pub fn parse_return_stmt<'a>(pair: Pair<'a, Rule>) -> ReturnStmtRef<'a> {
    let e = match pair.into_inner().next() {
        Some(m) => Some(parse_expr(m)),
        None => None,
    };
    ReturnStmtRef { expression: e }
}

pub fn parse_stmt<'a>(pair: Pair<'a, Rule>) -> StmtRef<'a> {
    match pair.as_rule() {
        Rule::return_stmt => StmtRef::Return(parse_return_stmt(pair)),
        Rule::statement => parse_statement_stmt(pair),
        Rule::variable_declaration => parse_variable_declaration(pair),
        Rule::fun_decl => parse_func_decl(pair),
        Rule::block => StmtRef::Block(parse_block_stmt(pair)),
        Rule::expr_stmt => StmtRef::Expr(parse_expr_stmt(pair)),
        _ => {
            unimplemented!("parse_stmt {:?}", pair.as_rule());
        }
    }
}

pub fn parse<'a>(pair: Pair<'a, Rule>) -> ProgramRef<'a> {
    let mut output = Vec::new();

    // A pair can be converted to an iterator of the tokens which make it up:
    for inner_pair in pair.into_inner() {
        let stmt = match inner_pair.as_rule() {
            Rule::variable_declaration => parse_variable_declaration(inner_pair),
            Rule::fun_decl => parse_func_decl(inner_pair),
            Rule::statement => parse_statement_stmt(inner_pair),
            Rule::block => StmtRef::Block(parse_block_stmt(inner_pair)),
            Rule::EOI => break,
            _ => {
                let out = format!("should not happen: {:?}", inner_pair.as_rule());
                unimplemented!("{}", out);
            }
        };
        output.push(stmt);
    }
    ProgramRef { statements: output }
}

#[cfg(test)]
mod tests {

    use super::lexer;

    #[test]
    fn number() {
        assert!(lexer("20").is_ok());
        assert!(lexer("1000").is_ok());
        assert!(lexer("2.0").is_ok());
        assert!(lexer("0.44345").is_ok());
        assert!(lexer("-0.44345").is_ok());
    }

    #[test]
    fn constants() {
        assert!(lexer("true").is_ok());
        assert!(lexer("false").is_ok());
    }

    #[test]
    fn fn_decl() {
        assert!(lexer("fn test() {}").is_ok());
        assert!(lexer("fn test(param1, param2) {}").is_ok());
        assert!(lexer("fn test(param1, param2) { return; }").is_ok());
        //assert!(lexer("fntest(param1, param2) {}").is_err());
    }

    #[test]
    fn variables() {
        assert!(lexer("let test").is_ok());
        assert!(lexer("let test = 20").is_ok());
        assert!(lexer("let test = true").is_ok());
        assert!(lexer("let test = false").is_ok());
    }

    #[test]
    fn full() {
        let m = r#"
            let test = (2 + 10) / 2 * 20;
            test = test > 20;
            let ost = 'Hello, World';
            fn test(){}

            test(hello, world); test;
            test(); 

            class Test {
                test() {

                }
            }
        "#;
        lexer(m).unwrap();
        assert!(lexer(m).is_ok());
    }

}
