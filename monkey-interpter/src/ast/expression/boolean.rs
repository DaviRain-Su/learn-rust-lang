use std::fmt::{Display, Formatter};
use crate::ast::expression::Expression;
use crate::ast::Node;
use crate::token::Token;

#[derive(Debug, Clone)]
pub struct Boolean {
    pub token: Token,
    pub value: bool,
}

impl Display for Boolean {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.token.literal)
    }
}

impl Node for Boolean {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

impl TryFrom<Expression> for Boolean {
    type Error = anyhow::Error;

    fn try_from(value: Expression) -> Result<Self, Self::Error> {
        match value {
            Expression::BooleanExpression(boolean) => Ok(boolean),
            _ => unimplemented!()
        }
    }
}