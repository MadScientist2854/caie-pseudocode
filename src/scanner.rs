use super::token::{Token, TokenType};
use super::expr::Expr;
use std::{collections::HashMap, str::FromStr};

pub struct Scanner {
    source: String,

    start: usize,
    current: usize,
    line: usize,

    keywords: HashMap<String, TokenType>
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source,
            start: 0,
            current: 0,
            line: 1,
            keywords: {
                let mut map = HashMap::new();
                map.insert("INTEGER".to_string(), TokenType::INTEGER);
                map.insert("BOOLEAN".to_string(), TokenType::BOOLEAN);
                map.insert("REAL".to_string(), TokenType::REAL);
                map.insert("CHAR".to_string(), TokenType::CHAR);
                map.insert("STRING".to_string(), TokenType::STRING);
                map.insert("DATE".to_string(), TokenType::DATE);
                map.insert("ARRAY".to_string(), TokenType::ARRAY);
                map.insert("DECLARE".to_string(), TokenType::DECLARE);
                map.insert("CONSTANT".to_string(), TokenType::CONSTANT);
                map.insert("CALL".to_string(), TokenType::CALL);
                map.insert("INPUT".to_string(), TokenType::INPUT);
                map.insert("OUTPUT".to_string(), TokenType::OUTPUT);
                map.insert("RETURN".to_string(), TokenType::RETURN);
                map.insert("OPENFILE".to_string(), TokenType::OPENFILE);
                map.insert("CLOSEFILE".to_string(), TokenType::CLOSEFILE);
                map.insert("READFILE".to_string(), TokenType::READFILE);
                map.insert("WRITEFILE".to_string(), TokenType::WRITEFILE);
                map.insert("GETRECORD".to_string(), TokenType::GETRECORD);
                map.insert("PUTRECORD".to_string(), TokenType::PUTRECORD);
                map.insert("SEEK".to_string(), TokenType::SEEK);
                map.insert("STEP".to_string(), TokenType::STEP);
                map.insert("PROCEDURE".to_string(), TokenType::PROCEDURE);
                map.insert("ENDPROCEDURE".to_string(), TokenType::ENDPROCEDURE);
                map.insert("BYREF".to_string(), TokenType::BYREF);
                map.insert("BYVALUE".to_string(), TokenType::BYVALUE);
                map.insert("FUNCTION".to_string(), TokenType::FUNCTION);
                map.insert("RETURNS".to_string(), TokenType::RETURNS);
                map.insert("ENDFUNCTION".to_string(), TokenType::ENDFUNCTION);
                map.insert("FOR".to_string(), TokenType::FOR);
                map.insert("ENDFOR".to_string(), TokenType::ENDFOR);
                map.insert("IF".to_string(), TokenType::IF);
                map.insert("THEN".to_string(), TokenType::THEN);
                map.insert("ELSE".to_string(), TokenType::ELSE);
                map.insert("ENDIF".to_string(), TokenType::ENDIF);
                map.insert("CASE".to_string(), TokenType::CASE);
                map.insert("ENDCASE".to_string(), TokenType::ENDCASE);
                map.insert("OTHERWISE".to_string(), TokenType::OTHERWISE);
                map.insert("REPEAT".to_string(), TokenType::REPEAT);
                map.insert("UNTIL".to_string(), TokenType::UNTIL);
                map.insert("WHILE".to_string(), TokenType::WHILE);
                map.insert("ENDWHILE".to_string(), TokenType::ENDWHILE);
                map.insert("TYPE".to_string(), TokenType::TYPE);
                map.insert("ENDTYPE".to_string(), TokenType::ENDTYPE);
                map.insert("MOD".to_string(), TokenType::MOD);
                map.insert("DIV".to_string(), TokenType::DIV);
                map.insert("AND".to_string(), TokenType::AND);
                map.insert("OR".to_string(), TokenType::OR);
                map.insert("NOT".to_string(), TokenType::NOT);
                map.insert("TRUE".to_string(), TokenType::TRUE);
                map.insert("FALSE".to_string(), TokenType::FALSE);
                map.insert("READ".to_string(), TokenType::READ);
                map.insert("WRITE".to_string(), TokenType::WRITE);
                map.insert("APPEND".to_string(), TokenType::APPEND);
                map.insert("RANDOM".to_string(), TokenType::RANDOM);
                map.insert("OF".to_string(), TokenType::OF);
                map.insert("TO".to_string(), TokenType::TO);
                map
            }
        }
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, ()> {
        let mut tokens: Vec<Token> = Vec::new();
        while !self.is_at_end() {
            tokens.push(self.scan_token()?);
        }
        tokens.push(Token::new(TokenType::End, "".to_string(), self.line));

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
            } else { self.new_token(TokenType::Slash) },
            '\n' => {
                self.line += 1;
                loop {
                    if self.peak() == '\n' {
                        self.advance();
                        self.line += 1;
                    } else if self.peak() == ' ' || self.peak() == '\r' || self.peak() == '\t' {
                        self.advance();
                    } else { break }
                }
                self.new_token(TokenType::NL)
            },

            '=' => self.new_token(TokenType::Equal),
            '[' => self.new_token(TokenType::LeftBracket),
            ']' => self.new_token(TokenType::RightBracket),
            '(' => self.new_token(TokenType::LeftParen),
            ')' => self.new_token(TokenType::RightParen),
            ':' => self.new_token(TokenType::Colon),
            ',' => self.new_token(TokenType::Comma),
            '.' => self.new_token(TokenType::Period),

            '*' => self.new_token(TokenType::Star),
            '+' => self.new_token(TokenType::Plus),
            '-' => self.new_token(TokenType::Minus),
            '<' => if self.peak() == '-' {
                self.advance();
                self.new_token(TokenType::Arrow)
            } else if self.peak() == '=' {
                self.advance();
                self.new_token(TokenType::LessEqual)
            } else if self.peak() == '>' {
                self.advance();
                self.new_token(TokenType::NotEqual)
            } else { self.new_token(TokenType::Less) },
            '>' => if self.peak() == '=' {
                self.advance();
                self.new_token(TokenType::GreaterEqual)
            } else { self.new_token(TokenType::Greater) },

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
            self.new_token(TokenType::Float(
                f32::from_str(self.source.get(self.start..self.current).unwrap()).unwrap()
            ))
        } else {
            self.new_token(TokenType::Int(
                i32::from_str(self.source.get(self.start..self.current).unwrap()).unwrap()
            ))
        }
    }

    fn scan_ident(&mut self) -> Token {
        while self.peak().is_alphanumeric() {
            self.advance();
        }
        let text = self.source.get(self.start..self.current).unwrap().to_string();
        if let Some(ttype) = self.keywords.get(&text) {
            self.new_token(ttype.clone())
        } else { self.new_token(TokenType::Identifier) }
    }
}