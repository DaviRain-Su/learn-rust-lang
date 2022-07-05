use std::any::Any;
use crate::ast::statement::{Node, Statement};
use crate::ast::Identifier;
use crate::token::token_type::TokenType;
use crate::token::Token;
use std::fmt::{Display, Formatter};

/// return statement
#[derive(Debug, Default)]
pub struct ReturnStatement {
    pub token: Token, //  return 词法单元
    pub return_value: Identifier,
}

impl Display for ReturnStatement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {};", self.token_literal(), self.return_value)
    }
}

impl Node for ReturnStatement {
    fn token_literal(&self) -> String {
        println!("[return statement] token_literal --> type_id = {:?}", self.type_id());
        self.token.literal.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Statement for ReturnStatement {
    fn statement_node(&self) {}

    fn identifier(&self) -> Identifier {
        Identifier::from(self.token.clone())
    }
}

impl From<Box<dyn Statement>> for ReturnStatement {
    fn from(value: Box<dyn Statement>) -> Self {
        Self {
            token: Token::from_string(TokenType::LET, value.token_literal()),
            return_value: value.identifier().clone(),
        }
    }
}
