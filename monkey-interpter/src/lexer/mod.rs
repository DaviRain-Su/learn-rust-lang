use crate::token::token_type::TokenType;
use crate::token::{token_type, Token};

#[cfg(test)]
mod tests;

pub struct Lexer {
    input: String,
    position: usize,      // 所输入字符串中的当前位置（指向当前字符）
    read_position: usize, // 所输入字符串中的当前读取位置（指向当前字符之后的一个字符）
    ch: char,             // 当前正在查看的字符
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let mut lexer = Self {
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
            self.ch = (*self
                .input
                .get(self.read_position..self.read_position + 1)
                .unwrap())
            .parse()
            .unwrap();
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> Token {
        let mut tok = Token::default();

        // skip whitespace
        self.skip_whitespace();

        match self.ch {
            '=' => {
                tok = Token::new(TokenType::ASSIGN, self.ch);
            }
            ';' => {
                tok = Token::new(TokenType::SEMICOLON, self.ch);
            }
            '(' => {
                tok = Token::new(TokenType::LPAREN, self.ch);
            }
            ')' => {
                tok = Token::new(TokenType::RPAREN, self.ch);
            }
            ',' => {
                tok = Token::new(TokenType::COMMA, self.ch);
            }
            '+' => {
                tok = Token::new(TokenType::PLUS, self.ch);
            }
            '{' => {
                tok = Token::new(TokenType::LBRACE, self.ch);
            }
            '}' => {
                tok = Token::new(TokenType::RBRACE, self.ch);
            }
            _ => {
                if Self::is_letter(self.ch) {
                    let literal = self.read_identifier();
                    tok.r#type = token_type::lookup_ident(literal);
                    tok.literal = literal.into();
                    return tok;
                } else if Self::is_digit(self.ch) {
                    tok.r#type = TokenType::INT;
                    tok.literal = self.read_number().into();
                    return tok;
                } else {
                    tok = Token::new(TokenType::ILLEGAL, self.ch);
                }
            }
        }

        self.read_char();

        tok
    }

    fn read_identifier(&mut self) -> &str {
        let mut position = self.position;
        while Self::is_letter(self.ch) {
            self.read_char();
        }

        let literal = self.input.get(position..self.position).unwrap();

        literal
    }

    fn skip_whitespace(&mut self) {
        loop {
            if self.ch == ' ' || self.ch == '\t' || self.ch == '\n' || self.ch == '\r' {
                self.read_char();
            } else {
                break;
            }
        }
    }

    fn read_number(&mut self) -> &str {
        let mut position = self.position;
        while Self::is_digit(self.ch) {
            self.read_char();
        }

        let number = self.input.get(position..self.position).unwrap();
        number
    }

    fn is_digit(ch: char) -> bool {
        '0' <= ch && ch <= '9'
    }

    fn is_letter(ch: char) -> bool {
        'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || ch == '_'
    }
}
