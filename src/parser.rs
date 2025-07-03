use crate::token::Token;
use crate::ast::{Expr, Ast};

pub fn parse(tokens: &[Token]) -> Ast {
    let mut iter = tokens.iter().peekable();
    let mut exprs = Vec::new();
    while iter.peek().is_some() {
        if let Some(expr) = parse_stmt(&mut iter) {
            exprs.push(expr);
        } else {
            break;
        }
    }
    Ast::Program(exprs)
}

fn parse_stmt<'a, I>(iter: &mut std::iter::Peekable<I>) -> Option<Expr>
where
    I: Iterator<Item = &'a Token>,
{
    if let Some(Token::Fn) = iter.peek() {
        iter.next();
        // fn <ident> ( <params> ) { <body> }
        let name = if let Some(Token::Ident(name)) = iter.next() {
            name.clone()
        } else {
            panic!("function name expected");
        };
        if let Some(Token::LParen) = iter.next() {
            let mut params = Vec::new();
            if let Some(Token::RParen) = iter.peek() {
                iter.next();
            } else {
                loop {
                    match iter.peek() {
                        Some(Token::Ident(param)) => {
                            params.push(param.clone());
                            iter.next();
                            if let Some(Token::Comma) = iter.peek() {
                                iter.next();
                            } else {
                                break;
                            }
                        }
                        Some(Token::RParen) => {
                            iter.next();
                            break;
                        }
                        _ => panic!("unexpected token in parameter list"),
                    }
                }
            }
            // Remove the redundant RParen check here - it's already consumed above
            if let Some(Token::LBrace) = iter.next() {
                let mut body = Vec::new();
                while let Some(token) = iter.peek() {
                    match token {
                        Token::RBrace => { iter.next(); break; },
                        Token::Print => {
                            iter.next();
                            let expr = if let Some(Token::LParen) = iter.peek() {
                                iter.next();
                                let inner = parse_expr(iter).expect("print ifadesinde değer bekleniyor");
                                if let Some(Token::RParen) = iter.next() {
                                    inner
                                } else {
                                    panic!("print ifadesinde kapanış parantezi bekleniyor");
                                }
                            } else {
                                parse_expr(iter).expect("print ifadesinde değer bekleniyor")
                            };
                            body.push(Expr::Print(Box::new(expr)));
                        }
                        _ => {
                            if let Some(expr) = parse_expr(iter) {
                                body.push(expr);
                            } else {
                                break;
                            }
                        }
                    }
                }
                return Some(Expr::FunctionDef(name, params, body));
            }
        }
        panic!("invalid function definition");
    }

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
                return Some(Expr::IfExpr(Box::new(condition), if_block, else_block));
            }
        }
    }

    // let ataması
    if let Some(Token::Let) = iter.peek() {
        iter.next();
        let name = if let Some(Token::Ident(name)) = iter.next() {
            name.clone()
        } else {
            panic!("let sonrası isim bekleniyor");
        };
        if let Some(Token::Equal) = iter.next() {
            if let Some(expr) = parse_expr(iter) {
                return Some(Expr::Let(name, Box::new(expr)));
            } else {
                panic!("let atamasında değer bekleniyor");
            }
        } else {
            panic!("let atamasında = bekleniyor");
        }
    }

    // Fonksiyon çağrısı veya sayı
    if let Some(expr) = parse_expr(iter) {
        return Some(expr);
    }

    None
}

fn parse_expr<'a, I>(iter: &mut std::iter::Peekable<I>) -> Option<Expr>
where
    I: Iterator<Item = &'a Token>,
{
    let mut left = parse_primary(iter)?;

    while let Some(op) = iter.peek() {
        match op {
            Token::Plus | Token::Minus => {
                let op = op.clone();
                iter.next();
                if let Some(right) = parse_primary(iter) {
                    left = Expr::BinaryOp(Box::new(left), op.clone(), Box::new(right));
                } else {
                    break;
                }
            }
            _ => break,
        }
    }

    Some(left)
}

fn parse_primary<'a, I>(iter: &mut std::iter::Peekable<I>) -> Option<Expr>
where
    I: Iterator<Item = &'a Token>,
{
    match iter.peek() {
        Some(Token::Number(n)) => {
            let n = *n;
            iter.next();
            Some(Expr::Number(n))
        }
        Some(Token::String(s)) => {
            let s = s.clone();
            iter.next();
            Some(Expr::StringLit(s))
        }
        Some(Token::Char(c)) => {
            let c = *c;
            iter.next();
            Some(Expr::CharLit(c))
        }
        Some(Token::Ident(name)) => {
            let name = name.clone();
            iter.next();
            // Fonksiyon çağrısı mı?
            if let Some(Token::LParen) = iter.peek() {
                iter.next();
                let mut args = Vec::new();
                loop {
                    match iter.peek() {
                        Some(Token::Number(_)) | Some(Token::Ident(_)) | Some(Token::String(_)) | Some(Token::Char(_)) => {
                            if let Some(arg) = parse_expr(iter) {
                                args.push(arg);
                            }
                            if let Some(Token::Comma) = iter.peek() {
                                iter.next();
                            } else {
                                break;
                            }
                        }
                        Some(Token::RParen) => break,
                        _ => break,
                    }
                }
                if let Some(Token::RParen) = iter.next() {
                    Some(Expr::FunctionCall(name, args))
                } else {
                    panic!("expected ) in function call");
                }
            } else {
                Some(Expr::Identifier(name))
            }
        }
        _ => None,
    }
}