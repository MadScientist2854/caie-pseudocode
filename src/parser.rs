use super::token::{Token, TokenType};
use super::expr::Expr;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0
        }
    }

    // fn match_tok(&mut self, types: Vec<TokenType>) -> bool {
    //     for ttype in types {
    //         if self.check(ttype) { self.advance(); return true }
    //     }
    //     false
    // }
    // fn check(&self, ttype: TokenType) -> bool {
    //     if self.is_at_end() {}
    //     self.peak().is_type(ttype)
    // }
    fn is_at_end(&self) -> bool {
        match self.peak().ttype {
            TokenType::End => true,
            _ => false
        }
    }
    fn peak(&self) -> Token {
        self.tokens[self.current + 1].clone()
    }
    fn advance(&mut self) -> Token {
        if !self.is_at_end() { self.current += 1 }
        self.previous()
    }
    fn previous(&self) -> Token {
        self.tokens.get(self.current - 1).unwrap().clone()
    }

    pub fn expr(&mut self) -> Expr {
        self.equality()
    }
    fn equality(&mut self) -> Expr {
        let mut expr = self.logic();
        let tkn = self.peak();
        loop { match tkn.ttype {
            TokenType::Equal | TokenType::NotEqual => {
                self.advance();
                expr = Expr::Binary(Box::new(expr), tkn.clone(), Box::new(self.logic()))
            },
            _ => {break;}
        }}
        expr
    }
    fn logic(&mut self) -> Expr {
        let mut expr = self.comparison();
        let tkn = self.peak();
        loop { match tkn.ttype {
            TokenType::AND | TokenType::OR => {
                self.advance();
                expr = Expr::Binary(Box::new(expr), tkn.clone(), Box::new(self.comparison()))
            },
            _ => {break;}
        }}
        expr
    }
    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();
        let tkn = self.peak();
        loop { match tkn.ttype {
            TokenType::Greater | TokenType::Less | TokenType::GreaterEqual | TokenType::LessEqual => {
                self.advance();
                expr = Expr::Binary(Box::new(expr), tkn.clone(), Box::new(self.term()))
            },
            _ => {break;}
        }}
        expr
    }
    fn term(&mut self) -> Expr {
        let mut expr = self.factor();
        let tkn = self.peak();
        loop { match tkn.ttype {
            TokenType::Plus | TokenType::Minus => {
                self.advance();
                expr = Expr::Binary(Box::new(expr), tkn.clone(), Box::new(self.factor()))
            },
            _ => {break;}
        }}
        expr
    }
    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();
        let tkn = self.peak();
        loop { match tkn.ttype {
            TokenType::Slash | TokenType::Star | TokenType::MOD | TokenType::DIV => {
                self.advance();
                expr = Expr::Binary(Box::new(expr), tkn.clone(), Box::new(self.unary()))
            },
            _ => {break;}
        }}
        expr
    }
    fn unary(&mut self) -> Expr {
        let tkn = self.peak();
        match tkn.ttype {
            TokenType::Minus | TokenType::NOT => {self.advance(); Expr::Unary(tkn, Box::new(self.primary()))},
            _ => self.primary()
        }
    }
    fn primary(&mut self) -> Expr {
        let tkn = self.peak();
        println!("{}", tkn.to_string());
        if let TokenType::Int(_) = tkn.ttype {self.advance(); println!("dds"); Expr::Literal(tkn)}
        else {panic!("parse error")}
    }
}