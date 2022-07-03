use std::fmt::Debug;
use crate::token::token_type::TokenType;
use crate::token::Token;

pub trait Node: Debug {
    /// 必须提供 TokenLiteral()方法，该方法返回与其
    /// 关联的词法单元的字面量
    fn token_literal(&self) -> String;
}

pub trait Statement: Node {
    fn statement_node(&self);

    // must be have this function
    fn identifier(&self) -> &Identifier;
}

pub trait Expression: Node {
    fn expression_node(&self);
}

/// 这个 Program 节点将成为语法分析器生成的每个 AST 的根节点。每个有效的
/// Monkey 程序都是一系列位于 Program.Statements 中的语句。Program.Statements
/// 是一个切片，其中有实现 Statement 接口的 AST 节点。
#[derive(Debug)] // add debug trait for debug
pub struct Program {
    pub(crate) statements: Vec<Box<dyn Statement>>,
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

/// let statement
#[derive(Debug, Default)]
pub struct LetStatement {
    pub token: Token, // token.LET 词法单元
    pub name: Identifier,
    pub value: ExpressionId,
}

impl From<&Box<dyn Statement>> for LetStatement {
    fn from(value: &Box<dyn Statement>) -> Self {
        Self {
            token: Token::from_string(TokenType::LET, "let".into()),
            name: value.identifier().clone(),
            value: ExpressionId,
        }
    }
}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

impl Statement for LetStatement {
    fn statement_node(&self) {}

    fn identifier(&self) -> &Identifier {
        &self.name
    }
}

#[derive(Debug, Default, Clone)]
pub struct Identifier {
    pub token: Token, // token.IDENT 词法单元
    pub value: String,
}

impl Identifier {
    pub fn new(token: Token, value: String) -> Self {
        Self { token, value }
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

#[derive(Debug, Default)]
pub struct ExpressionId;

/// return statement
#[derive(Debug, Default)]
pub struct ReturnStatement {
    pub token: Token, //  return 词法单元
    pub return_value: ExpressionId,
}

impl Node for ReturnStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

impl Statement for ReturnStatement {
    fn statement_node(&self) {}

    fn identifier(&self) -> &Identifier {
        todo!()
    }
}

impl From<Box<dyn Statement>> for ReturnStatement {
    fn from(value: Box<dyn Statement>) -> Self {
        Self {
            token: Token::from_string(TokenType::LET, value.token_literal()),
            return_value: ExpressionId,
        }
    }
}

/// expression statement
#[derive(Debug)]
pub struct ExpressionStatement {
    pub token: Token,  // 该表达式中的第一个词法单元
    expression: ExpressionId,
}

impl Node for ExpressionStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

impl Statement for ExpressionStatement {
    fn statement_node(&self) {
        todo!()
    }

    fn identifier(&self) -> &Identifier {
        todo!()
    }
}
