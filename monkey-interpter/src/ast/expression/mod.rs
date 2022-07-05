use std::fmt::{Display, Formatter, write};
use std::process::id;
use crate::ast::expression::integer_literal::IntegerLiteral;
use crate::ast::expression::prefix_expression::PrefixExpression;
use crate::ast::{Identifier, Node};

pub mod integer_literal;
pub mod prefix_expression;

#[derive(Debug, Clone)]
pub enum Expression {
    PrefixExpression(PrefixExpression),
    IntegerLiteralExpression(IntegerLiteral),
    IdentifierExpression(Identifier)
}

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::PrefixExpression(pre_exp) => write!(f, "{}", pre_exp),
            Expression::IntegerLiteralExpression(integ_exp) => write!(f, "{}", integ_exp),
            Expression::IdentifierExpression(ident) => write!(f,"{}", ident),
        }
    }
}


impl Node for Expression {
    fn token_literal(&self) -> String {
        match self {
            Self::PrefixExpression(pre_exp) => pre_exp.token_literal(),
            Self::IntegerLiteralExpression(integ_exp) => integ_exp.token_literal(),
            Self::IdentifierExpression(ident) => ident.token_literal(),
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

impl  From<Identifier> for Expression {
    fn from(identifier: Identifier) -> Self {
        Self::IdentifierExpression(identifier)
    }
}