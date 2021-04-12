pub struct Token {
    ttype: TokenType,
    lexeme: String,
    line: usize
}

impl Token {
    pub fn new(ttype: TokenType, lexeme: String, line: usize) -> Self {
        Self {
            ttype,
            lexeme,
            line
        }
    }

    pub fn to_string(&self) -> String {
        format!("{:?} token on line {}: {}", self.ttype, self.line, self.lexeme)
    }
}

#[derive(Debug, Clone)]
pub enum TokenType {
    Equal,
    LeftBracket,
    RightBracket,
    LeftParen,
    RightParen,
    Colon,
    Comma,
    Dot,
    Star,
    Slash,
    Plus,
    Minus,
    Less,
    Greater,
    MinusEqual,
    Arrow,
    Literal(Literal),
    Identifier
}

#[derive(Debug, Clone)]
pub enum Literal {
    Int(i32),
    Float(f32),
    Char(char),
    String(String),
    Date(i8, i8, i16)
}