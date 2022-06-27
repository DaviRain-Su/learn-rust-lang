use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq, Eq)]
pub enum TokenType {
    ILLEGAL,
    EOF,

    //  标识符 + 字面量
    IDENT(String), // add，foobar, x, y, z,...
    INT,           // 12345

    // 运算符
    ASSIGN,
    PLUS,

    // 分隔符
    COMMA,
    SEMICOLON,

    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

    // 关键字
    FUNCTION,
    LET,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ILLEGAL => write!(f, "ILLEGAL"),
            Self::EOF => write!(f, "EOF"),
            Self::IDENT(value) => write!(f, "IDENT({})", value),
            Self::INT => write!(f, "INT"),
            Self::ASSIGN => write!(f, "="),
            Self::PLUS => write!(f, "+"),
            Self::COMMA => write!(f, ","),
            Self::SEMICOLON => write!(f, ";"),
            Self::LPAREN => write!(f, "("),
            Self::RPAREN => write!(f, ")"),
            Self::LBRACE => write!(f, "{{"),
            Self::RBRACE => write!(f, "}}"),
            Self::FUNCTION => write!(f, "FUNCTION"),
            Self::LET => write!(f, "LET"),
        }
    }
}
