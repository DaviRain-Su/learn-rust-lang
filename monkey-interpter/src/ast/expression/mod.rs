use crate::ast::expression::boolean::Boolean;
use crate::ast::expression::infix_expression::InfixExpression;
use crate::ast::expression::integer_literal::IntegerLiteral;
use crate::ast::expression::prefix_expression::PrefixExpression;
use crate::ast::{Identifier, Node};
use std::fmt::{Display, Formatter};

pub mod boolean;
pub mod infix_expression;
pub mod integer_literal;
pub mod prefix_expression;

#[derive(Debug, Clone)]
pub enum Expression {
    PrefixExpression(PrefixExpression),
    InfixExpression(InfixExpression),
    IntegerLiteralExpression(IntegerLiteral),
    IdentifierExpression(Identifier),
    BooleanExpression(Boolean),
}

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::PrefixExpression(pre_exp) => write!(f, "{}", pre_exp),
            Expression::InfixExpression(infix_exp) => write!(f, "{}", infix_exp),
            Expression::IntegerLiteralExpression(integ_exp) => write!(f, "{}", integ_exp),
            Expression::IdentifierExpression(ident) => write!(f, "{}", ident),
            Expression::BooleanExpression(boolean) => write!(f, "{}", boolean),
        }
    }
}

impl Node for Expression {
    fn token_literal(&self) -> String {
        match self {
            Self::PrefixExpression(pre_exp) => pre_exp.token_literal(),
            Self::InfixExpression(infix_exp) => infix_exp.token_literal(),
            Self::IntegerLiteralExpression(integ_exp) => integ_exp.token_literal(),
            Self::IdentifierExpression(ident) => ident.token_literal(),
            Self::BooleanExpression(boolean) => boolean.token_literal(),
        }
    }
}

impl From<PrefixExpression> for Expression {
    fn from(pre_exp: PrefixExpression) -> Self {
        Self::PrefixExpression(pre_exp)
    }
}

impl From<IntegerLiteral> for Expression {
    fn from(integ_exp: IntegerLiteral) -> Self {
        Self::IntegerLiteralExpression(integ_exp)
    }
}

impl From<Identifier> for Expression {
    fn from(identifier: Identifier) -> Self {
        Self::IdentifierExpression(identifier)
    }
}

impl From<InfixExpression> for Expression {
    fn from(infix_exp: InfixExpression) -> Self {
        Self::InfixExpression(infix_exp)
    }
}

impl From<Boolean> for Expression {
    fn from(boolean: Boolean) -> Self {
        Self::BooleanExpression(boolean)
    }
}
