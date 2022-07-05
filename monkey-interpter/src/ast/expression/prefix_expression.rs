use crate::ast::expression::integer_literal::IntegerLiteral;
use crate::ast::expression::Expression;
use crate::ast::statement::expression_statement::ExpressionStatement;
use crate::ast::Node;
use crate::token::token_type::TokenType;
use crate::token::Token;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub struct PrefixExpression {
    pub token: Token,
    pub operator: String,
    pub right: Box<Expression>,
}

impl Default for PrefixExpression {
    fn default() -> Self {
        Self {
            token: Token::default(),
            operator: String::default(),
            right: Box::new(IntegerLiteral::default().into()),
        }
    }
}

impl Display for PrefixExpression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}{})", self.operator, self.right)
    }
}

impl Node for PrefixExpression {
    fn token_literal(&self) -> String {
        self.right.token_literal()
    }
}

impl TryFrom<ExpressionStatement> for PrefixExpression {
    type Error = anyhow::Error;

    fn try_from(value: ExpressionStatement) -> Result<Self, Self::Error> {
        Ok(Self {
            token: Token::from_string(TokenType::IDENT, value.token_literal()),
            operator: value.token_literal(),
            right: Box::new(value.expression.clone()),
        })
    }
}
