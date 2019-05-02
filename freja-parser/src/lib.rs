#[cfg(feature = "serde_support")]
#[macro_use]
extern crate serde;

pub mod ast;

#[inline]
pub(crate) fn resolve_binary(l: ast::Expr, mut o:Vec<(ast::BinaryOperator, ast::Expr)>) -> ast::Expr {
    if o.is_empty() {
        return l;
    }
    let first = o.pop().unwrap();
    let left = ast::Expr::Binary(ast::BinaryExpr::new(Box::new(l), Box::new(first.1), first.0));
    if o.is_empty() {
        return left;
    }
    resolve_binary(left, o)
}

#[inline]
pub(crate) fn resolve_logical(l: ast::Expr, mut o:Vec<(ast::LogicalOperator, ast::Expr)>) -> ast::Expr {
    if o.is_empty() {
        return l;
    }
    let first = o.pop().unwrap();
    let left = ast::Expr::Logical(ast::LogicalExpr::new(Box::new(l), Box::new(first.1), first.0));
    if o.is_empty() {
        return left;
    }
    resolve_logical(left, o)
}

// pub mod grammar2;
// pub use grammar2 as parser;

pub mod parser {
    include!(concat!(env!("OUT_DIR"), "/grammar.rs"));
}

#[cfg(test)]
mod tests {

    use super::ast::*;
    use super::parser;

    mod literals {
        use super::*;

        #[test]
        fn number_test() {
            assert_eq!(
                parser::program("2000").unwrap()[0],
                Stmt::Expr(ExprStmt::new(Expr::Literal(LiteralExpr::new(
                    Literal::Number(Number::Integer(2000))
                ))))
            );
            assert_eq!(
                parser::program("0").unwrap()[0],
                Stmt::Expr(ExprStmt::new(Expr::Literal(LiteralExpr::new(
                    Literal::Number(Number::Integer(0))
                ))))
            );
            assert_eq!(
                parser::program("2.02").unwrap()[0],
                Stmt::Expr(ExprStmt::new(Expr::Literal(LiteralExpr::new(
                    Literal::Number(Number::Double(2.02))
                ))))
            );
            assert_eq!(
                parser::program("103030302.20202").unwrap()[0],
                Stmt::Expr(ExprStmt::new(Expr::Literal(LiteralExpr::new(
                    Literal::Number(Number::Double(103030302.20202))
                ))))
            );
            assert_eq!(
                parser::program("0.1").unwrap()[0],
                Stmt::Expr(ExprStmt::new(Expr::Literal(LiteralExpr::new(
                    Literal::Number(Number::Double(0.1))
                ))))
            );
        }

        #[test]
        fn boolean_test() {
            assert_eq!(
                parser::program("true").unwrap()[0],
                Stmt::Expr(ExprStmt::new(Expr::Literal(LiteralExpr::new(
                    Literal::Boolean(true)
                ))))
            );
            assert_eq!(
                parser::program("false").unwrap()[0],
                Stmt::Expr(ExprStmt::new(Expr::Literal(LiteralExpr::new(
                    Literal::Boolean(false)
                ))))
            );
        }

        #[test]
        fn string_test() {
            assert_eq!(
                parser::program("\"Hello, World!\"").unwrap()[0],
                Stmt::Expr(ExprStmt::new(Expr::Literal(LiteralExpr::new(
                    Literal::String("Hello, World!".to_string())
                ))))
            );
            assert_eq!(
                parser::program("\"Test \nmig i øret 😀\"").unwrap()[0],
                Stmt::Expr(ExprStmt::new(Expr::Literal(LiteralExpr::new(
                    Literal::String("Test \nmig i øret 😀".to_string())
                ))))
            );
        }
    }
}
