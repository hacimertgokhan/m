#[cfg(test)]
mod tests {
    use crate::lexer::lex;
    use crate::token::Token;

    #[test]
    fn test_lex() {
        let input = "if 1 { 10 } else { 20 }";
        let tokens = lex(input);
        assert_eq!(
            tokens,
            vec![
                Token::If,
                Token::Number(1),
                Token::LBrace,
                Token::Number(10),
                Token::RBrace,
                Token::Else,
                Token::LBrace,
                Token::Number(20),
                Token::RBrace,
            ]
        );
    }
}
