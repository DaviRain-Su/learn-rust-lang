use crate::lexer::Lexer;
use crate::token::token_type::TokenType;
use crate::token::Token;

fn test_next_token() {
    // let input = "=+(){},;";
    let input = "let five = 5;
let ten = 10;
let add = fn(x, y) {
    x + y;
};
let result = add(five, ten);";

    let tests = vec![
        Token {
            r#type: TokenType::LET,
            literal: String::from("let"),
        },
        Token {
            r#type: TokenType::IDENT,
            literal: String::from("five"),
        },
        Token {
            r#type: TokenType::ASSIGN,
            literal: String::from("="),
        },
        Token {
            r#type: TokenType::INT,
            literal: String::from("5"),
        },
        Token {
            r#type: TokenType::SEMICOLON,
            literal: String::from(";"),
        },
        Token {
            r#type: TokenType::LET,
            literal: String::from("let"),
        },
        Token {
            r#type: TokenType::IDENT,
            literal: String::from("ten"),
        },
        Token {
            r#type: TokenType::SEMICOLON,
            literal: String::from(";"),
        },
        Token {
            r#type: TokenType::LET,
            literal: String::from("let"),
        },
        Token {
            r#type: TokenType::IDENT,
            literal: String::from("add"),
        },
        Token {
            r#type: TokenType::ASSIGN,
            literal: String::from("="),
        },
        Token {
            r#type: TokenType::INT,
            literal: String::from("10"),
        },
        Token {
            r#type: TokenType::SEMICOLON,
            literal: String::from(";"),
        },
        Token {
            r#type: TokenType::LET,
            literal: String::from("let"),
        },
        Token {
            r#type: TokenType::IDENT,
            literal: String::from("add"),
        },
        Token {
            r#type: TokenType::ASSIGN,
            literal: String::from("="),
        },
        Token {
            r#type: TokenType::FUNCTION,
            literal: String::from("fn"),
        },
        Token {
            r#type: TokenType::LPAREN,
            literal: String::from("("),
        },
        Token {
            r#type: TokenType::IDENT,
            literal: String::from("x"),
        },
        Token {
            r#type: TokenType::COMMA,
            literal: String::from(","),
        },
        Token {
            r#type: TokenType::IDENT,
            literal: String::from("y"),
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
            r#type: TokenType::IDENT,
            literal: String::from("x"),
        },
        Token {
            r#type: TokenType::PLUS,
            literal: String::from("+"),
        },
        Token {
            r#type: TokenType::IDENT,
            literal: String::from("y"),
        },
        Token {
            r#type: TokenType::SEMICOLON,
            literal: String::from(";"),
        },
        Token {
            r#type: TokenType::RBRACE,
            literal: String::from("}"),
        },
        Token {
            r#type: TokenType::SEMICOLON,
            literal: String::from(";"),
        },
        Token {
            r#type: TokenType::LET,
            literal: String::from("let"),
        },
        Token {
            r#type: TokenType::IDENT,
            literal: String::from("result"),
        },
        Token {
            r#type: TokenType::ASSIGN,
            literal: String::from("="),
        },
        Token {
            r#type: TokenType::IDENT,
            literal: String::from("add"),
        },
        Token {
            r#type: TokenType::LPAREN,
            literal: String::from("("),
        },
        Token {
            r#type: TokenType::IDENT,
            literal: String::from("five"),
        },
        Token {
            r#type: TokenType::COMMA,
            literal: String::from(","),
        },
        Token {
            r#type: TokenType::IDENT,
            literal: String::from("ten"),
        },
        Token {
            r#type: TokenType::RPAREN,
            literal: String::from(")"),
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
        let mut tok = l.next_token();
        while tok.r#type.is_match(&TokenType::SPACE) {
            tok = l.next_token();
        }
        println!("token = {:?}", tok);

        if tok.r#type != tt.r#type {
            println!(
                "tests[{}] - token type wrong. expected = {:?}, \
                   got = {:?}
                ",
                i, tt.r#type, tok.r#type
            );
        }

        if tok.literal != tt.literal {
            println!(
                "tests[{}] - literal wrong. expected = {:?}, \
                got = {:?}
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
