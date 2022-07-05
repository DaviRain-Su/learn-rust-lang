use std::fmt::{Display, Formatter};
use crate::ast::expression::Expression;
use crate::ast::Node;
use crate::ast::statement::expression_statement::ExpressionStatement;
use crate::token::Token;

#[derive(Debug, Clone)]
pub struct InfixExpression {
    pub token: Token,
    pub left: Box<Expression>,
    pub operator: String,
    pub right: Box<Expression>,
}


impl Display for InfixExpression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"( {} {} {} )", self.left, self.operator, self.right)
    }
}

impl Node for InfixExpression {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

impl TryFrom<ExpressionStatement> for InfixExpression {
    type Error = anyhow::Error;

    fn try_from(value: ExpressionStatement) -> Result<Self, Self::Error> {
        match value.expression.clone() {
            Expression::InfixExpression(infix_exp) => Ok(infix_exp.clone()),
            _ => unimplemented!(),
        }
    }
}