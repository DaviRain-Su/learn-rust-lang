use std::collections::HashMap;
use crate::token::Token;


#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenType {
    ILLEGAL,
    EOF,
    SPACE,

    //  标识符 + 字面量
    IDENT, // add，foobar, x, y, z,...
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

impl TokenType {
    pub fn is_match(&self, token_type: &TokenType) -> bool {
        self == token_type
    }


}

impl  From<&str> for TokenType {
    fn from(token_type: &str) -> Self {
        match token_type {
            "let" => Self::LET,
            " " => Self::SPACE,
            "=" => Self::ASSIGN,
            _ => Self::IDENT,
        }
    }
}

lazy_static! {
    static ref  KEYWORDS: HashMap<&'static str, TokenType> = {
        let mut m = HashMap::new();
        m.insert("fn", TokenType::FUNCTION);
        m.insert("let", TokenType::LET);
        m
    };
}


pub fn lookup_ident(ident: &str) -> TokenType {
   match KEYWORDS.get(ident) {
       Some(value) => value.clone(),
       None => TokenType::IDENT,
   }
}