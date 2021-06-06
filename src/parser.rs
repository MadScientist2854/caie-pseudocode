use core::panic;

use super::token::{Token, TokenType};
use super::expr::Expr;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize
}

struct ParseError {
    pub msg: String,
    pub token: Token
}

impl ParseError {
    pub fn new(token: Token, msg: String) -> Self {
        Self {
            token,
            msg
        }
    }
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0
        }
    }

    pub fn parse(&mut self) -> Expr {
        match self.expr() { // TODO: move this into all statement productions
            Ok(expr) => expr,
            Err(err) => match err.token.ttype {
                TokenType::End => panic!("error at end: {}", err.msg),
                _ => panic!("error at token {}: {}", err.token, err.msg)
            }
        }
    }

    fn is_at_end(&self) -> bool {
        match self.peak().ttype {
            TokenType::End => true,
            _ => false
        }
    }
    fn peak(&self) -> Token {
        self.tokens[self.current].clone()
    }
    fn advance(&mut self) -> Token {
        if !self.is_at_end() { self.current += 1 }
        self.previous()
    }
    fn previous(&self) -> Token {
        self.tokens.get(self.current - 1).unwrap().clone()
    }

    // fn statement(&self) -> Expr {}

    pub fn expr(&mut self) -> Result<Expr, ParseError> {
        self.equality()
    }
    fn equality(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.logic()?;
        let mut tkn = self.peak();
        loop { match tkn.ttype {
                TokenType::Equal | TokenType::NotEqual => {
                    self.advance();
                    expr = Expr::Binary(Box::new(expr), tkn.clone(), Box::new(self.logic()?));
                    // println!("{}", expr.prettify())
                },
                _ => {break;}
            }
            tkn = self.peak();
        }
        Ok(expr)
    }
    fn logic(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.comparison()?;
        let mut tkn = self.peak();
        loop { match tkn.ttype {
                TokenType::AND | TokenType::OR => {
                    self.advance();
                    expr = Expr::Binary(Box::new(expr), tkn.clone(), Box::new(self.comparison()?));
                    // println!("{}", expr.prettify())
                },
                _ => {break;}
            }
            tkn = self.peak();
        }
        Ok(expr)
    }
    fn comparison(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.term()?;
        let mut tkn = self.peak();
        loop { match tkn.ttype {
                TokenType::Greater | TokenType::Less | TokenType::GreaterEqual | TokenType::LessEqual => {
                    self.advance();
                    expr = Expr::Binary(Box::new(expr), tkn.clone(), Box::new(self.term()?));
                    // println!("{}", expr.prettify())
                },
                _ => {break;}
            }
            tkn = self.peak();
        }
        Ok(expr)
    }
    fn term(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.factor()?;
        let mut tkn = self.peak();
        loop { match tkn.ttype {
                TokenType::Plus | TokenType::Minus => {
                    self.advance();
                    expr = Expr::Binary(Box::new(expr), tkn.clone(), Box::new(self.factor()?));
                    // println!("{}", expr.prettify())
                },
                _ => {break;}
            }
            tkn = self.peak();
        }
        Ok(expr)
    }
    fn factor(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.unary()?;
        let mut tkn = self.peak();
        loop { match tkn.ttype {
                TokenType::Slash | TokenType::Star | TokenType::MOD | TokenType::DIV => {
                    self.advance();
                    expr = Expr::Binary(Box::new(expr), tkn.clone(), Box::new(self.unary()?));
                    // println!("{}", expr.prettify())
                }
                _ => {break;}
            }
            tkn = self.peak();
        }
        Ok(expr)
    }
    fn unary(&mut self) -> Result<Expr, ParseError> {
        let tkn = self.peak();
        if let TokenType::Minus | TokenType::NOT = tkn.ttype
        {self.advance(); Ok(Expr::Unary(tkn, Box::new(self.primary()?)))}
        else { self.primary() }
    }
    fn primary(&mut self) -> Result<Expr, ParseError> {
        let tkn = self.peak();
        // println!("{}", tkn);
        match tkn.ttype {
            TokenType::Literal(lit) => {self.advance(); Ok(Expr::Literal(lit))},
            // TokenType::Identifier => {
            //     self.advance();
            //     let mut expr = Expr::Literal(tkn);
            //     let mut tkn = self.peak();
            //     loop { match tkn.ttype {
            //             TokenType::Period => {
            //                 self.advance();
            //                 expr = Expr::Binary(Box::new(expr), tkn.clone(), Box::new(self.primary()));
            //             }
            //             TokenType::LeftBracket => {
            //                 let expr
            //             }
            //             _ => {break}
            //         }
            //         tkn = self.peak();
            //     }
            //     expr
            // }
            TokenType::LeftParen => {
                self.advance();
                let expr = Expr::Grouping(Box::new(self.expr()?));
                if self.peak().ttype != TokenType::RightParen { return Err(ParseError::new(self.peak(), "Unterminated Grouping".to_string())) }
                self.advance();
                Ok(expr)
            },
            TokenType::End => Err(ParseError::new(tkn, "Expected expression".to_string())),
            _ => Err(ParseError::new(tkn, "Invalid expression-starting token".to_string()))
        }
    }
}