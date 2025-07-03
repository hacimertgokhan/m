#[derive(Debug, Clone)]
pub enum Token {
    Number(i64),
    Plus,
    Minus,
    If,
    Else,
    LBrace,  // {
    RBrace,  // }
    Greater, // >
    Less,    // <
    Equal,   // ==
    Fn,      // fn
    Ident(String), // identifier
    LParen,  // (
    RParen,  // )
    Comma,   // ,
    Let,        // let
    String(String), // "..."
    Char(char),    // 'c'
    Print,      // print
}
