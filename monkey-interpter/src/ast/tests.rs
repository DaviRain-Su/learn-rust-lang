use crate::ast::statement::let_statement::LetStatement;
use crate::ast::{Identifier, Program};
use crate::token::token_type::TokenType;
use crate::token::Token;

#[test]
#[ignore]
fn test_display() {
    let let_statement = LetStatement {
        token: Token::from_string(TokenType::LET, "let".into()),
        name: Identifier {
            token: Token::from_string(TokenType::IDENT, "myVar".into()),
            value: "myVar".into(),
        },
        value: Identifier {
            token: Token::from_string(TokenType::IDENT, "anotherVar".into()),
            value: "anotherVar".into(),
        },
    };

    println!("let statement debug = {:#?}", let_statement);
    println!("let statement display = {}", let_statement);

    let program = Program {
        statements: vec![let_statement.into()],
    };

    println!("program debug = {:#?}", program);
    println!("program display = {}", program);

    assert_eq!(format!("{}", program), "let myVar = anotherVar;");
}
