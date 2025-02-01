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
}
