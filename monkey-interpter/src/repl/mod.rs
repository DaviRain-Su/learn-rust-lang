use crate::lexer::Lexer;
use crate::token::token_type::TokenType;
use std::io;
use std::io::BufRead;
use std::io::Write;

const PROMPT: &'static str = ">> ";

pub fn start(std_in: io::Stdin, mut std_out: io::Stdout) -> anyhow::Result<()> {
    let mut std_buffer_reader = io::BufReader::new(std_in);

    loop {
        let _ = std_out.write_all(PROMPT.as_ref());
        let _ = std_out.flush();

        let mut buffer_reader = String::new();
        let _line = std_buffer_reader.read_line(&mut buffer_reader);

        let mut lexer = Lexer::new(buffer_reader.as_str())?;

        let mut tok = lexer.next_token()?;

        while tok.r#type != TokenType::ILLEGAL {
            let _ = std_out.write_all(format!("{:?}\n", tok).as_ref());
            let _ = std_out.flush();

            tok = lexer.next_token()?;
        }
    }
}
