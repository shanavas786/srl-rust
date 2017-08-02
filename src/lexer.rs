use std::str::Chars;
use std::iter::Peekable;
use token::Token;
use grammar::{get_token, MAX_SPC_INDEX};
use std::ascii::AsciiExt;

#[derive(PartialEq)]
enum State {
    Character,
    Quantifier,
    Group,
    Lookaround,
    Flag,
    Anchor,
    SrcWhitespace,
    SrcNumber,
    SrcString,
    Delimiter,
    Undefined,

    Identifier,
    Number,
    String,

    None,
    EndOfFile,
    Error
}

pub struct Lexer<'a> {
    src: Peekable<Chars<'a>>,
    buffer: String,
    last_char: char,
    state: State,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &str) -> Lexer {
        Lexer {
            src: src.chars().peekable(),
            buffer: String::new(),
            last_char: ' ',
            state: State::None
        }
    }

    /// sets last char to ch
    fn last_char(&mut self, ch: char) {
        self.last_char = ch;
    }

    /// reset buffer to empty string
    fn reset_buffer(&mut self) {
        self.buffer = String::new();
    }

    /// check if lexer is in Identifier state
    fn is_ident(&self) -> bool {
        self.state == State::Identifier
    }

    /// sets identifer state
    fn set_ident(&mut self) {
        self.state = State::Identifier
    }

    /// Checks if eof is reached
    fn is_eof(&self) -> bool {
        self.state == State::EndOfFile
    }

    /// sets identifer state
    fn set_eof(&mut self) {
        self.state = State::EndOfFile
    }

    /// sets error state
    fn set_error(&mut self) {
        self.state = State::Error
    }

    /// Checks if char is src whitespace
    fn is_src_space(&mut self, ch: char) -> bool {
        match ch {
            ' ' => {
                // skip space if buffer is empty or last char is space
                self.is_ident() && (self.buffer.is_empty() || self.last_char == ' ')
            },
            ',' | '\n' | '\t' => { self.is_ident() && self.buffer.is_empty() },
            _ => { false }
        }
    }

    /// checks if ch is one of whitespace chars allowed in token
    /// eg: any\nof is a valid identifier
    fn is_token_whitespace(&self, ch: char) -> bool {
        (ch == ' ') || (ch == ',') || (ch == '\n') || (ch == '\t')
    }

    /// Returns next identifier Token
    fn next_identifier(&mut self) -> Option<Token> {
        self.set_ident();

        loop {
            if let Some(ch) = self.src.next() {
                if self.is_src_space(ch) {
                    continue;
                }

                if ch.is_ascii() && ch.is_alphabetic() {
                    self.buffer.push(ch);
                    self.last_char(ch);
                } else if self.is_token_whitespace(ch) {
                    if let Some(token) = get_token(&self.buffer) {
                        // valid token !!
                        // TODO set next state
                        self.reset_buffer();
                        return Some(token);
                    } else if self.buffer.len() < MAX_SPC_INDEX {
                        // add space to token
                        self.buffer.push(' ');
                        self.last_char(' ');
                    } else {
                        // invalid identifier
                        self.set_error();
                        break;
                    }
                } else {
                    // invalid char in identifier
                    self.set_error();
                    break;
                }
            } else {
                // check buffer is valid token else set err state
                self.set_eof();
                break;
            }
        }
        None
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        match self.state {
            State::None | State::Identifier => { self.next_identifier() },
            _ => { None },
        }
    }
}
