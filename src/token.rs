#[derive(Clone)]
pub struct Token {
    ttype: TokenType,
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

    pub fn to_string(&self) -> String {
        format!("{:?} token on line {}: {}", self.ttype, self.line, self.lexeme)
    }

    pub fn is_type(&self, ttype: TokenType) -> bool {
        self.ttype == ttype
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

    // Literals
    Int(i32),
    Float(f32),
    Char(char), //''
    String(String), //""
    Date(i8, i8, i16), // dd/mm/yyyy
    Identifier,

    NL,
    End

    // Pre-defined functions
    // RND
    // RANDOMBETWEEN
    // EOF
}