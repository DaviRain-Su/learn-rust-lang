use crate::token::Token;
use crate::token::token_type::TokenType;

pub trait Node {
    /// 必须提供 TokenLiteral()方法，该方法返回与其
    /// 关联的词法单元的字面量
    fn token_literal(&self) -> String;
}

pub trait Statement: Node {
    fn statement_node(&self);
}

pub trait Expression: Node {
    fn expression_node(&self);
}

/// 这个 Program 节点将成为语法分析器生成的每个 AST 的根节点。每个有效的
/// Monkey 程序都是一系列位于 Program.Statements 中的语句。Program.Statements
/// 是一个切片，其中有实现 Statement 接口的 AST 节点。
pub struct Program {
    pub(crate) statements: Vec<Box<dyn Statement>>,
}


impl Program {
    pub fn token_literal(&self) -> String {
        if self.statements.is_empty() {
            return String::new();
        } else {
            return self.statements.first().expect("never failed").token_literal();
        }
    }

    pub fn len(&self) -> usize {
        self.statements.len()
    }
}


pub struct LetStatement {
    token: Token, // token.LET 词法单元
    pub name: Identifier,
    value: ExpressionId,
}


impl  From<&Box<dyn Statement>> for LetStatement {
    fn from(value: &Box<dyn Statement>) -> Self {
        Self {
            token: Token::from_string(TokenType::LET, "let".into()),
            name: Identifier {
                token: Token::from_string(TokenType::IDENT, value.token_literal()),
                value: value.token_literal(),
            },
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
}


pub struct Identifier {
    pub token: Token, // token.IDENT 词法单元
    pub value: String,
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

impl Expression for Identifier {
    fn expression_node(&self) {}
}

pub struct ExpressionId;
