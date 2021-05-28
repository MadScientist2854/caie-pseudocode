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

    fn match_tok(&mut self, types: Vec<TokenType>) -> bool {
        for ttype in types {
            if self.check(ttype) { self.advance(); return true }
        }
        false
    }
    fn check(&self, ttype: TokenType) -> bool {
        if self.is_at_end() {}
        self.peak().is_type(ttype)
    }
    fn is_at_end(&self) -> bool {
        self.peak().is_type(TokenType::End)
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

    // fn expr(&self) -> Expr {
    //     self.equality()
    // }
    // fn equality(&self) -> Expr {
    //     let mut expr = self.logic();
    //     while self.match_tok(vec![TokenType::Equal, TokenType::NotEqual]) {
    //         expr = Expr::Binary(Box::new(expr), self.previous(), Box::new(self.logic()))
    //     }
    //     expr
    // }
    // fn logic(&self) -> Expr {
    //     let mut expr = self.comparison();

    // }

}