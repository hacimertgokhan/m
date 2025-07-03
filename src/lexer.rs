use crate::token::Token;

pub fn lex(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let chars: Vec<char> = input.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        match chars[i] {
            '0'..='9' => {
                let mut num = chars[i].to_digit(10).unwrap() as i64;
                i += 1;
                while i < chars.len() && chars[i].is_numeric() {
                    num = num * 10 + chars[i].to_digit(10).unwrap() as i64;
                    i += 1;
                }
                tokens.push(Token::Number(num));
                continue;
            }
            '+' => tokens.push(Token::Plus),
            '-' => tokens.push(Token::Minus),
            '>' => tokens.push(Token::Greater),
            '<' => tokens.push(Token::Less),
            '=' => {
                if i + 1 < chars.len() && chars[i + 1] == '=' {
                    tokens.push(Token::Equal);
                    i += 1;
                } else {
                    tokens.push(Token::Equal);
                }
            }
            '{' => tokens.push(Token::LBrace),
            '}' => tokens.push(Token::RBrace),
            '(' => tokens.push(Token::LParen),
            ')' => tokens.push(Token::RParen),
            ',' => tokens.push(Token::Comma),
            ' ' | '\n' => {}
            '"' => {
                i += 1;
                let start = i;
                while i < chars.len() && chars[i] != '"' {
                    i += 1;
                }
                if i >= chars.len() {
                    panic!("String bitmedi!");
                }
                let s: String = chars[start..i].iter().collect();
                tokens.push(Token::String(s));
                i += 1;
                continue;
            }
            '\'' => {
                if i + 2 < chars.len() && chars[i + 2] == '\'' {
                    let c = chars[i + 1];
                    tokens.push(Token::Char(c));
                    i += 3;
                    continue;
                } else {
                    panic!("Char literal hatalı!");
                }
            }
            _ => {
                if input[i..].starts_with("if") && (i + 2 == chars.len() || !chars[i + 2].is_alphanumeric()) {
                    tokens.push(Token::If);
                    i += 1;
                } else if input[i..].starts_with("else") && (i + 4 == chars.len() || !chars[i + 4].is_alphanumeric()) {
                    tokens.push(Token::Else);
                    i += 3;
                } else if input[i..].starts_with("fn") && (i + 2 == chars.len() || !chars[i + 2].is_alphanumeric()) {
                    tokens.push(Token::Fn);
                    i += 1;
                } else if input[i..].starts_with("let") && (i + 3 == chars.len() || !chars[i + 3].is_alphanumeric()) {
                    tokens.push(Token::Let);
                    i += 2;
                } else if input[i..].starts_with("print") && (i + 5 == chars.len() || !chars[i + 5].is_alphanumeric()) {
                    tokens.push(Token::Print);
                    i += 4;
                } else if chars[i].is_alphabetic() {
                    let start = i;
                    while i < chars.len() && (chars[i].is_alphanumeric() || chars[i] == '_') {
                        i += 1;
                    }
                    let ident: String = chars[start..i].iter().collect();
                    tokens.push(Token::Ident(ident));
                    i -= 1;
                } else {
                    panic!("unknow char: {}", chars[i]);
                }
            }
        }
        i += 1;
    }

    tokens
}
