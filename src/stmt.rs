use super::expr::Expr;
use std::fmt::Debug;

#[derive(Clone, Debug)]
pub enum Stmt {
    ExprStmt(Box<Expr>),
    Output(Vec<Expr>),
}