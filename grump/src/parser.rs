use crate::token::Token;

#[derive(Debug, Clone)]
pub enum Expr {
    Number(i64),
    Symbol(String),
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
            Token::Symbol(s) => Expr::Symbol(s),
            Token::OpenParen => {
                let mut list = Vec::new();
                while let Some(tok) = self.peek() {
                    if tok == &Token::CloseParen {
                        self.next_token();
                        break;
                    }
                    list.push(self.parse());
                }
                Expr::List(list)
            }
            _ => panic!("Unexpected token"),
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
