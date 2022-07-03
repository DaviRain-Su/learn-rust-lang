use crate::ast::statement::{Expression, Node, Statement};
use crate::ast::Identifier;
use crate::token::token_type::TokenType;
use crate::token::Token;
use std::fmt::{Display, Formatter};

/// expression statement
/// ExpressionStatement 类型具有两个字段，分别是每个节点都具有的token字段
/// 和保存表达的expression字段。
#[derive(Debug)]
pub struct ExpressionStatement {
    pub token: Token, // 该表达式中的第一个词法单元
    pub expression: Box<dyn Expression>,
}

impl Default for ExpressionStatement {
    fn default() -> Self {
        Self {
            token: Token::default(),
            expression: Box::new(Identifier::default()),
        }
    }
}

impl Display for ExpressionStatement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{};", self.expression)
    }
}

impl Node for ExpressionStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

/// ExpressionStatement 实现了 Statement 接口，这意味着表达式语句可以添加到Program
/// 的statements vector中。
impl Statement for ExpressionStatement {
    fn statement_node(&self) {}

    fn identifier(&self) -> Identifier {
        Identifier::from(self.token.clone())
    }
}

impl Expression for ExpressionStatement {
    fn expression_node(&self) {}
}

impl From<&Box<dyn Statement>> for ExpressionStatement {
    fn from(value: &Box<dyn Statement>) -> Self {
        Self {
            token: Token::from_string(TokenType::IDENT, value.token_literal()),
            expression: Box::new(value.identifier()),
        }
    }
}
