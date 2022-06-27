#[cfg(test)]
mod tests;

use crate::ast::Program;
use crate::lexer::Lexer;
use crate::token::Token;

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

    fn parse_program(&self) -> Option<Program> {
        None
    }
}