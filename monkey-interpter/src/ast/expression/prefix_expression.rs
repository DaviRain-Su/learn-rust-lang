use std::any::Any;
use crate::ast::statement::expression_statement::ExpressionStatement;
use crate::ast::statement::{Expression, Node};
use crate::token::token_type::TokenType;
use crate::token::Token;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct PrefixExpression {
    pub token: Token,
    pub operator: String,
    pub right: Box<dyn Expression>,
}

impl Default for PrefixExpression {
    fn default() -> Self {
        Self {
            token: Token::default(),
            operator: String::default(),
            right: Box::new(ExpressionStatement::default()),
        }
    }
}

impl Expression for PrefixExpression {
    fn expression_node(&self) {}
}

impl Display for PrefixExpression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}{})", self.operator, self.right)
    }
}

impl Node for PrefixExpression {
    fn token_literal(&self) -> String {
        println!("[prefix expression] token_literal --> type_id = {:?}", self.type_id());
        self.right.token_literal()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl TryFrom<ExpressionStatement> for PrefixExpression {
    type Error = anyhow::Error;

    fn try_from(value: ExpressionStatement) -> Result<Self, Self::Error> {
        Ok(Self {
            token: Token::from_string(TokenType::IDENT, value.token_literal()),
            operator: value.token_literal(),
            right: Box::new(value),
        })
    }
}
