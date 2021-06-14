use super::expr::Expr;
use super::token::Token;
use std::fmt::Debug;

#[derive(Clone, Debug)]
pub enum Stmt {
    Block(Vec<Stmt>),
    ExprStmt(Expr),
    Assign(Token, Expr),
    Input(Expr),
    Output(Vec<Expr>),
    IfThen(Expr, Box<Stmt>, Option<Box<Stmt>>),
}