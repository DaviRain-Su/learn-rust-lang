use crate::token::Token;
use crate::token::token_type::TokenType;

#[cfg(test)]
mod tests;


pub struct Lexer {
    input: String,
    position: usize, // 所输入字符串中的当前位置（指向当前字符）
    read_position: usize, // 所输入字符串中的当前读取位置（指向当前字符之后的一个字符）
    ch: char, // 当前正在查看的字符
}


impl Lexer {
    pub fn new(input: &str) -> Self {
        let mut lexer=Self {
            input: String::from(input),
            position: 0,
            read_position: 0,
            ch: 0 as char,
        };

        lexer.read_char();

        lexer
    }

    //TODO, only support ASCII, not supported Unicode
    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0 as char;
        } else {
            self.ch = (*self.input.get(self.read_position..self.read_position + 1).unwrap()).parse().unwrap();
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> Token {
        let mut tok = Token::default();
        match self.ch {
            '=' => {
                tok = Token::new(TokenType::ASSIGN, self.ch);
            }
            ';' => {
                tok = Token::new(TokenType::SEMICOLON, self.ch);
            }
            '(' =>  {
                tok = Token::new(TokenType::LPAREN, self.ch);
            },
            ')' =>  {
                tok = Token::new(TokenType::RPAREN,self.ch);
            },
            ',' =>  {
                tok =Token::new(TokenType::COMMA,self.ch);
            },
            '+' =>  {
                tok = Token::new(TokenType::PLUS,self.ch);
            },
            '{' => {
                tok = Token::new(TokenType::LBRACE,self.ch);
            },
            '}' =>  {
                tok = Token::new(TokenType::RBRACE, self.ch);
            },
            _ =>  {
                tok = Token::new(TokenType::EOF, 0 as char);
            },
        }

        self.read_char();

        tok
    }
}