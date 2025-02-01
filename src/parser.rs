use crate::token::Token;
use crate::ast::Expr;

pub fn parse(tokens: &[Token]) -> Expr {
    let mut iter = tokens.iter().peekable();

    if let Some(Token::If) = iter.peek() {
        iter.next();

        let condition = if let Some(Token::Number(n)) = iter.next() {
            Expr::Number(*n)
        } else {
            panic!("must be number!");
        };

        if let Some(Token::LBrace) = iter.next() {
            let mut if_block = Vec::new();
            while let Some(Token::Number(n)) = iter.peek() {
                if_block.push(Expr::Number(*n));
                iter.next();
            }

            if let Some(Token::RBrace) = iter.next() {
                let mut else_block = Vec::new();
                if let Some(Token::Else) = iter.peek() {
                    iter.next();

                    if let Some(Token::LBrace) = iter.next() {
                        while let Some(Token::Number(n)) = iter.peek() {
                            else_block.push(Expr::Number(*n));
                            iter.next();
                        }
                        iter.next();
                    }
                }

                return Expr::IfExpr(Box::new(condition), if_block, else_block);
            }
        }
    }

    panic!("unknow char!");
}
