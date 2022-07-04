mod operator_priority;
#[cfg(test)]
mod tests;

use crate::ast::statement::expression_statement::ExpressionStatement;
use crate::ast::statement::integer_literal::IntegerLiteral;
use crate::ast::statement::let_statement::LetStatement;
use crate::ast::statement::prefix_expression::PrefixExpression;
use crate::ast::statement::return_statement::ReturnStatement;
use crate::ast::statement::{Expression, Statement};
use crate::ast::{Identifier, Program};
use crate::lexer::Lexer;
use crate::parser::operator_priority::OperatorPriority;
use crate::parser::operator_priority::OperatorPriority::PREFIX;
use crate::token::token_type::TokenType;
use crate::token::Token;
use std::collections::HashMap;
use std::default::default;

/// 前缀解析函数
/// 前缀运算符左侧为空。
/// 在前缀位置遇到关联的词法单元类型时会调用 prefixParseFn
type PrefixParseFn = Box<fn(&mut Parser) -> anyhow::Result<Box<dyn Expression>>>;

/// 中缀解析函数
/// infixParseFn 接受另一个 ast.Expression 作为参数。该参数是所解析的中缀运算符
/// 左侧的内容。
/// 在中缀位置遇到词法单元类型时会调用 infixParseFn
type InferParseFn = Box<fn(&mut Parser, Box<dyn Expression>) -> Box<dyn Expression>>;

#[derive(Clone)]
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
    prefix_parse_fns: HashMap<TokenType, PrefixParseFn>,
    infix_parse_fns: HashMap<TokenType, InferParseFn>,
}

impl Parser {
    pub fn new(lexer: Lexer) -> anyhow::Result<Self> {
        let mut parser = Parser {
            lexer,
            current_token: Token::default(),
            peek_token: Token::default(),
            prefix_parse_fns: HashMap::default(),
            infix_parse_fns: HashMap::default(),
        };

        parser.register_prefix(TokenType::IDENT, Box::new(Self::parse_identifier));
        parser.register_prefix(TokenType::INT, Box::new(Self::parser_integer_literal));
        parser.register_prefix(TokenType::BANG, Box::new(Self::parse_prefix_expression));
        parser.register_prefix(TokenType::MINUS, Box::new(Self::parse_prefix_expression));

        // 读取两个词法单元，以设置 curToken 和 peekToken
        parser.next_token()?;
        parser.next_token()?;

        Ok(parser)
    }

    pub fn update_parser(&mut self, parse: Parser) {
        self.lexer = parse.lexer;
        self.current_token = parse.current_token;
        self.peek_token = parse.peek_token;
        self.prefix_parse_fns = parse.prefix_parse_fns;
        self.infix_parse_fns = parse.infix_parse_fns;
    }

    fn next_token(&mut self) -> anyhow::Result<()> {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token()?;

        Ok(())
    }

    fn parse_program(&mut self) -> anyhow::Result<Program> {
        let mut program = Program::new();

        // TODO this should be EOF, but this is ILLEGAL
        while !self.cur_token_is(TokenType::ILLEGAL) {
            // println!("current_token = {:?}", self.current_token);
            let stmt = self.parse_statement()?;
            program.statements.push(stmt);
            self.next_token()?;
        }

        Ok(program)
    }

    fn parse_statement(&mut self) -> anyhow::Result<Box<dyn Statement>> {
        match self.current_token.r#type {
            TokenType::LET => Ok(Box::new(self.parse_let_statement()?)),
            TokenType::RETURN => Ok(Box::new(self.parse_return_statement()?)),
            _ => {
                // default parse expression statement
                Ok(Box::new(self.parse_expression_statement()?))
            }
        }
    }

    /// 先来看 parseLetStatement。这里使用当前所在的词法单元（token.LET）构造
    /// 了一个*ast.LetStatement 节点，然后调用 expectPeek 来判断下一个是不是期望的
    /// 词法单元，如果是，则前移词法单元指针。第一次期望的是一个 token.IDENT 词法单
    /// 元，用于构造一个*ast.Identifier 节点。然后期望下一个词法单元是等号。之后跳
    /// 过了一些表达式，直到遇见分号为止。目前的代码跳过了表达式的处理，后面介绍完
    /// 如何解析表达式后会返回来替换这里的代码。
    ///
    /// # 解析let 语句
    fn parse_let_statement(&mut self) -> anyhow::Result<LetStatement> {
        let mut stmt = LetStatement {
            token: self.current_token.clone(),
            ..default()
        };

        if self.expect_peek(TokenType::IDENT).is_err() {
            return Ok(stmt);
        }

        stmt.name = Identifier::new(
            self.current_token.clone(),
            self.current_token.literal.clone(),
        );

        if self.expect_peek(TokenType::ASSIGN).is_err() {
            return Ok(stmt);
        }

        // TODO: 跳过对表达式的处理，直到遇见分号
        while !self.cur_token_is(TokenType::SEMICOLON) {
            self.next_token()?;
        }

        println!("stmt = {:?}", stmt);

        Ok(stmt)
    }

    /// 解析return 语句
    fn parse_return_statement(&mut self) -> anyhow::Result<ReturnStatement> {
        let stmt = ReturnStatement {
            token: self.current_token.clone(),
            ..default()
        };

        self.next_token()?;

        // TODO: 跳过对表达式的处理，直到遇见分号
        while !self.cur_token_is(TokenType::SEMICOLON) {
            self.next_token()?;
        }

        Ok(stmt)
    }

    /// 解析表达式语句
    /// 这是因为表达式语句不是真正的语句，而是仅由表达式构成的语句，相当于一层封装
    fn parse_expression_statement(&mut self) -> anyhow::Result<ExpressionStatement> {
        let mut stmt = ExpressionStatement {
            token: self.current_token.clone(),
            ..default()
        };
        println!("[parse_expression_statement] >> init stmt = {:?}", stmt);

        stmt.expression = self.parse_expression(OperatorPriority::LOWEST)?.into();

        if self.peek_token_is(TokenType::SEMICOLON) {
            self.next_token()?;
        }

        println!("[parse_expression_statement] >> stmt = {:?}", stmt);

        Ok(stmt)
    }

    /// parse expression
    fn parse_expression(
        &mut self,
        _precedence: OperatorPriority,
    ) -> anyhow::Result<Box<dyn Expression>> {
        // clone evn to temp value
        let mut parser = self.clone(); // todo
        let prefix = self.prefix_parse_fns.get(&self.current_token.r#type);

        if prefix.is_none() {
            Err(anyhow::anyhow!(format!(
                "no prefix parse function for {:?} found.",
                self.current_token.r#type.clone()
            )))
        } else {
            // FIXME: THIS IS OK
            let prefix = prefix.unwrap();

            let left_exp = prefix(&mut parser);

            // update env with temp value
            self.update_parser(parser);

            left_exp
        }
    }

    /// parse identifier
    fn parse_identifier(&mut self) -> anyhow::Result<Box<dyn Expression>> {
        Ok(Box::new(Identifier {
            token: self.current_token.clone(),
            value: self.current_token.literal.clone(),
        }))
    }

    /// parse integer literal
    fn parser_integer_literal(&mut self) -> anyhow::Result<Box<dyn Expression>> {
        let mut literal = IntegerLiteral::new(self.current_token.clone());
        let value = self.current_token.literal.parse::<i64>()?;

        literal.value = value;
        Ok(Box::new(literal))
    }

    fn parse_prefix_expression(&mut self) -> anyhow::Result<Box<dyn Expression>> {
        let mut expression = PrefixExpression {
            token: self.current_token.clone(),
            operator: self.current_token.literal.clone(),
            ..default()
        };

        self.next_token()?;

        expression.right = self.parse_expression(PREFIX)?;

        Ok(Box::new(expression))
    }

    fn cur_token_is(&self, t: TokenType) -> bool {
        self.peek_token.r#type == t
    }

    fn peek_token_is(&self, t: TokenType) -> bool {
        self.peek_token.r#type == t
    }

    /// 断言函数的主要目的是通过检查下一个词法单元的
    /// 类型，确保词法单元顺序的正确性。这里的 expectPeek 会检查 peekToken 的类型，
    /// 并且只有在类型正确的情况下，它才会调用 nextToken 前移词法单元。
    fn expect_peek(&mut self, t: TokenType) -> anyhow::Result<()> {
        if self.peek_token_is(t.clone()) {
            self.next_token()?;
            Ok(())
        } else {
            Err(anyhow::anyhow!(format!(
                "expected next token be {:?}, got {:?} instead",
                t, self.peek_token.r#type
            )))
        }
    }

    /// register prefix
    fn register_prefix(&mut self, token_type: TokenType, prefix_parse_fn: PrefixParseFn) {
        self.prefix_parse_fns.insert(token_type, prefix_parse_fn);
    }

    /// register infix
    fn register_infix(&mut self, token_type: TokenType, infix_parse_fn: InferParseFn) {
        self.infix_parse_fns.insert(token_type, infix_parse_fn);
    }
}
