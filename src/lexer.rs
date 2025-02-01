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
                }
            }
            '{' => tokens.push(Token::LBrace),
            '}' => tokens.push(Token::RBrace),
            ' ' | '\n' => {}
            _ => {
                if input[i..].starts_with("if") {
                    tokens.push(Token::If);
                    i += 1;
                } else if input[i..].starts_with("else") {
                    tokens.push(Token::Else);
                    i += 3;
                } else {
                    panic!("unknow char: {}", chars[i]);
                }
            }
        }
        i += 1;
    }

    tokens
}
