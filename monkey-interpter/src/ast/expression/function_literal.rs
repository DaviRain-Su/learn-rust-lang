use std::fmt::{Display, Formatter};
use crate::ast::{Identifier, Node};
use crate::ast::statement::block_statement::BlockStatement;
use crate::token::Token;
use string_join::Join;
use crate::ast::expression::Expression;

#[derive(Debug, Clone)]
pub struct FunctionLiteral {
    pub token: Token, // 'fn' 词法单元
    pub parameters: Vec<Identifier>,
    pub body: BlockStatement,
}

impl Display for FunctionLiteral {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let parameters = self
            .parameters
            .iter()
            .map(|value| format!("{}", value))
            .collect::<Vec<_>>();

        let parameters = ",".join(parameters);
        write!(f, "{}", self.token_literal())?;
        write!(f, "(")?;
        write!(f, "{}", parameters)?;
        write!(f, ")")?;
        write!(f, "{}", self.body)
    }
}

impl Node for FunctionLiteral {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}


impl TryFrom<Expression> for FunctionLiteral {
    type Error = anyhow::Error;

    fn try_from(value: Expression) -> Result<Self, Self::Error> {
        match value {
            Expression::FunctionLiteral(fun_xp) => Ok(fun_xp.clone()),
            _ => unimplemented!(),
        }
    }
}