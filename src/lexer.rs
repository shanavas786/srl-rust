use std::str::Chars;
use token::Token;
use grammar::{get_token, eof};

enum State {
    Identifier,
    Number,
    String,

    None,
    EndOfFile,
    Error
}

pub struct Lexer<'a> {
    src: Chars<'a>,
    buffer: String,
    state: State,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &str) -> Lexer {
        Lexer {
            src: src.chars(),
            buffer: String::new(),
            state: State::None
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        match self.state {
            State::None => self.next_ident(),
            State::String => self.next_string(),
            State::Number => self.next_number(),
            _ => None,
        }
    }

    fn next_ident(&mut self) -> Option<Token> {
        // if last char in buffer is space
        let space: bool = false;

        while let Some(ch) = self.src.next() {
            if ch != ' ' {
                space = false;
                self.buffer.push(ch);
            } else if space {
                // ignore repeated spaces
                continue;
            }
            if let Some(token) = get_token(&self.buffer) {
                // check if buffer matches a token
                return Some(token);
            }
        }
        Some(eof())
    }

    // TODO change return type to Result<Token, Err>
    // TODO add Token::from_string method
    fn next_string(&mut self) -> Option<Token> {
       None
    }

    fn next_number(&mut self) -> Option<Token> {
        None
    }
}
