use std::fmt::{Debug, Display, Formatter};
use crate::ast::statement::{Expression, Node};
use crate::ast::statement::expression_statement::ExpressionStatement;
use crate::token::Token;

#[derive(Debug)]
pub struct  IntegerLiteral {
    pub token: Token,
    pub value: i64,
}

impl  IntegerLiteral {
    pub fn new(token: Token) -> Self {
        Self {
            token,
            value: i64::default(),
        }
    }
}

impl Display for IntegerLiteral {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
       write!(f, "{}", self.token.literal.clone())
    }
}

impl Node for IntegerLiteral {
    fn token_literal(&self) -> String {
       self.token.literal.clone()
    }
}
impl Expression for IntegerLiteral {
    fn expression_node(&self) {

    }
}

impl From<ExpressionStatement> for IntegerLiteral {
    fn from(expression_statement: ExpressionStatement) -> Self {
        let value = expression_statement.expression.value.parse::<i64>().unwrap();
        Self {
            token: expression_statement.token.clone(),
            value
        }
    }
}
