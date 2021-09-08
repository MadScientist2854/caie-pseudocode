use super::token::{Token, TokenType};
use super::expr::Expr;
use super::stmt::Stmt;

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

    pub fn parse(&mut self) -> Stmt {
        match self.program() {
            Ok(prog) => prog,
            Err(err) => match err.token.ttype {
                TokenType::End => panic!("parse error at end: {}", err.msg),
                _ => panic!("parse error at token {}: {}", err.token, err.msg)
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

    fn program(&mut self) -> Result<Stmt, ParseError> {
        self.block(vec![TokenType::End])
    }
    fn block(&mut self, terminators: Vec<TokenType>) -> Result<Stmt, ParseError> {
        while self.peak().ttype == TokenType::NL { self.advance(); } // get rid of newlines at the start

        let mut statements = Vec::new();
        'outer: loop {
            for terminator in terminators.clone() {
                if self.peak().ttype == terminator { break 'outer }
            }
            statements.push(self.statement()?)
        }
        self.advance();
        Ok(Stmt::Block(statements))
    }
    fn statement(&mut self) -> Result<Stmt, ParseError> {
        let tkn = self.peak();
        let stmt = match tkn.ttype {
            TokenType::DECLARE => self.declare(),
            TokenType::CONSTANT => self.constant(),
            TokenType::Identifier => self.assign(),
            TokenType::CALL => self.proccall(),
            TokenType::INPUT => self.input(),
            TokenType::OUTPUT => self.output(),
            TokenType::RETURN => self.ret(),
            TokenType::PROCEDURE => self.procedure(),
            TokenType::FUNCTION => self.function(),
            TokenType::FOR => self.forto(),
            TokenType::IF => self.ifthen(),
            TokenType::CASE => self.case(),
            TokenType::REPEAT => self.repeat(),
            TokenType::WHILE => self.whiledo(),
            _ => Ok(Stmt::ExprStmt(self.expr()?))
        };
        match self.peak().ttype {
            TokenType::NL | TokenType::End => { self.advance(); stmt },
            _ => Err(ParseError::new(self.peak(), "Expected newline after statement".into()))
        }
    }
    fn declare(&mut self) -> Result<Stmt, ParseError> {
        self.advance();
        let name  = self.peak();
        if name.ttype == TokenType::Identifier
        { self.advance(); }
        else { return Err(ParseError::new(self.peak(), "Expected identifier".into())) }
        if self.peak().ttype == TokenType::Colon { self.advance(); Ok(Stmt::Declare(name, self.expr()?)) }
        else { Err(ParseError::new(self.peak(), "Expected ':' token".into())) }
    }
    fn constant(&mut self) -> Result<Stmt, ParseError> {
        self.advance();
        let name  = self.peak();
        if name.ttype == TokenType::Identifier { self.advance(); }
        else { return Err(ParseError::new(self.peak(), "Expected identifier".into())) }
        if self.peak().ttype == TokenType::Equal { self.advance(); Ok(Stmt::Constant(name, self.expr()?)) }
        else { Err(ParseError::new(self.peak(), "Expected '=' token".into())) }
    }
    fn assign(&mut self) -> Result<Stmt, ParseError> {
        let name = self.expr()?;
        if self.peak().ttype == TokenType::Arrow
        { self.advance(); Ok(Stmt::Assign(name, self.expr()?)) }
        else { Ok(Stmt::ExprStmt(name)) }
    }
    fn proccall(&mut self) -> Result<Stmt, ParseError> {
        self.advance();
        let name = if self.peak().ttype == TokenType::Identifier { self.advance() }
        else { panic!("expected identifier") };

        let mut arg_list = vec![];
        if self.peak().ttype == TokenType::LeftParen {
            self.advance();
            arg_list.push(self.expr()?);
            while self.peak().ttype == TokenType::Comma {
                self.advance();
                if self.peak().ttype == TokenType::RightParen { break }
                arg_list.push(self.expr()?);
            }
            if self.peak().ttype == TokenType::RightParen { self.advance(); }
            else { return Err(ParseError::new(self.peak(), "Expected ')' token".into())) }
        }
        
        Ok(Stmt::ProcCall(name, arg_list))
    }
    fn input(&mut self) -> Result<Stmt, ParseError> {
        self.advance();
        Ok(Stmt::Input(self.expr()?))
    }
    fn output(&mut self) -> Result<Stmt, ParseError> {
        self.advance();
        let mut exprs = Vec::new();
        exprs.push(self.expr()?);
        while self.peak().ttype == TokenType::Comma {
            self.advance();
            exprs.push(self.expr()?);
        }
        Ok(Stmt::Output(exprs))
    }
    fn ret(&mut self) -> Result<Stmt, ParseError> {
        self.advance();
        Ok(Stmt::Ret(self.expr()?))
    }
    fn procedure(&mut self) -> Result<Stmt, ParseError> {
        self.advance();
        let name = self.advance();
        if name.ttype != TokenType::Identifier {
            panic!("expected identifier")
        }

        let mut args = vec![];
        if self.peak().ttype == TokenType::LeftParen {
            self.advance();
            loop {
                let byref;
                if self.peak().ttype == TokenType::Identifier { byref = false }
                else if self.peak().ttype == TokenType::BYREF { self.advance(); byref = true }
                else if self.peak().ttype == TokenType::BYVALUE { self.advance(); byref = false }
                else { break }

                let name = self.advance();
                if self.peak().ttype == TokenType::Colon { self.advance(); }
                else { return Err(ParseError::new(self.peak(), "Expected ':' token".into())) }
                let dtype = self.expr()?;
                args.push((name, dtype, byref));
    
                if self.peak().ttype != TokenType::Comma { break }
            }
            if self.peak().ttype == TokenType::RightParen { self.advance(); }
            else { return Err(ParseError::new(self.peak(), "Expected ')' token".into())) }
        }

        if self.peak().ttype == TokenType::NL { self.advance(); }
        else { return Err(ParseError::new(self.peak(), "Expected newline after function signature".into())) }

        let block = self.block(vec![TokenType::ENDPROCEDURE])?;
        Ok(Stmt::Procedure(name, args, Box::new(block)))
    }
    fn function(&mut self) -> Result<Stmt, ParseError> {
        self.advance();
        let name = self.advance();
        if name.ttype != TokenType::Identifier {
            panic!("expected identifier")
        }

        let mut args = vec![];
        if self.peak().ttype == TokenType::LeftParen {
            self.advance();
            loop {
                let byref;
                if self.peak().ttype == TokenType::Identifier { byref = false }
                else if self.peak().ttype == TokenType::BYREF { self.advance(); byref = true }
                else if self.peak().ttype == TokenType::BYVALUE { self.advance(); byref = false }
                else { break }

                let name = self.advance();
                if self.peak().ttype == TokenType::Colon { self.advance(); }
                else { return Err(ParseError::new(self.peak(), "Expected ':' token".into())) }
                let dtype = self.expr()?;
                args.push((name, dtype, byref));
    
                if self.peak().ttype != TokenType::Comma { break }
            }
            if self.peak().ttype == TokenType::RightParen { self.advance(); }
            else { return Err(ParseError::new(self.peak(), "Expected ')' token".into())) }
        }

        if self.peak().ttype == TokenType::RETURNS { self.advance(); }
        else { return Err(ParseError::new(self.peak(), "Expected return type".into())) }
        let ret_type = self.expr()?;

        if self.peak().ttype == TokenType::NL { self.advance(); }
        else { return Err(ParseError::new(self.peak(), "Expected newline".into())) }

        let block = self.block(vec![TokenType::ENDFUNCTION])?;
        Ok(Stmt::Function(name, args, ret_type, Box::new(block)))
    }
    fn forto(&mut self) -> Result<Stmt, ParseError> {
        self.advance();
        let name  = self.peak();
        if name.ttype == TokenType::Identifier { self.advance(); }
        else { return Err(ParseError::new(self.peak(), "Expected identifier".into())) }
        
        let val1: Expr;
        if self.peak().ttype == TokenType::Arrow
        { self.advance(); val1 = self.expr()? }
        else { return Err(ParseError::new(self.peak(), "Expected '<-' token".into())) }
        if self.peak().ttype == TokenType::TO { self.advance(); }
        else { return Err(ParseError::new(self.peak(), "Expected `TO` token".into())) }
        let val2 = self.expr()?;
        let mut step = None;
        if let TokenType::STEP = self.peak().ttype { self.advance(); step = Some(self.expr()?) }
        if self.peak().ttype == TokenType::NL { self.advance(); }
        else { return Err(ParseError::new(self.peak(), "Expected newline".into())) }
        let block = self.block(vec![TokenType::ENDFOR])?;

        if self.peak().ttype == TokenType::Identifier { self.advance(); }

        Ok(Stmt::ForTo(name, val1, val2, step, Box::new(block)))
    }
    fn ifthen(&mut self) -> Result<Stmt, ParseError> {
        self.advance();
        let condition = self.expr()?;
        if self.peak().ttype == TokenType::NL { self.advance(); }
        else { return Err(ParseError::new(self.peak(), "Expected newline".into())) }
        if self.peak().ttype == TokenType::THEN { self.advance(); }
        else { return Err(ParseError::new(self.peak(), "'THEN' required after 'IF'".into())) }
        if self.peak().ttype == TokenType::NL { self.advance(); }
        else { return Err(ParseError::new(self.peak(), "Expected newline".into())) }
        let then_block = self.block(vec![TokenType::ELSE, TokenType::ENDIF])?;
        if let TokenType::ELSE = self.previous().ttype {
            if self.peak().ttype == TokenType::NL { self.advance(); }
            else { return Err(ParseError::new(self.peak(), "Expected newline".into())) }
            let else_block = self.block(vec![TokenType::ENDIF])?;
            Ok(Stmt::IfThen(condition, Box::new(then_block), Some(Box::new(else_block))))
        }
        else { Ok(Stmt::IfThen(condition, Box::new(then_block), None)) }
    }
    fn case(&mut self) -> Result<Stmt, ParseError> {
        self.advance();
        if self.peak().ttype == TokenType::OF { self.advance(); }
        else { return Err(ParseError::new(self.peak(), "'OF' required after 'CASE'".into())) }
        let val = self.expr()?;
        if self.peak().ttype == TokenType::NL { self.advance(); }
        else { return Err(ParseError::new(self.peak(), "Expected newline".into())) }
        let mut cases: Vec<(Expr, Stmt)> = Vec::new();
        while self.peak().ttype != TokenType::ENDCASE {
            if let TokenType::OTHERWISE = self.peak().ttype {
                self.advance();
                if self.peak().ttype == TokenType::Colon { self.advance(); }
                else { return Err(ParseError::new(self.peak(), "Expected ':' token".into())) }
                let ret = Ok(Stmt::Case(val, cases, Some(Box::new(self.statement()?))));
                if self.peak().ttype == TokenType::ENDCASE { self.advance(); return ret }
                else { return Err(ParseError::new(self.peak(), "Expected 'ENDWHILE' token".into())) }
            }
            let case = self.expr()?;
            if self.peak().ttype == TokenType::Colon { self.advance(); }
            else { return Err(ParseError::new(self.peak(), "Expected ':' token".into())) }
            let stmt = self.statement()?;
            cases.push((case, stmt));
        }
        self.advance();
        Ok(Stmt::Case(val, cases, None))
    }
    fn repeat(&mut self) -> Result<Stmt, ParseError> {
        self.advance();
        if self.peak().ttype == TokenType::NL { self.advance(); }
        else { return Err(ParseError::new(self.peak(), "Expected newline".into())) }
        let block = self.block(vec![TokenType::UNTIL])?;
        let condition = self.expr()?;
        Ok(Stmt::Repeat(condition, Box::new(block)))
    }
    fn whiledo(&mut self) -> Result<Stmt, ParseError> {
        self.advance();
        let condition = self.expr()?;
        if self.peak().ttype == TokenType::DO { self.advance(); }
        else { return Err(ParseError::new(self.peak(), "'DO' required after 'WHILE'".into())) }
        if self.peak().ttype == TokenType::NL { self.advance(); }
        else { return Err(ParseError::new(self.peak(), "Expected newline".into())) }
        let block = self.block(vec![TokenType::ENDWHILE])?;
        Ok(Stmt::WhileDo(condition, Box::new(block)))
    }

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
            TokenType::Identifier => {
                self.advance();
                match self.peak().ttype {
                    TokenType::LeftParen => {
                        self.advance();
                        let mut arg_list = vec![];
                        arg_list.push(self.expr()?);
                        while self.peak().ttype == TokenType::Comma {
                            self.advance();
                            if self.peak().ttype == TokenType::RightParen { break }
                            arg_list.push(self.expr()?);
                        }
                        if self.peak().ttype == TokenType::RightParen { self.advance(); }
                        else { return Err(ParseError::new(self.peak(), "Expected ')' token".into())) }
                        Ok(Expr::FnCall(tkn, arg_list))
                    },
                    TokenType::LeftBracket => {
                        self.advance();
                        let expr1 = self.expr()?;
                        let mut expr2 = None;
                        if self.peak().ttype == TokenType::Comma {
                            self.advance();
                            expr2 = Some(Box::new(self.expr()?));
                        }
                        if self.peak().ttype != TokenType::RightBracket {
                            Err(ParseError::new(self.peak(), "unterminated array index".into()))
                        } else {
                            self.advance();
                            Ok(Expr::ArrIdx(tkn, Box::new(expr1), expr2))
                        }
                    },
                    _ => Ok(Expr::IdentExpr(tkn))
                }
            },
            TokenType::ARRAY => {
                self.advance();
                if self.peak().ttype != TokenType::LeftBracket
                    { return Err(ParseError::new(self.peak(), "expected `[`".into())) }
                self.advance();
                let idx1start = self.expr()?;
                if self.peak().ttype != TokenType::Colon
                    { return Err(ParseError::new(self.peak(), "expected `:`".into())) }
                self.advance();
                let idx1end = self.expr()?;
                let mut idx2start = None;
                let mut idx2end = None;
                if self.peak().ttype == TokenType::Comma {
                    self.advance();
                    idx2start = Some(self.expr()?);
                    if self.peak().ttype != TokenType::Colon
                        { return Err(ParseError::new(self.peak(), "expected `:`".into())) }
                    self.advance();
                    idx2end = Some(self.expr()?);
                }
                if self.peak().ttype != TokenType::RightBracket
                    { return Err(ParseError::new(self.peak(), "expected `]`".into())) }
                self.advance();
                if self.peak().ttype != TokenType::OF
                    { return Err(ParseError::new(self.peak(), "expected OF".into())) }
                self.advance();
                let dtype = self.expr()?;
                let idx2 = idx2start.map(move |idx2start| {
                    (Box::new(idx2start), Box::new(idx2end.unwrap()))
                });
                Ok(Expr::ArrType((Box::new(idx1start), Box::new(idx1end)), idx2, Box::new(dtype)))
            },
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

    fn dtype(&mut self) -> Result<Expr, ParseError> {
        let dtype = match self.peak().ttype {
            // TokenType::Identifier => Ok(Type::UDT), TODO
            // TokenType::ARRAY => Ok(Type::Array(Type, size)),
            _ => Err(ParseError::new(self.peak(), "Expected type".into()))
        };
        self.advance();
        dtype
    }
}

#[derive(Debug, Clone)]
pub struct ParseError {
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