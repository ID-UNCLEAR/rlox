use crate::ast::Expr;
use crate::common::Token;

/// Statement enum
/// Statements DO something, not producing values
/// For example:
///     print "Hello!";
#[derive(Debug, PartialEq)]
pub enum Stmt {
    Block { statements: Vec<Stmt> },
    Expression { expression: Box<Expr> },
    Print { expression: Box<Expr> },
    Var { name: Token, initializer: Box<Expr> },
}
