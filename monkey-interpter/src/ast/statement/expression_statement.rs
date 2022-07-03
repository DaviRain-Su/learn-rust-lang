use crate::ast::statement::{Node, Statement};
use crate::ast::Identifier;
use crate::token::token_type::TokenType;
use crate::token::Token;
use std::fmt::{Display, Formatter};

/// expression statement
#[derive(Debug, Default)]
pub struct ExpressionStatement {
    pub token: Token, // 该表达式中的第一个词法单元
    pub expression: Identifier,
}

impl Display for ExpressionStatement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{};", self.expression)
    }
}

impl Node for ExpressionStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

impl Statement for ExpressionStatement {
    fn statement_node(&self) {}

    fn identifier(&self) -> Identifier {
        Identifier::from(self.token.clone())
    }
}

impl From<&Box<dyn Statement>> for ExpressionStatement {
    fn from(value: &Box<dyn Statement>) -> Self {
        Self {
            token: Token::from_string(TokenType::IDENT, value.token_literal()),
            expression: value.identifier().clone(),
        }
    }
}
