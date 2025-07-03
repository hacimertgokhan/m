use crate::ast::{Expr, Ast};
use crate::token::Token;
use std::collections::HashMap;

// Değer türlerini desteklemek için enum
#[derive(Debug, Clone)]
pub enum Value {
    Number(i64),
    String(String),
    Char(char),
}

impl Value {
    fn to_number(&self) -> i64 {
        match self {
            Value::Number(n) => *n,
            Value::String(_) => 0,
            Value::Char(_) => 0,
        }
    }

    fn to_string(&self) -> String {
        match self {
            Value::Number(n) => n.to_string(),
            Value::String(s) => s.clone(),
            Value::Char(c) => c.to_string(),
        }
    }
}

pub fn evaluate(ast: &Ast) -> i64 {
    let mut functions = HashMap::new();
    let mut env = HashMap::new();
    match ast {
        Ast::Program(exprs) => {
            let mut last = 0;
            for expr in exprs {
                last = evaluate_with_env(expr, &mut functions, &mut env).to_number();
            }
            last
        }
    }
}

fn evaluate_with_env(expr: &Expr, functions: &mut HashMap<String, (Vec<String>, Vec<Expr>)>, env: &mut HashMap<String, Value>) -> Value {
    match expr {
        Expr::Number(n) => Value::Number(*n),
        Expr::StringLit(s) => {
            // String literal'ı sadece döndür, yazdırma!
            Value::String(s.clone())
        }
        Expr::CharLit(c) => {
            // Char literal'ı sadece döndür, yazdırma!
            Value::Char(*c)
        }
        Expr::Identifier(name) => {
            env.get(name).cloned().unwrap_or(Value::Number(0))
        }
        Expr::Let(name, expr) => {
            let value = evaluate_with_env(expr, functions, env);
            env.insert(name.clone(), value.clone());
            value
        }
        Expr::BinaryOp(lhs, op, rhs) => {
            let left = evaluate_with_env(lhs, functions, env).to_number();
            let right = evaluate_with_env(rhs, functions, env).to_number();
            match op {
                Token::Plus => Value::Number(left + right),
                Token::Minus => Value::Number(left - right),
                _ => panic!("Bilinmeyen işlem"),
            }
        }
        Expr::IfExpr(condition, if_block, else_block) => {
            let cond_value = evaluate_with_env(condition, functions, env).to_number();
            let block = if cond_value != 0 { if_block } else { else_block };
            let mut result = Value::Number(0);
            for e in block {
                result = evaluate_with_env(e, functions, env);
            }
            result
        }
        Expr::FunctionDef(name, params, body) => {
            functions.insert(name.clone(), (params.clone(), body.clone()));
            Value::Number(0) // Tanım sonucu 0 döndür
        }
        Expr::FunctionCall(name, args) => {
            let (params, body) = if let Some((params, body)) = functions.get(name) {
                (params.clone(), body.clone())
            } else {
                panic!("Fonksiyon bulunamadı: {}", name);
            };
            if params.len() != args.len() {
                panic!("Parametre sayısı uyuşmuyor");
            }
            let mut local_env = env.clone();
            for (param, arg) in params.iter().zip(args.iter()) {
                let value = evaluate_with_env(arg, functions, env);
                local_env.insert(param.clone(), value);
            }
            let mut result = Value::Number(0);
            for e in body {
                result = evaluate_with_env(&e, functions, &mut local_env);
            }
            result
        }
        Expr::Print(expr) => {
            let value = evaluate_with_env(expr, functions, env);
            // Print sadece burada yazdırır
            println!("{}", value.to_string());
            Value::Number(0)
        }
    }
}