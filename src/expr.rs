use super::token::{Token, Literal};
use super::env::Type;
use std::fmt::Debug;

#[derive(Clone, Debug)]
pub enum Expr {
    Unary(Token, Box<Expr>),
    Binary(Box<Expr>, Token, Box<Expr>),
    Grouping(Box<Expr>),
    IdentExpr(Token),
    Literal(Literal),
}