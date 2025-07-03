use crate::token::Token;

#[derive(Debug, Clone)]
pub enum Expr {
    Number(i64),
    BinaryOp(Box<Expr>, Token, Box<Expr>),
    IfExpr(Box<Expr>, Vec<Expr>, Vec<Expr>),
    FunctionDef(String, Vec<String>, Vec<Expr>),
    FunctionCall(String, Vec<Expr>),
    Let(String, Box<Expr>),
    Identifier(String),
    StringLit(String),
    CharLit(char),
    Print(Box<Expr>),
}

pub enum Ast {
    Program(Vec<Expr>),
}
