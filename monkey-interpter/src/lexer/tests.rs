use crate::lexer::Lexer;
use crate::token::token_type::TokenType;
use crate::token::Token;

fn test_next_token() {
    let input = "=+(){},;";

    let tests = vec![
        Token {
            r#type: TokenType::ASSIGN,
            literal: String::from("="),
        },
        Token {
            r#type: TokenType::PLUS,
            literal: String::from("+"),
        },
        Token {
            r#type: TokenType::LPAREN,
            literal: String::from("("),
        },
        Token {
            r#type: TokenType::RPAREN,
            literal: String::from(")"),
        },
        Token {
            r#type: TokenType::LBRACE,
            literal: String::from("{"),
        },
        Token {
            r#type: TokenType::RBRACE,
            literal: String::from("}"),
        },
        Token {
            r#type: TokenType::COMMA,
            literal: String::from(","),
        },
        Token {
            r#type: TokenType::SEMICOLON,
            literal: String::from(";"),
        },
        Token {
            r#type: TokenType::EOF,
            literal: String::from(0 as char),
        },
    ];


    let mut l = Lexer::new(input);
    for (i, tt) in tests.iter().enumerate() {
        let tok = l.next_token();
        println!("token = {:?}", tok);

        if tok.r#type != tt.r#type {
            println!(
                "tests[{0}] - token type wrong. expected = {1}, \
                   got = {2}
                ",
                i, tt.r#type, tok.r#type
            );
        }

        if tok.literal != tt.literal {
            println!(
                "tests[{0}] - literal wrong. expected = {1}, \
                got = {2}
                ",
                i, tt.literal, tok.literal
            );
        }
    }
}

#[test]
fn test_test_next_token() {
    test_next_token();
}
