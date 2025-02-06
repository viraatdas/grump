use crate::token::Token;

#[derive(Debug, Clone)]
pub enum Expr {
    Number(i64),
    Symbol(String), // Store operator names as symbols
    List(Vec<Expr>),
}

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }

    fn next_token(&mut self) -> Token {
        if self.pos >= self.tokens.len() {
            Token::EOF
        } else {
            let tok = self.tokens[self.pos].clone();
            self.pos += 1;
            tok
        }
    }

    pub fn parse(&mut self) -> Expr {
        match self.next_token() {
            Token::Number(n) => Expr::Number(n),

            // ✅ Fix: Treat operators as valid symbols
            Token::Plus => Expr::Symbol("+".to_string()),
            Token::Minus => Expr::Symbol("-".to_string()),
            Token::Multiply => Expr::Symbol("*".to_string()),
            Token::Divide => Expr::Symbol("/".to_string()),

            Token::Symbol(s) => Expr::Symbol(s),

            Token::OpenParen => {
                let mut list = Vec::new();
                while let Some(tok) = self.peek() {
                    if *tok == Token::CloseParen {
                        self.next_token(); // Consume ')'
                        break;
                    }
                    list.push(self.parse()); // Recursively parse inner expressions
                }
                Expr::List(list)
            }
            Token::CloseParen => {
                panic!("Unexpected token: CloseParen - Maybe you have an extra ')'?");
            }
            Token::EOF => {
                panic!("Unexpected EOF - Your input might be incomplete");
            }
            _ => {
                panic!("Unexpected token: {:?}", self.tokens[self.pos - 1]);
            }
        }
    }

    fn peek(&self) -> Option<&Token> {
        if self.pos >= self.tokens.len() {
            None
        } else {
            Some(&self.tokens[self.pos])
        }
    }
}
