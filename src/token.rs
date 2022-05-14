#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    ILLEGAL,
    EOF,

    IDENT(String),
    INT(u32),

    ASSIGN,
    PLUS,
    MINUS,
    EQUALS,
    NOTEQUALS,
    BANG,
    ASTERISK,
    SLASH,
    LT,
    GT,

    COMA,
    SEMICOLON,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

    FUNCTION,
    LET,
    TRUE,
    FALSE,
}
