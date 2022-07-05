pub mod expression_statement;
pub mod integer_literal;
pub mod let_statement;
pub mod prefix_expression;
pub mod return_statement;

use std::any::Any;
use crate::ast::Identifier;
use std::fmt::{Debug, Display};

pub trait Node: Debug + Display {
    /// 必须提供 TokenLiteral()方法，该方法返回与其
    /// 关联的词法单元的字面量
    fn token_literal(&self) -> String;

    fn as_any(&self) -> &dyn Any;
}

pub trait Statement: Node {
    fn statement_node(&self);

    // must be have this function
    fn identifier(&self) -> Identifier;
}

pub trait Expression: Node {
    fn expression_node(&self);
}
