use crate::token::Token;

#[derive(Debug)]
pub enum Expr {
    Number(i64),
    BinaryOp(Box<Expr>, Token, Box<Expr>),
    IfExpr(Box<Expr>, Vec<Expr>, Vec<Expr>),
}
