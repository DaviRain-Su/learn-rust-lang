use crate::token::token_type::TokenType;
use crate::token::{token_type, Token};

#[cfg(test)]
mod tests;

/// Lexer 中的大多数字段很容易理解，但 position 和 readPosition 的含义可能让人
/// 困惑。两者都可以用作索引来访问 input 中的字符，例如 l.input[l.readPosition]。
/// 这里之所以用两个“指针”来指向所输入的字符串，是因为词法分析器除了查看当前
/// 字符，还需要进一步“查看”字符串，即查看字符串中的下一个字符。readPosition
/// 始终指向所输入字符串中的“下一个”字符，position 则指向所输入字符串中与 ch
/// 字节对应的字符。
#[derive(Debug)]
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

    /// readChar 的目的是读取 input 中的下一个字符，并前移其在 input 中的位置。
    /// 这个过程的第一件事就是检查是否已经到达 input 的末尾。如果是，则将 l.ch 设置为 0，
    /// 这是 NUL 字符的 ASCII 编码，用来表示“尚未读取任何内容”或“文件结尾”。如果还
    /// 没有到达 input 的末尾，则将 l.ch 设置为下一个字符，即 l.input[l.readPosition]
    /// 指向的字符.
    /// TODO, 在谈到 readChar 时，值得指出的是，该词法分析器仅支持 ASCII 字符，不能
    /// 支持所有的 Unicode 字符。
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

    /// 这就是 NextToken()方法的基本结构。它首先检查了当前正在查看的字符 l.ch，
    /// 根据具体的字符来返回对应的词法单元。在返回词法单元之前，位于所输入字符串中
    /// 的指针会前移，所以之后再次调用 NextToken()时，l.ch 字段就已经更新过了。
    pub fn next_token(&mut self) -> Token {
        let mut tok = Token::default();

        // skip whitespace
        self.skip_whitespace();

        match self.ch {
            '=' => {
                if self.peek_char() == '=' {
                    let ch = self.ch;
                    self.read_char();
                    let literal = String::from(ch) + &String::from(self.ch);
                    tok = Token::from_string(TokenType::EQ, literal);
                } else {
                    tok = Token::from_char(TokenType::ASSIGN, self.ch);
                }
            }
            '-' => {
                tok = Token::from_char(TokenType::MINUS, self.ch);
            }
            '!' => {
                if self.peek_char() == '=' {
                    let ch = self.ch;
                    self.read_char();
                    let literal = String::from(ch) + &String::from(self.ch);
                    tok = Token::from_string(TokenType::NOTEQ, literal);
                } else {
                    tok = Token::from_char(TokenType::BANG, self.ch);
                }
            }
            '/' => {
                tok = Token::from_char(TokenType::SLASH, self.ch);
            }
            '*' => {
                tok = Token::from_char(TokenType::ASTERISK, self.ch);
            }
            '<' => {
                tok = Token::from_char(TokenType::LT, self.ch);
            }
            '>' => {
                tok = Token::from_char(TokenType::GT, self.ch);
            }
            ';' => {
                tok = Token::from_char(TokenType::SEMICOLON, self.ch);
            }
            '(' => {
                tok = Token::from_char(TokenType::LPAREN, self.ch);
            }
            ')' => {
                tok = Token::from_char(TokenType::RPAREN, self.ch);
            }
            ',' => {
                tok = Token::from_char(TokenType::COMMA, self.ch);
            }
            '+' => {
                tok = Token::from_char(TokenType::PLUS, self.ch);
            }
            '{' => {
                tok = Token::from_char(TokenType::LBRACE, self.ch);
            }
            '}' => {
                tok = Token::from_char(TokenType::RBRACE, self.ch);
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

    /// 先处理标识符和关键字。对于这两者，词法分析器需要识别当前字符是否为字母。
    /// 如果是，则还需要读取标识符/关键字的剩余部分，直到遇见非字母字符为止。读取完
    /// 该标识符/关键字之后，还需要判断它到底是标识符还是关键字，以便使用正确的
    /// token.TokenType。
    /// readIdentifier()函数顾名思义，就是读入一个标识符并前移词法分析器的扫描
    /// 位置，直到遇见非字母字符。
    fn read_identifier(&mut self) -> &str {
        let position = self.position;
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
        let position = self.position;
        while Self::is_digit(self.ch) {
            self.read_char();
        }

        let number = self.input.get(position..self.position).unwrap();
        number
    }

    /// isDigit 函数与 isLetter 一样简单，只是判断传入的内容是否为 Latin 字符集中
    /// 0 和 9 之间的数字。
    fn is_digit(ch: char) -> bool {
        '0' <= ch && ch <= '9'
    }

    /// isLetter 辅助函数用来判断给定的参数是否为字母
    /// 示例中包含 ch =='_'，这意味着下划线_会被视为字母，允许在标识符和关键字中使用
    fn is_letter(ch: char) -> bool {
        'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || ch == '_'
    }

    /// 但这个函数不会前移 l.position 和
    /// l.readPosition。它的目的只是窥视一下输入中的下一个字符，不会移动位于输入中
    /// 的指针位置，这样就能知道下一步在调用 readChar()时会返回什么。
    fn peek_char(&self) -> char {
        if self.read_position >= self.input.len() {
            return 0 as char;
        } else {
            return self
                .input
                .get(self.read_position..self.read_position + 1)
                .unwrap()
                .parse()
                .unwrap();
        }
    }
}
