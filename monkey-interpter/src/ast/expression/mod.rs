use crate::ast::expression::boolean::Boolean;
use crate::ast::expression::if_expression::IfExpression;
use crate::ast::expression::infix_expression::InfixExpression;
use crate::ast::expression::integer_literal::IntegerLiteral;
use crate::ast::expression::prefix_expression::PrefixExpression;
use crate::ast::{Identifier, Node};
use std::fmt::{Display, Formatter};
use crate::ast::expression::function_literal::FunctionLiteral;

pub mod boolean;
pub mod if_expression;
pub mod infix_expression;
pub mod integer_literal;
pub mod prefix_expression;
pub mod function_literal;

#[derive(Debug, Clone)]
pub enum Expression {
    PrefixExpression(PrefixExpression),
    InfixExpression(InfixExpression),
    IntegerLiteralExpression(IntegerLiteral),
    IdentifierExpression(Identifier),
    BooleanExpression(Boolean),
    IfExpression(IfExpression),
    FunctionLiteral(FunctionLiteral),
}

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::PrefixExpression(pre_exp) => write!(f, "{}", pre_exp),
            Expression::InfixExpression(infix_exp) => write!(f, "{}", infix_exp),
            Expression::IntegerLiteralExpression(integ_exp) => write!(f, "{}", integ_exp),
            Expression::IdentifierExpression(ident) => write!(f, "{}", ident),
            Expression::BooleanExpression(boolean) => write!(f, "{}", boolean),
            Expression::IfExpression(if_exp) => write!(f, "{}", if_exp),
            Expression::FunctionLiteral(fun_exp) => write!(f, "{}", fun_exp),
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
            Self::IfExpression(if_exp) => if_exp.token_literal(),
            Self::FunctionLiteral(fun_exp) => fun_exp.token_literal(),
        }
    }
}

impl From<PrefixExpression> for Expression {
    fn from(pre_exp: PrefixExpression) -> Self {
        println!("PrefixExpression convert to Expression: {:?}", pre_exp);
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

impl From<IfExpression> for Expression {
    fn from(if_exp: IfExpression) -> Self {
        Self::IfExpression(if_exp)
    }
}

impl From<FunctionLiteral> for Expression {
    fn from(fn_exp: FunctionLiteral) -> Self {
        Self::FunctionLiteral(fn_exp)
    }
}
