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
    StarEqual, SlashEqual, PlusEqual, MinusEqual,
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
    STEP,
    //// Blocks
    PROCEDURE,
    ENDPROCEDURE,
    BYREF, BYVALUE,
    FUNCTION,
    RETURNS,
    ENDFUNCTION,
    FOR,
    ENDFOR,
    IF,
    THEN,
    ELSE,
    ENDIF,
    CASE,
    ENDCASE,
    OTHERWISE,
    REPEAT,
    UNTIL,
    WHILE,
    ENDWHILE,
    TYPE,
    ENDTYPE,
    //// Expression keywords
    MOD,
    DIV,
    AND,
    OR,
    NOT,
    TRUE,
    FALSE,
    READ,
    WRITE,
    APPEND,
    RANDOM,
    EOF,
    //// Misc
    OF,
    TO,

    // Literals
    Literal(Literal),
    Identifier

    // Pre-defined functions
    // RND
    // RANDOMBETWEEN
}

#[derive(Debug, Clone)]
pub enum Literal {
    Int(i32),
    Float(f32),
    Char(char), //''
    String(String), //""
    Date(i8, i8, i16) // dd/mm/yyyy
}