use std::fmt::{Debug, Display, Formatter};
use crate::token::token_type::TokenType;
use crate::token::Token;

pub trait Node: Debug + Display {
    /// 必须提供 TokenLiteral()方法，该方法返回与其
    /// 关联的词法单元的字面量
    fn token_literal(&self) -> String;
}

pub trait Statement: Node {
    fn statement_node(&self);

    // must be have this function
    fn identifier(&self) -> Identifier;
}

pub trait Expression: Node {
    fn expression_node(&self);
}

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

/// let statement
#[derive(Debug, Default)]
pub struct LetStatement {
    pub token: Token, // token.LET 词法单元
    pub name: Identifier,
    pub value: Identifier,
}

impl From<&Box<dyn Statement>> for LetStatement {
    fn from(value: &Box<dyn Statement>) -> Self {
        Self {
            token: Token::from_string(TokenType::LET, "let".into()),
            name: value.identifier().clone(),
            value: value.identifier().clone(),
        }
    }
}

impl Display for LetStatement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} = {};", self.token_literal(), self.name, self.value)
    }
}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

impl Statement for LetStatement {
    fn statement_node(&self) {}

    fn identifier(&self) -> Identifier {
        self.name.clone()
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
    fn from(token : Token) -> Self {
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


/// return statement
#[derive(Debug, Default)]
pub struct ReturnStatement {
    pub token: Token, //  return 词法单元
    pub return_value: Identifier,
}

impl Display for ReturnStatement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"{} {};", self.token_literal(), self.return_value)
    }
}

impl Node for ReturnStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

impl Statement for ReturnStatement {
    fn statement_node(&self) {}

    fn identifier(&self) -> Identifier {
        Identifier::from(self.token.clone())
    }
}

impl From<Box<dyn Statement>> for ReturnStatement {
    fn from(value: Box<dyn Statement>) -> Self {
        Self {
            token: Token::from_string(TokenType::LET, value.token_literal()),
            return_value: value.identifier().clone(),
        }
    }
}

/// expression statement
#[derive(Debug, Default)]
pub struct ExpressionStatement {
    pub token: Token,  // 该表达式中的第一个词法单元
    pub expression: Identifier,
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

impl Statement for ExpressionStatement {
    fn statement_node(&self) {}

    fn identifier(&self) -> Identifier {
        Identifier::from(self.token.clone())
    }
}

impl From<&Box<dyn Statement>> for ExpressionStatement {
    fn from(value: &Box<dyn Statement>) -> Self {
        Self {
            // TODO ILLEGAL
            token: Token::from_string(TokenType::ILLEGAL, value.token_literal()),
            expression: value.identifier().clone(),
        }
    }
}



#[cfg(test)]
mod tests {
    use crate::ast::{Identifier, LetStatement, Program};
    use crate::token::Token;
    use crate::token::token_type::TokenType;

    #[test]
    #[ignore]
    fn test_display() {
        let let_statement = LetStatement {
            token: Token::from_string(TokenType::LET, "let".into()),
            name: Identifier {
                token: Token::from_string(TokenType::IDENT, "myVar".into()),
                value: "myVar".into(),
            },
            value: Identifier {
                token: Token::from_string(TokenType::IDENT, "anotherVar".into()),
                value: "anotherVar".into(),
            },
        };

        println!("let statement debug = {:#?}", let_statement);
        println!("let statement display = {}",let_statement);

        let program = Program {
            statements: vec![Box::new(let_statement)]
        };

        println!("program debug = {:#?}", program);
        println!("program display = {}", program);

        assert_eq!(format!("{}", program), "let myVar = anotherVar;");
    }
}
