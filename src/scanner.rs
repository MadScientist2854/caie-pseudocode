use super::token::{Token, TokenType, Literal};
use std::str::FromStr;

pub struct Scanner {
    source: String,

    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source,
            start: 0,
            current: 0,
            line: 1
        }
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, ()> {
        let mut tokens: Vec<Token> = Vec::new();
        while !self.is_at_end() {
            tokens.push(self.scan_token()?);
        }

        Ok(tokens)
    }

    fn scan_token(&mut self) -> Result<Token, ()> {
        let c = self.advance();

        let t = match c {
            ' '|'\r'|'\t' => { self.start = self.current; self.scan_token()? }, // Ignore whitespace
            '/' => if self.peak() == '/' {
                self.advance();
                while self.peak() != '\n' && self.peak() != '\0' {
                    self.advance();
                }
                self.start = self.current;
                self.scan_token()?
            } else { return Err(()) },
            '\n' => {
                self.line += 1;
                self.start = self.current;
                self.scan_token()?
            },

            '-' => if self.peak().is_digit(10) {
                self.advance();
                self.scan_digit()
            } else if self.peak() == '=' {
                self.advance();
                self.new_token(TokenType::MinusEqual)
            } else {
                self.new_token(TokenType::Minus)
            },
            '<' => if self.peak() == '-' {
                self.advance();
                self.new_token(TokenType::Arrow)
            } else { return Err(()) },
            _ => if c.is_digit(10) {
                self.scan_digit()
            } else if c.is_alphabetic() {
                self.scan_ident()
            } else { return Err(()) }
        };

        self.start = self.current;

        Ok(t)
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn new_token(&self, ttype: TokenType) -> Token {
        let text = self.source.get(self.start..self.current).unwrap().to_string();

        Token::new(ttype, text, self.line)
    }

    fn advance(&mut self) -> char {
        self.current += 1;

        self.source.chars().nth(self.current - 1).unwrap()
    }

    fn peak(&self) -> char {
        if self.is_at_end() { '\0' }
        else { self.source.chars().nth(self.current).unwrap() }
    }

    fn peak_next(&self) -> char {
        if self.is_at_end() { '\0' }
        else { self.source.chars().nth(self.current + 1).unwrap() }
    }

    fn scan_digit(&mut self) -> Token {
        while self.peak().is_digit(10) {
            self.advance();
        }
        if self.peak() == '.' && self.peak_next().is_digit(10) {
            self.advance();
            self.advance();
            while self.peak().is_digit(10) {
                self.advance();
            }
            self.new_token(TokenType::Literal(Literal::Float(
                f32::from_str(self.source.get(self.start..self.current).unwrap()).unwrap()
            )))
        } else {
            self.new_token(TokenType::Literal(Literal::Int(
                i32::from_str(self.source.get(self.start..self.current).unwrap()).unwrap()
            )))
        }
    }

    fn scan_ident(&mut self) -> Token {
        while self.peak().is_alphanumeric() {
            self.advance();
        }
        self.new_token(TokenType::Identifier)
    }
}