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

macro_rules! token_from_rule {
    ($rule: expr) => {
        match $rule {
            Rule::THIS_TOKEN => TokenType::This,
            Rule::IDENTIFIER => TokenType::Identifier,
            Rule::IDENTIFIER_NAME => TokenType::Identifier,
            Rule::RELATIONAL_OPERATOR => TokenType::RelationalOperator,
            Rule::EQUALITY_OPERATOR => TokenType::EqualityOperator,
            Rule::ADDITIVE_OPERATOR => TokenType::OpAdditive,
            Rule::MULTIPLICATIVE_OPERATOR => TokenType::OpMultiplicative,
            Rule::SHIFT_OPERATOR => TokenType::ShiftOperator,
            Rule::BITWISE_OR_OPERATOR => TokenType::BitwiseOrOperator,
            Rule::BITWISE_AND_OPERATOR => TokenType::BitwiseAndOperator,
            Rule::LOGICAL_AND_OPERATOR => TokenType::LogicalAndOperator,
            Rule::LOGICAL_OR_OPERATOR => TokenType::LogicalOrOperator,
            _ => unreachable!("should not reach {:?}", $rule),
        }
    };
}

fn parse_literal<'a>(pair: Pair<'a, Rule>) -> Expr<'a> {
    let location = location!(pair);
    let inner = pair.into_inner().next().unwrap();

    let lit = match inner.as_rule() {
        Rule::string_literal => Literal::String(&inner.as_str()[1..inner.as_str().len() - 1]),
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
}

// Expressions
#[inline(always)]
fn parse_expression<'a>(pair: Pair<'a, Rule>) -> Expr<'a> {
    match pair.as_rule() {
        Rule::assignment_expr => parse_assign_expr(pair),
        Rule::logical_or_expr
        | Rule::logical_and_expr
        | Rule::bitwise_or_expr
        | Rule::bitwise_xor_expr
        | Rule::bitwise_and_expr => parse_logical_expr(pair),

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
    let location = location!(pair);
    let mut children = pair
        .into_inner()
        .map(|m| parse_expression(m))
        .collect::<Vec<_>>();

    if children.len() == 1 {
        return children.pop().take().expect("assign child");
    }

    Expr::Assign(AssignExpr {
        location,
        destination: Box::new(children.pop().unwrap()),
        value: Box::new(children.pop().unwrap()),
    })
}

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
            Rule::ADDITIVE_OPERATOR
            | Rule::MULTIPLICATIVE_OPERATOR
            | Rule::SHIFT_OPERATOR
            | Rule::EQUALITY_OPERATOR
            | Rule::RELATIONAL_OPERATOR => {
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
fn parse_logical_expr<'a>(pair: Pair<'a, Rule>) -> Expr<'a> {
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
            Rule::BITWISE_OR_OPERATOR
            | Rule::BITWISE_AND_OPERATOR
            | Rule::LOGICAL_AND_OPERATOR
            | Rule::LOGICAL_OR_OPERATOR => {
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

    Expr::Logical(LogicalExpr {
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

#[inline(always)]
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
fn parse_for_stmt<'a>(pair: Pair<'a, Rule>) -> Stmt<'a> {
    let location = location!(pair);
    let mut children = pair.into_inner().collect::<Vec<_>>();
    let body = children
        .pop()
        .map(|m| Box::new(parse_statement(m)))
        .unwrap();

    let iterator = children.pop().map(|m| parse_expression(m)).unwrap();

    let index = if children.len() == 2 {
        children
            .pop()
            .map(|m| Token::new(location!(m), token_from_rule!(m.as_rule()), m.as_str()))
    } else {
        None
    };
    let element = children
        .pop()
        .map(|m| Token::new(location!(m), token_from_rule!(m.as_rule()), m.as_str()))
        .expect("for element");
    Stmt::For(ForStmt {
        location,
        element,
        index,
        iterator,
        body,
    })
}

#[inline(always)]
fn parse_if_stmt<'a>(pair: Pair<'a, Rule>) -> Stmt<'a> {
    let location = location!(pair);
    let mut children = pair.into_inner().collect::<Vec<_>>();

    let alternative = if children.len() == 3 {
        children
            .pop()
            .map(|m| Box::new(parse_statement(m.into_inner().next().unwrap())))
    } else {
        None
    };

    let consequent = children
        .pop()
        .map(|m| Box::new(parse_statement(m)))
        .unwrap();
    let test = children.pop().map(|m| parse_expression(m)).unwrap();

    Stmt::If(IfStmt {
        location,
        test,
        consequent,
        alternative,
    })
}

fn parse_return_stmt<'a>(pair: Pair<'a, Rule>) -> Stmt<'a> {
    let location = location!(pair);
    let expression = pair.into_inner().next().map(|m| parse_expression(m));
    Stmt::Return(ReturnStmt {
        location,
        expression,
    })
}

#[inline(always)]
fn parse_statement<'a>(pair: Pair<'a, Rule>) -> Stmt<'a> {
    match pair.as_rule() {
        Rule::block => parse_block_stmt(pair),
        Rule::expr_stmt => parse_expr_stmt(pair),
        Rule::var_stmt => parse_var_stmt(pair),
        Rule::for_stmt => parse_for_stmt(pair),
        Rule::if_stmt => parse_if_stmt(pair),
        Rule::return_stmt => parse_return_stmt(pair),
        Rule::continue_stmt => Stmt::Continue(ContinueStmt {
            location: location!(pair),
        }),
        _ => unreachable!("could not: {:?}", pair.as_rule()),
    }
}

#[inline(always)]
fn parse_program<'a>(pair: Pair<'a, Rule>) -> Stmt<'a> {
    ensure_rule!(pair, Rule::program);

    let location = location!(pair);

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
