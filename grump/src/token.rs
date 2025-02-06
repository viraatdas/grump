#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(i64),
    Symbol(String),
    OpenParen,
    CloseParen,
    Define,
    Lambda,
    Plus,
    Minus,
    Multiply,
    Divide,
    EOF,
}
