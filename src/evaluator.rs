use crate::ast::Expr;
use crate::token::Token;

pub fn evaluate(expr: &Expr) -> i64 {
    match expr {
        Expr::Number(n) => *n,
        Expr::BinaryOp(lhs, op, rhs) => {
            let left = evaluate(lhs);
            let right = evaluate(rhs);
            match op {
                Token::Plus => left + right,
                Token::Minus => left - right,
                _ => panic!("Bilinmeyen işlem"),
            }
        }
        Expr::IfExpr(condition, if_block, else_block) => {
            let cond_value = evaluate(condition);
            let block = if cond_value != 0 { if_block } else { else_block };
            block.iter().map(evaluate).sum()
        }
    }
}
