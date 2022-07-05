use crate::ast::{Identifier, Node};
use crate::token::token_type::TokenType;
use crate::token::Token;
use std::fmt::{Display, Formatter};
use crate::ast::statement::Statement;

/// let statement
#[derive(Debug, Default, Clone)]
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

impl From<Statement> for LetStatement {
    fn from(value: Statement) -> Self {
       match value {
           Statement::LetStatement(let_s) => let_s.clone(),
           _ => unimplemented!()
       }
    }
}

impl From<&Statement> for LetStatement {
    fn from(value: &Statement) -> Self {
        match value {
            Statement::LetStatement(let_s) => let_s.clone(),
            _ => unimplemented!()
        }
    }
}
