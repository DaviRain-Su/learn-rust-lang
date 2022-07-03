pub mod statement;

#[cfg(test)]
mod tests;

use crate::ast::statement::{Expression, Node, Statement};
use crate::token::token_type::TokenType;
use crate::token::Token;
use std::fmt::{Debug, Display, Formatter};

/// 解析函数
pub trait ParseFunction {
    /// 前缀解析函数
    /// 前缀运算符左侧为空。
    /// 在前缀位置遇到关联的词法单元类型时会调用 prefixParseFn
    fn prefix_parse_fn(&self) -> Box<dyn Expression>;

    /// 中缀解析函数
    /// infixParseFn 接受另一个 ast.Expression 作为参数。该参数是所解析的中缀运算符
    /// 左侧的内容。
    /// 在中缀位置遇到词法单元类型时会调用 infixParseFn
    fn infix_parse_fn(&self, expression: Box<dyn Expression>) -> Box<dyn Expression>;
}

/// 这个 Program 节点将成为语法分析器生成的每个 AST 的根节点。每个有效的
/// Monkey 程序都是一系列位于 Program.Statements 中的语句。Program.Statements
/// 是一个切片，其中有实现 Statement 接口的 AST 节点。
#[derive(Debug)] // add debug trait for debug
pub struct Program {
    pub(crate) statements: Vec<Box<dyn Statement>>,
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

impl Expression for Identifier {
    fn expression_node(&self) {}
}

impl From<Box<dyn Expression>> for Identifier {
    fn from(value: Box<dyn Expression>) -> Self {
        Self {
            token: Token::from_string(TokenType::IDENT, value.token_literal()),
            value: value.token_literal(),
        }
    }
}
