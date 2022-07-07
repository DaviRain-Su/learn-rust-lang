use std::fmt::{Debug, Display, Formatter};
use crate::ast::Node;
use crate::ast::statement::Statement;
use crate::token::Token;

#[derive(Debug, Clone)]
pub struct BlockStatement {
    pub token: Token, // '{' 词法单元
    pub statements: Vec<Statement>,
}


impl Display for BlockStatement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ ")?;
        for statement in self.statements.iter() {
            write!(f, "{}", statement)?;
        }
        write!(f, " }}")?;
        Ok(())
    }
}

impl Node for BlockStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}