use super::token::{Token, Literal};
use std::fmt::Debug;

#[derive(Clone, Debug)]
pub enum Expr {
    Unary(Token, Box<Expr>),
    Binary(Box<Expr>, Token, Box<Expr>),
    Grouping(Box<Expr>),
    IdentExpr(Token),
    FnCall(Token, Vec<Expr>),
    ArrIdx(Token, Box<Expr>, Option<Box<Expr>>),
    ArrType((Box<Expr>, Box<Expr>), Option<(Box<Expr>, Box<Expr>)>, Box<Expr>),
    Literal(Literal),
}