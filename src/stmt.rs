use super::expr::Expr;
use super::token::Token;
use super::env::Type;
use std::fmt::Debug;

#[derive(Clone, Debug)]
pub enum Stmt {
    Block(Vec<Stmt>),
    ExprStmt(Expr),
    Declare(Token, Expr),
    Constant(Token, Expr),
    Assign(Expr, Expr),
    ProcCall(Expr, Vec<Expr>),
    Input(Expr),
    Output(Vec<Expr>),
    Procedure(Token, Vec<(Token, Expr)>, Box<Stmt>),
    ForTo(Token, Expr, Expr, Option<Expr>, Box<Stmt>),
    IfThen(Expr, Box<Stmt>, Option<Box<Stmt>>),
    Case(Expr, Vec<(Expr, Stmt)>, Option<Box<Stmt>>),
    Repeat(Expr, Box<Stmt>),
    WhileDo(Expr, Box<Stmt>),
}