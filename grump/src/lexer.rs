use crate::token::Token;

pub struct Lexer {
    input: Vec<char>,
    pos: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Lexer {
            input: input.chars().collect(),
            pos: 0,
        }
    }

    fn next_char(&mut self) -> Option<char> {
        if self.pos >= self.input.len() {
            None
        } else {
            let ch = self.input[self.pos];
            self.pos += 1;
            Some(ch)
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while let Some(ch) = self.next_char() {
            match ch {
                '(' => tokens.push(Token::OpenParen),
                ')' => tokens.push(Token::CloseParen),
                '+' => tokens.push(Token::Plus),
                '-' => tokens.push(Token::Minus),
                '*' => tokens.push(Token::Multiply),
                '/' => tokens.push(Token::Divide),
                '0'..='9' => {
                    let mut num = ch.to_string();
                    while let Some(next_ch) = self.peek() {
                        if next_ch.is_digit(10) {
                            num.push(self.next_char().unwrap());
                        } else {
                            break;
                        }
                    }
                    tokens.push(Token::Number(num.parse::<i64>().unwrap()));
                }
                _ if ch.is_alphabetic() => {
                    let mut ident = ch.to_string();
                    while let Some(next_ch) = self.peek() {
                        if next_ch.is_alphanumeric() {
                            ident.push(self.next_char().unwrap());
                        } else {
                            break;
                        }
                    }
                    match ident.as_str() {
                        "define" => tokens.push(Token::Define),
                        "lambda" => tokens.push(Token::Lambda),
                        _ => tokens.push(Token::Symbol(ident)),
                    }
                }
                _ if ch.is_whitespace() => continue,
                _ => panic!("Unexpected character: {}", ch),
            }
        }
        tokens.push(Token::EOF);
        tokens
    }

    fn peek(&self) -> Option<char> {
        if self.pos >= self.input.len() {
            None
        } else {
            Some(self.input[self.pos])
        }
    }
}
