pub mod expression;
pub mod statement;

#[cfg(test)]
mod tests;

use crate::ast::expression::Expression;
use crate::ast::statement::Statement;
use crate::token::token_type::TokenType;
use crate::token::Token;
use std::any::Any;
use std::fmt::{Debug, Display, Formatter};

pub trait Node: Debug + Display {
    /// 必须提供 TokenLiteral()方法，该方法返回与其
    /// 关联的词法单元的字面量
    fn token_literal(&self) -> String;
}

/// 这个 Program 节点将成为语法分析器生成的每个 AST 的根节点。每个有效的
/// Monkey 程序都是一系列位于 Program.Statements 中的语句。Program.Statements
/// 是一个切片，其中有实现 Statement 接口的 AST 节点。
#[derive(Debug)] // add debug trait for debug
pub struct Program {
    pub(crate) statements: Vec<Statement>,
}

impl Display for Program {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for statement in self.statements.iter() {
            write!(f, "{}", statement)?;
        }

        Ok(())
    }
}

impl Program {
    pub fn new() -> Self {
        Self { statements: vec![] }
    }

    pub fn token_literal(&self) -> String {
        if self.statements.is_empty() {
            return String::new();
        } else {
            return self
                .statements
                .first()
                .expect("never failed")
                .token_literal();
        }
    }

    pub fn len(&self) -> usize {
        self.statements.len()
    }
}

#[derive(Debug, Default, Clone)]
pub struct Identifier {
    pub token: Token, // token.IDENT 词法单元
    pub value: String,
}

impl Display for Identifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Identifier {
    pub fn new(token: Token, value: String) -> Self {
        Self { token, value }
    }
}

impl From<Token> for Identifier {
    fn from(token: Token) -> Self {
        Self {
            token: token.clone(),
            value: token.literal.clone(),
        }
    }
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

// impl From<Box<dyn Expression>> for Identifier {
//     fn from(value: Box<dyn Expression>) -> Self {
//         Self {
//             token: Token::from_string(TokenType::IDENT, value.token_literal()),
//             value: value.token_literal(),
//         }
//     }
// }

impl From<Expression> for Identifier {
    fn from(expression: Expression) -> Self {
        match expression {
            Expression::IdentifierExpression(ident) => ident.clone(),
            _ => unimplemented!(),
        }
    }
}
