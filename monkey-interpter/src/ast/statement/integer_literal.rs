use crate::ast::statement::expression_statement::ExpressionStatement;
use crate::ast::statement::{Expression, Node};
use crate::ast::Identifier;
use crate::token::token_type::TokenType;
use crate::token::Token;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub struct IntegerLiteral {
    pub token: Token,
    pub value: i64,
}

impl IntegerLiteral {
    pub fn new(token: Token) -> Self {
        Self {
            token,
            value: i64::default(),
        }
    }
}

impl Display for IntegerLiteral {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.token.literal.clone())
    }
}

impl Node for IntegerLiteral {
    fn token_literal(&self) -> String {
        print!("[integer literal] token_literal");
        format!("{}", self.value)
    }
}
impl Expression for IntegerLiteral {
    fn expression_node(&self) {}
}

impl TryFrom<ExpressionStatement> for IntegerLiteral {
    type Error = anyhow::Error;

    fn try_from(expression_statement: ExpressionStatement) -> Result<Self, Self::Error> {
        let identifier = Identifier::try_from(expression_statement.expression)?;
        let value = identifier.value.parse::<i64>()?;

        Ok(Self {
            token: expression_statement.token.clone(),
            value,
        })
    }
}

impl TryFrom<Box<dyn Expression>> for IntegerLiteral {
    type Error = anyhow::Error;

    fn try_from(value: Box<dyn Expression>) -> Result<Self, Self::Error> {
        let temp_value = value.token_literal();
        println!("[integer_literal] try_from temp_value: {:?}", temp_value);

        Ok(Self {
            token: Token::from_string(TokenType::INT, value.token_literal()),
            value: value.token_literal().parse::<i64>()?,
        })
    }
}
