use std::fmt::{Display, Formatter};
use super::env::Type;

#[derive(Clone, Debug)]
pub struct Token {
    pub ttype: TokenType,
    pub lexeme: String,
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
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {}", self.ttype, self.lexeme)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Single-char
    Equal,
    LeftBracket, RightBracket,
    LeftParen, RightParen,
    Colon,
    Comma,
    Period,
    Star, Slash, Plus, Minus,
    Less, Greater,

    // Multi-char
    LessEqual, GreaterEqual, NotEqual,
    Arrow,

    // Keywords
    //// Primitives
    BOOLEAN,
    INTEGER, REAL,
    CHAR, STRING,
    DATE,
    ARRAY,
    //// Single-line statements
    DECLARE,
    CONSTANT,
    CALL,
    INPUT, OUTPUT,
    RETURN,
    OPENFILE, CLOSEFILE,
    READFILE, WRITEFILE,
    GETRECORD,
    PUTRECORD,
    SEEK,
    //// Blocks
    PROCEDURE,
    ENDPROCEDURE,
    BYREF, BYVALUE,
    FUNCTION,
    RETURNS,
    ENDFUNCTION,
    FOR, TO, STEP,
    ENDFOR,
    IF,
    THEN,
    ELSE,
    ENDIF,
    CASE, OF,
    OTHERWISE,
    ENDCASE,
    REPEAT,
    UNTIL,
    WHILE,
    DO,
    ENDWHILE,
    TYPE,
    ENDTYPE,
    //// Expression keywords
    MOD,
    DIV,
    AND,
    OR,
    NOT,

    Literal(Literal),
    Identifier,

    NL,
    End

    // Pre-defined functions
    // RND
    // RANDOMBETWEEN
    // EOF
}

#[derive(Clone, Debug, PartialEq)]
pub enum Literal {
    TRUE,
    FALSE,
    READ,
    WRITE,
    APPEND,
    RANDOM,

    Int(i32),
    Float(f32),
    Char(char), //''
    String(String), //""
    Date(i8, i8, i16), // dd/mm/yyyy

    Type(Type)
}

impl Literal {
    pub fn to_string(&self) -> String {
        match self {
            Literal::TRUE => "TRUE".to_string(),
            Literal::FALSE => "FALSE".to_string(),
            Literal::READ => "READ".to_string(),
            Literal::WRITE => "WRITE".to_string(),
            Literal::APPEND => "APPEND".to_string(),
            Literal::RANDOM => "RANDOM".to_string(),
            Literal::Int(val) => format!("{}", val),
            Literal::Float(val) => format!("{}", val),
            Literal::Char(val) => format!("'{}'", val),
            Literal::String(val) => format!("\"{}\"", val),
            Literal::Date(d, m, y) => format!("{}/{}/{}", d, m, y),
            Literal::Type(inner) => format!("{:?}", inner),
        }
    }
}

// impl Display for Literal {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}", self.to_string())
//     }
// }