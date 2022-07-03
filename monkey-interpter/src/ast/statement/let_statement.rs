use crate::ast::statement::{Node, Statement};
use crate::ast::Identifier;
use crate::token::token_type::TokenType;
use crate::token::Token;
use std::fmt::{Display, Formatter};

/// let statement
#[derive(Debug, Default)]
pub struct LetStatement {
    pub token: Token, // token.LET 词法单元
    pub name: Identifier,
    pub value: Identifier,
}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

impl Statement for LetStatement {
    fn statement_node(&self) {}

    fn identifier(&self) -> Identifier {
        self.name.clone()
    }
}

impl Display for LetStatement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} = {};",
            self.token_literal(),
            self.name,
            self.value
        )
    }
}

impl From<&Box<dyn Statement>> for LetStatement {
    fn from(value: &Box<dyn Statement>) -> Self {
        Self {
            token: Token::from_string(TokenType::LET, "let".into()),
            name: value.identifier().clone(),
            value: value.identifier().clone(),
        }
    }
}
