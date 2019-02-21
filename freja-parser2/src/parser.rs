use super::ast::*;
use super::lexer::*;
use pest::iterators::{Pair, Pairs};

macro_rules! ensure_rule {
    ($pair:expr, $rule: expr) => {
        if $pair.as_rule() != $rule {
            panic!("expected rule: {:?}, found: {:?}", $rule, $pair.as_rule());
        }
    };
}

macro_rules! location {
    ($ctx:expr) => {
        Location($ctx.as_span().start(), $ctx.as_span().end())
    };
}

macro_rules! make_token {
    ($pair: expr) => {{
        let loc = location!($pair);
        Token::new(loc, $pair.as_span().as_str())
    }};
}

macro_rules! dump_pair {
    ($pair:expr) => {
        println!("Rule:    {:?}", $pair.as_rule());
        println!("Text:    {}", $pair.as_str());
    };
}

macro_rules! dump_inner_pair {
    ($pair:expr) => {
        for p in $pair.clone().into_inner() {
            println!("Rule:    {:?}", p.as_rule());
            println!("Text:    {}", p.as_str());
        }
    };
}

macro_rules! token_from_rule {
    ($rule: expr) => {
        match $rule {
            Rule::THIS_TOKEN => TokenType::This,
            Rule::IDENTIFIER => TokenType::Identifier,
            Rule::IDENTIFIER_NAME => TokenType::Identifier,
            Rule::ADDITIVE_OPERATOR => TokenType::OpAdditive,
            Rule::MULTIPLICATIVE_OPERATOR => TokenType::OpMultiplicative,
            _ => unreachable!("should not reach {:?}", $rule),
        }
    };
}

fn parse_literal<'a>(pair: Pair<'a, Rule>) -> Expr<'a> {
    let location = location!(pair);
    let inner = pair.into_inner().next().unwrap();

    let lit = match inner.as_rule() {
        Rule::string_literal => Literal::String(inner.as_str()),
        Rule::numeric_literal => {
            let p = inner.into_inner().next().unwrap();
            let n = match p.as_rule() {
                Rule::float => Number::Double(p.as_str()),
                Rule::integer => Number::Integer(p.as_str()),
                _ => unreachable!("should not be reached"),
            };
            Literal::Number(n)
        }
        _ => unreachable!("should net be reached {:?}", inner.as_rule()),
    };
    Expr::Literal(LiteralExpr {
        location,
        value: lit,
    })
}

// Arguments
#[inline(always)]
fn parse_arguments_expr<'a>(pair: Pair<'a, Rule>) -> Expr<'a> {
    let location = location!(pair);
    Expr::Arguments(ArgumentsExpr {
        location,
        expressions: pair
            .into_inner()
            .map(|m| Box::new(parse_expression(m)))
            .collect(),
    })
    //unimplemented!("{:?}", pair);
}

// Expressions
#[inline(always)]
fn parse_expression<'a>(pair: Pair<'a, Rule>) -> Expr<'a> {
    match pair.as_rule() {
        Rule::assignment_expr => parse_assign_expr(pair),
        Rule::logical_or_expr => parse_logic_or_expr(pair),
        Rule::logical_and_expr => parse_logic_and_expr(pair),
        Rule::bitwise_or_expr => parse_bitwise_or_expr(pair),
        Rule::bitwise_xor_expr => parse_bitwise_xor_expr(pair),
        Rule::bitwise_and_expr => parse_bitwise_and_expr(pair),
        // Rule::equality_expr => parse_equality_expr(pair),
        // Rule::relational_expr => parse_relational_expr(pair),
        // Rule::shift_expr => parse_shift_expr(pair),
        // Rule::additive_expr => parse_additive_expr(pair),
        // Rule::multiplicative_expr => parse_multiplicative_expr(pair),
        Rule::equality_expr
        | Rule::relational_expr
        | Rule::shift_expr
        | Rule::additive_expr
        | Rule::multiplicative_expr => parse_binray_expr(pair),

        Rule::unary_expr => parse_unary_expr(pair),
        Rule::postfix_expr => parse_postfix_expr(pair),
        Rule::left_hand_side_expr => parse_left_hand_side_expr(pair),
        Rule::call_expr => parse_call_expr(pair),
        Rule::member_expr => parse_member_expr(pair),
        Rule::primary_expr => parse_primary_expr(pair),
        Rule::arguments => parse_arguments_expr(pair),
        Rule::literal => parse_literal(pair),
        Rule::THIS_TOKEN | Rule::IDENTIFIER | Rule::IDENTIFIER_NAME => Expr::Lookup(LookupExpr {
            location: location!(pair),
            token: Token::new(
                location!(pair),
                token_from_rule!(pair.as_rule()),
                pair.as_span().as_str(),
            ),
        }),
        _ => unimplemented!("{:?}", pair.as_rule()),
    }
}

#[inline(always)]
fn parse_assign_expr<'a>(pair: Pair<'a, Rule>) -> Expr<'a> {
    let mut children = pair
        .clone()
        .into_inner()
        .map(|m| parse_expression(m))
        .collect::<Vec<_>>();

    if children.len() == 1 {
        return children.pop().take().expect("assign child");
    }

    unimplemented!("parse {:?}", pair.as_rule());
}

#[inline(always)]
fn parse_logic_or_expr<'a>(pair: Pair<'a, Rule>) -> Expr<'a> {
    let mut children = pair
        .clone()
        .into_inner()
        .map(|m| parse_expression(m))
        .collect::<Vec<_>>();

    if children.len() == 1 {
        return children.pop().take().expect("logic or child");
    }

    unimplemented!("parse {:?}", pair.as_rule());
}

#[inline(always)]
fn parse_logic_and_expr<'a>(pair: Pair<'a, Rule>) -> Expr<'a> {
    let mut children = pair
        .clone()
        .into_inner()
        .map(|m| parse_expression(m))
        .collect::<Vec<_>>();

    if children.len() == 1 {
        return children.pop().take().expect("logic and child");
    }

    unimplemented!("parse {:?}", pair.as_rule());
}

#[inline(always)]
fn parse_bitwise_and_expr<'a>(pair: Pair<'a, Rule>) -> Expr<'a> {
    let mut children = pair
        .clone()
        .into_inner()
        .map(|m| parse_expression(m))
        .collect::<Vec<_>>();

    if children.len() == 1 {
        return children.pop().take().expect("butise and or child");
    }

    unimplemented!("parse {:?}", pair.as_rule());
}

#[inline(always)]
fn parse_bitwise_or_expr<'a>(pair: Pair<'a, Rule>) -> Expr<'a> {
    let mut children = pair
        .clone()
        .into_inner()
        .map(|m| parse_expression(m))
        .collect::<Vec<_>>();

    if children.len() == 1 {
        return children.pop().take().expect("bitwise or or child");
    }
    unimplemented!("parse {:?}", pair.as_rule());
}

#[inline(always)]
fn parse_bitwise_xor_expr<'a>(pair: Pair<'a, Rule>) -> Expr<'a> {
    let mut children = pair
        .clone()
        .into_inner()
        .map(|m| parse_expression(m))
        .collect::<Vec<_>>();

    if children.len() == 1 {
        return children.pop().take().expect("bitwise xor child");
    }

    unimplemented!("parse {:?}", pair.as_rule());
}

// #[inline(always)]
// fn parse_equality_expr<'a>(pair: Pair<'a, Rule>) -> Expr<'a> {
//     let mut children = pair
//         .clone()
//         .into_inner()
//         .map(|m| parse_expression(m))
//         .collect::<Vec<_>>();

//     if children.len() == 1 {
//         return children.pop().take().expect("equality child");
//     }
//     unimplemented!("parse {:?}", pair.as_rule());
// }

// #[inline(always)]
// fn parse_relational_expr<'a>(pair: Pair<'a, Rule>) -> Expr<'a> {
//     let mut children = pair
//         .clone()
//         .into_inner()
//         .map(|m| parse_expression(m))
//         .collect::<Vec<_>>();

//     if children.len() == 1 {
//         return children.pop().take().expect("relational or child");
//     }

//     unimplemented!("parse {:?}", pair.as_rule());
// }

// #[inline(always)]
// fn parse_shift_expr<'a>(pair: Pair<'a, Rule>) -> Expr<'a> {
//     let mut children = pair
//         .clone()
//         .into_inner()
//         .map(|m| parse_expression(m))
//         .collect::<Vec<_>>();

//     if children.len() == 1 {
//         return children.pop().take().expect("shift or child");
//     }
//     unimplemented!("parse {:?}", pair.as_rule());
// }

// #[inline(always)]
// fn parse_additive_expr<'a>(pair: Pair<'a, Rule>) -> Expr<'a> {
//     let location = location!(pair);
//     let mut children = pair.into_inner().collect::<Vec<_>>();

//     if children.len() == 1 {
//         return children
//             .pop()
//             .take()
//             .map(|m| parse_expression(m))
//             .expect("multiplicative child");
//     }

//     let right = children
//         .pop()
//         .map(|m| Box::new(parse_expression(m)))
//         .take()
//         .expect("right");
//     let operator = children
//         .pop()
//         .map(|m| match m.as_rule() {
//             Rule::ADDITIVE_OPERATOR | Rule::MULTIPLICATIVE_OPERATOR => {
//                 Token::new(location!(m), token_from_rule!(m.as_rule()), m.as_str())
//             }
//             _ => unreachable!("not a operator {:?}", m.as_rule()),
//         })
//         .take()
//         .expect("operator");
//     let left = children
//         .pop()
//         .map(|m| Box::new(parse_expression(m)))
//         .take()
//         .expect("left");

//     Expr::Binary(BinaryExpr {
//         location,
//         left,
//         right,
//         operator,
//     })
// }

// #[inline(always)]
// fn parse_multiplicative_expr<'a>(pair: Pair<'a, Rule>) -> Expr<'a> {
//     let location = location!(pair);
//     let mut children = pair.into_inner().collect::<Vec<_>>();

//     if children.len() == 1 {
//         return children
//             .pop()
//             .take()
//             .map(|m| parse_expression(m))
//             .expect("multiplicative child");
//     }

//     let right = children
//         .pop()
//         .map(|m| Box::new(parse_expression(m)))
//         .take()
//         .expect("right");
//     let operator = children
//         .pop()
//         .map(|m| match m.as_rule() {
//             Rule::ADDITIVE_OPERATOR | Rule::MULTIPLICATIVE_OPERATOR => {
//                 Token::new(location!(m), token_from_rule!(m.as_rule()), m.as_str())
//             }
//             _ => unreachable!("not a operator {:?}", m.as_rule()),
//         })
//         .take()
//         .expect("operator");
//     let left = children
//         .pop()
//         .map(|m| Box::new(parse_expression(m)))
//         .take()
//         .expect("left");

//     Expr::Binary(BinaryExpr {
//         location,
//         left,
//         right,
//         operator,
//     })
// }

#[inline(always)]
fn parse_binray_expr<'a>(pair: Pair<'a, Rule>) -> Expr<'a> {
    let location = location!(pair);
    let mut children = pair.into_inner().collect::<Vec<_>>();

    if children.len() == 1 {
        return children
            .pop()
            .take()
            .map(|m| parse_expression(m))
            .expect("multiplicative child");
    }

    let right = children
        .pop()
        .map(|m| Box::new(parse_expression(m)))
        .take()
        .expect("right");
    let operator = children
        .pop()
        .map(|m| match m.as_rule() {
            Rule::ADDITIVE_OPERATOR | Rule::MULTIPLICATIVE_OPERATOR => {
                Token::new(location!(m), token_from_rule!(m.as_rule()), m.as_str())
            }
            _ => unreachable!("not a operator {:?}", m.as_rule()),
        })
        .take()
        .expect("operator");
    let left = children
        .pop()
        .map(|m| Box::new(parse_expression(m)))
        .take()
        .expect("left");

    Expr::Binary(BinaryExpr {
        location,
        left,
        right,
        operator,
    })
}

#[inline(always)]
fn parse_unary_expr<'a>(pair: Pair<'a, Rule>) -> Expr<'a> {
    let mut children = pair
        .clone()
        .into_inner()
        .map(|m| parse_expression(m))
        .collect::<Vec<_>>();

    if children.len() == 1 {
        return children.pop().take().expect("unary or child");
    }

    unimplemented!("parse {:?}", pair.as_rule());
}

#[inline(always)]
fn parse_postfix_expr<'a>(pair: Pair<'a, Rule>) -> Expr<'a> {
    let mut children = pair
        .clone()
        .into_inner()
        .map(|m| parse_expression(m))
        .collect::<Vec<_>>();

    if children.len() == 1 {
        return children.pop().take().expect("postfix or child");
    }

    unimplemented!("parse {:?}", pair.as_rule());
}

#[inline(always)]
fn parse_left_hand_side_expr<'a>(pair: Pair<'a, Rule>) -> Expr<'a> {
    let mut children = pair
        .clone()
        .into_inner()
        .map(|m| parse_expression(m))
        .collect::<Vec<_>>();

    if children.len() == 1 {
        return children.pop().take().expect("lefthand or child");
    }

    unimplemented!("parse {:?}", pair.as_rule());
}

#[inline(always)]
fn parse_call_expr<'a>(pair: Pair<'a, Rule>) -> Expr<'a> {
    let location = location!(pair);
    let mut children = pair
        .clone()
        .into_inner()
        .map(|m| parse_expression(m))
        .collect::<Vec<_>>();

    if children.len() == 1 {
        return children.pop().take().expect("call child");
    }

    Expr::Call(CallExpr {
        location,
        member: Box::new(children.remove(0)),
        arguments: Box::new(children.remove(0)),
    })
}

#[inline(always)]
fn parse_member_expr<'a>(pair: Pair<'a, Rule>) -> Expr<'a> {
    let mut children = pair
        .clone()
        .into_inner()
        .map(|m| parse_expression(m))
        .collect::<Vec<_>>();

    if children.len() == 1 {
        return children.pop().take().unwrap();
    }

    let object = children.remove(0);
    let access = children.remove(0);

    let e = children.into_iter().fold(
        MemberExpr {
            location: location!(pair),
            object: Box::new(object),
            property: Box::new(access),
            computed: false,
        },
        |acc, x| MemberExpr {
            location: acc.location.clone(),
            object: Box::new(Expr::Member(acc)),
            property: Box::new(x),
            computed: false,
        },
    );
    Expr::Member(e)

    // unimplemented!("parse {:?}", pair.as_rule());
}

#[inline(always)]
fn parse_primary_expr<'a>(pair: Pair<'a, Rule>) -> Expr<'a> {
    let next = pair.into_inner().next().expect("primary");

    let p = match next.as_rule() {
        Rule::THIS_TOKEN | Rule::IDENTIFIER | Rule::IDENTIFIER_NAME => Expr::Lookup(LookupExpr {
            location: location!(next),
            token: Token::new(
                location!(next),
                token_from_rule!(next.as_rule()),
                next.as_span().as_str(),
            ),
        }),
        _ => parse_expression(next),
    };

    p
    //parse_expression(pair)
}

// Statements

#[inline(always)]
fn parse_class_decl<'a>(pair: Pair<'a, Rule>) -> Stmt<'a> {
    ensure_rule!(pair, Rule::class_decl);
    let location = location!(pair);
    let inner = pair.into_inner();
    let mut members = Vec::new();
    let mut name = None;
    for inner_pair in inner {
        if inner_pair.as_rule() == Rule::IDENTIFIER {
            name = Some(Token::new(
                location!(inner_pair),
                TokenType::Identifier,
                inner_pair.as_span().as_str(),
            ));
            continue;
        }
        let stmt = match inner_pair.as_rule() {
            Rule::func_decl => parse_func_decl(inner_pair),
            _ => {
                unimplemented!("parse {:?}", inner_pair.as_rule());
            }
        };
        members.push(Box::new(stmt));
    }

    if name.is_none() {
        panic!("class has no name");
    }

    Stmt::Class(ClassStmt {
        location,
        members,
        name: name.take().expect("class name"),
    })
}

#[inline(always)]
fn parse_func_decl<'a>(pair: Pair<'a, Rule>) -> Stmt<'a> {
    ensure_rule!(pair, Rule::func_decl);
    let location = location!(pair);
    let mut inner = pair.clone().into_inner();

    let name = inner.next().expect("function name");
    let name = Token::new(location!(name), TokenType::Identifier, name.as_str());
    let parameters = inner
        .next()
        .expect("function params")
        .into_inner()
        .map(|pair| match pair.as_rule() {
            Rule::IDENTIFIER => Argument::Regular(pair.as_str()),
            Rule::formal_parameter_item_vaargs => Argument::Rest(&pair.as_str()[3..]),
            _ => unreachable!("invalid argument"),
        })
        .collect::<Vec<_>>();
    let body = parse_statement(inner.next().expect("func decl body"));
    Stmt::Func(FuncStmt {
        location,
        name,
        parameters,
        body: Box::new(body),
    })
}

#[inline(always)]
fn parse_block_stmt<'a>(pair: Pair<'a, Rule>) -> Stmt<'a> {
    ensure_rule!(pair, Rule::block);
    let location = location!(pair);
    let statements = pair
        .into_inner()
        .map(|pair| Box::new(parse_statement(pair)))
        .collect::<Vec<_>>();
    Stmt::Block(BlockStmt {
        location,
        statements,
    })
}

#[inline(always)]
fn parse_expr_stmt<'a>(pair: Pair<'a, Rule>) -> Stmt<'a> {
    let location = location!(pair);
    let next = pair.into_inner().next().expect("parse_expr");
    Stmt::Expr(ExprStmt {
        location,
        expression: parse_expression(next),
    })
}

#[inline(always)]
fn parse_var_stmt<'a>(pair: Pair<'a, Rule>) -> Stmt<'a> {
    let location = location!(pair);
    let variables = pair
        .clone()
        .into_inner()
        .map(|m| parse_decl(m))
        .collect::<Vec<_>>();
    Stmt::VarList(VarListStmt {
        location,
        variables,
    })
}

// #[inline(always)]
// fn parse_decl_list<'a>(pair: Pair<'a, Rule>) -> Stmt<'a> {
//     let location = location!(pair);
//     let variables = pair
//         .clone()
//         .into_inner()
//         .map(|m| parse_decl(m))
//         .collect::<Vec<_>>();
//     Stmt::VarList(VarListStmt {
//         location,
//         variables,
//     })
// }

fn parse_decl<'a>(pair: Pair<'a, Rule>) -> VarStmt<'a> {
    let location = location!(pair);
    let mut inner = pair.into_inner();
    let name = inner.next().unwrap();
    let name = Token::new(
        location!(name),
        TokenType::Identifier,
        name.as_span().as_str(),
    );
    let initializer = inner.next().map(|pair| parse_expression(pair));
    VarStmt {
        location,
        name,
        initializer,
    }
}

#[inline(always)]
fn parse_statement<'a>(pair: Pair<'a, Rule>) -> Stmt<'a> {
    match pair.as_rule() {
        Rule::block => parse_block_stmt(pair),
        Rule::expr_stmt => parse_expr_stmt(pair),
        Rule::var_stmt => parse_var_stmt(pair),
        _ => unreachable!("could not: {:?}", pair.as_rule()),
    }
}

#[inline(always)]
fn parse_program<'a>(pair: Pair<'a, Rule>) -> Stmt<'a> {
    ensure_rule!(pair, Rule::program);

    let location = location!(pair);
    // // A pair is a combination of the rule which matched and a span of input
    // println!("Rule:    {:?}", pair.as_rule());
    // println!("Span:    {:?}", span);
    // println!("Text:    {}", span.as_str());

    // A pair can be converted to an iterator of the tokens which make it up:
    let inner = pair.into_inner();
    let mut statements = Vec::new();
    for inner_pair in inner {
        let stmt = match inner_pair.as_rule() {
            Rule::class_decl => parse_class_decl(inner_pair),
            Rule::func_decl => parse_func_decl(inner_pair),
            Rule::var_stmt => parse_var_stmt(inner_pair),
            Rule::EOI => continue,
            _ => parse_statement(inner_pair),
        };
        statements.push(Box::new(stmt));
    }

    Stmt::Program(ProgramStmt {
        location,
        statements,
    })
}

pub fn parse<'a>(mut pairs: Pairs<'a, Rule>) -> Stmt<'a> {
    parse_program(pairs.next().unwrap())
}
