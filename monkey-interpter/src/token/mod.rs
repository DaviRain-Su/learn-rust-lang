// type TokenType = String;

pub mod token_type;

use crate::token::token_type::TokenType;

#[derive(Debug)]
pub struct Token {
    // identifier for token type
    pub(crate) r#type: TokenType,
    // identifier for token value
    pub(crate) literal: String,
}

impl Token {
    pub fn new(token_type: TokenType, ch: char) -> Self {
        Self {
            r#type: token_type,
            literal: String::from(ch),
        }
    }
}

impl Default for Token {
    fn default() -> Self {
        Token::new(TokenType::EOF, 0 as char)
    }
}

#[test]
#[ignore]
fn test_token_struct() {
    let temp_struct = Token {
        r#type: TokenType::LET,
        literal: String::from("literal"),
    };

    println!("token = {:?}", temp_struct);
}
