#[cfg(test)]
mod tests;

use crate::ast::{Identifier, LetStatement, Program, Statement};
use crate::lexer::Lexer;
use crate::token::token_type::TokenType;
use crate::token::Token;
use std::default::default;

#[derive(Debug)]
pub struct Parser {
    /// lexer 是指向词法分析器实例的指针，在该实例上重复调用NextToken()能不断获取输入中的下一个词法单元
    lexer: Lexer,
    /// curToken和 peekToken 的行为与词法分析器中的两个“指针”position 和 readPosition 完全
    /// 相同，但它们分别指向输入中的当前词法单元和下一个词法单元，而不是输入中的字
    /// 符。查看 curToken（当前正在检查的词法单元）是为了决定下
    /// 一步该怎么做，如果 curToken 没有提供足够的信息，还需要根据 peekToken 来做决
    /// 策。
    current_token: Token,
    peek_token: Token,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        let mut parser = Parser {
            lexer,
            current_token: Token::default(),
            peek_token: Token::default(),
        };

        // 读取两个词法单元，以设置 curToken 和 peekToken
        parser.next_token();
        parser.next_token();

        parser
    }

    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    fn parse_program(&mut self) -> Option<Program> {
        let mut program = Program::new();

        // TODO this should be EOF, but this is ILLEGAL
        while self.current_token.r#type != TokenType::ILLEGAL {
            // println!("current_token = {:?}", self.current_token);
            let stmt = self.parse_statement();
            if stmt.is_some() {
                program.statements.push(stmt.unwrap());
            }
            self.next_token();
        }

        Some(program)
    }

    fn parse_statement(&mut self) -> Option<Box<dyn Statement>> {
        match self.current_token.r#type {
            TokenType::LET => Some(Box::new(self.parse_let_statement().unwrap())),
            _ => None,
        }
    }

    fn parse_let_statement(&mut self) -> Option<LetStatement> {
        let mut stmt = LetStatement {
            token: self.current_token.clone(),
            ..default()
        };

        if !self.expect_peek(TokenType::IDENT) {
            return None;
        }

        stmt.name = Identifier::new(
            self.current_token.clone(),
            self.current_token.literal.clone(),
        );

        if !self.expect_peek(TokenType::ASSIGN) {
            return None;
        }

        // TODO: 跳过对表达式的处理，直到遇见分号
        while !self.cur_token_is(TokenType::SEMICOLON) {
            self.next_token();
        }

        println!("stmt = {:?}", stmt);

        Some(stmt)
    }

    fn cur_token_is(&self, t: TokenType) -> bool {
        self.peek_token.r#type == t
    }

    fn peek_token_is(&self, t: TokenType) -> bool {
        self.peek_token.r#type == t
    }

    fn expect_peek(&mut self, t: TokenType) -> bool {
        if self.peek_token_is(t) {
            self.next_token();
            true
        } else {
            false
        }
    }
}
