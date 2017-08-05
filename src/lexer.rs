use std::str::Chars;
use std::iter::Peekable;
use token::{Token, TokenType};
use grammar::{MAX_SPC_INDEX, get_token, get_string_token, get_number_token};
use std::ascii::AsciiExt;

#[derive(PartialEq)]
enum State {
    Identifier,
    String,
    Number,

    None,
    EndOfFile,
    Error,
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
            state: State::None,
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

    /// set next state
    fn next_state(&mut self, token: &Token) {
        // TODO may require last token to reduce number of states
        match token.token_type() {
            TokenType::BeginWith |
            TokenType::Letter |
            TokenType::UppercaseLetter |
            TokenType::AnyCharacter |
            TokenType::Digit |
            TokenType::Number |
            TokenType::NoCharacter => self.set_ident(),
            TokenType::Raw | TokenType::Literally | TokenType::OneOf => self.set_string(),
            TokenType::Exactly | TokenType::Between => self.set_number(),
            _ => {}
        }
    }

    /// check if lexer is in Identifier state
    fn is_ident(&self) -> bool {
        self.state == State::Identifier
    }

    /// sets identifer state
    fn set_ident(&mut self) {
        self.state = State::Identifier
    }

    /// sets string state
    fn set_string(&mut self) {
        self.state = State::String
    }

    /// sets string state
    fn is_string(&self) -> bool {
        self.state == State::String
    }

    /// sets number state
    fn set_number(&mut self) {
        self.state = State::Number
    }

    /// check number state
    fn is_number(&self) -> bool {
        self.state == State::Number
    }

    /// sets identifer state
    fn set_eof(&mut self) {
        self.state = State::EndOfFile
    }

    /// check error state
    #[allow(dead_code)]
    fn is_error(&self) -> bool {
        self.state == State::Error
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
                if self.is_ident() {
                    (self.buffer.is_empty() || self.last_char == ' ')
                } else if self.is_string() || self.is_number() {
                    self.buffer.is_empty()
                } else {
                    false
                }
            }
            ',' | '\n' | '\t' => self.is_ident() && self.buffer.is_empty(),
            _ => false,
        }
    }

    /// checks if ch is one of whitespace chars
    fn is_whitespace_char(&self, ch: char) -> bool {
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
                    // identifiers are case insensitive
                    self.buffer.push(ch.to_ascii_lowercase());
                    self.last_char(ch);
                } else if self.is_whitespace_char(ch) {
                    if let Some(token) = get_token(&self.buffer) {
                        // valid token !!
                        self.reset_buffer();
                        self.next_state(&token);
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

    /// Returns next string Token
    fn next_string(&mut self) -> Option<Token> {
        self.set_string();
        // ' or "
        #[allow(unused_assignments)]
        let mut start_char = '\0';

        loop {
            if let Some(ch) = self.src.next() {
                if self.is_src_space(ch) {
                    continue;
                } else if (ch == '\'') || (ch == '"') {
                    start_char = ch;
                    break;
                } else {
                    // expeced ' or "
                    self.set_error();
                    return None;
                }

            } else {
                self.set_error();
                return None;
            }
        }

        loop {
            if let Some(ch) = self.src.next() {
                if ch == '\\' {
                    if self.last_char == '\\' {
                        self.buffer.push(ch);
                        self.last_char('\0');
                    } else {
                        // escape for next character
                        self.last_char(ch);
                    }
                } else if ch == start_char {
                    if self.last_char == '\\' {
                        self.buffer.push(ch);
                        self.last_char('\0');
                    } else {
                        // string terminated
                        let token = get_string_token(self.buffer.as_ref());
                        self.reset_buffer();
                        // correct ?
                        self.set_ident();
                        return Some(token);
                    }
                } else {
                    self.buffer.push(ch);
                    self.last_char(ch);
                }
            } else {
                // unterminated string
                self.set_error();
                break;
            }
        }

        None
    }

    // Returns next number token
    fn next_number(&mut self) -> Option<Token> {
        self.set_number();

        loop {
            if let Some(ch) = self.src.next() {
                if self.is_src_space(ch) {
                    continue;
                } else if ch.is_digit(10) {
                    self.buffer.push(ch);
                } else if self.is_whitespace_char(ch) {
                    // number ends
                    let token = get_number_token(self.buffer.as_ref());
                    self.next_state(&token);
                    self.reset_buffer();
                    self.last_char(' ');
                    return Some(token);
                }
            } else {
                self.set_error();
                return None;
            }
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        match self.state {
            State::None | State::Identifier => self.next_identifier(),
            State::String => self.next_string(),
            State::Number => self.next_number(),
            _ => None,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_identifier() {
        let mut lx = Lexer::new("bEgin with literally \"a\" exactly twice");
        let token1 = lx.next_identifier().unwrap();
        let token2 = lx.next_identifier().unwrap();
        assert_eq!(token1.val(), "begin with");
        assert_eq!(token2.val(), "literally");
        assert!(match token1.token_type() {
            TokenType::BeginWith => true,
            _ => false,
        });
        assert!(match token2.token_type() {
            TokenType::Literally => true,
            _ => false,
        });
    }

    #[test]
    fn test_next_string() {
        let mut lx = Lexer::new("\"first string\" 'second' \"esca\\\"ped1\" 'escaped\\'2'");
        let token1 = lx.next_string().unwrap();
        assert_eq!(token1.val(), "first string");
        assert!(match token1.token_type() {
            TokenType::String => true,
            _ => false,
        });
        let token2 = lx.next_string().unwrap();
        assert_eq!(token2.val(), "second");

        let token3 = lx.next_string().unwrap();
        assert_eq!(token3.val(), "esca\"ped1");

        let token4 = lx.next_string().unwrap();
        assert_eq!(token4.val(), "escaped'2");

        lx.next_string();
        assert!(lx.is_error());

        let mut lx2 = Lexer::new("\"unterminated ");
        lx2.next_string();
        assert!(lx2.is_error());
    }


    #[test]
    fn test_next_number() {
        let mut lx = Lexer::new("112 28, 28a");
        let token1 = lx.next_number().unwrap();
        assert_eq!(token1.val(), "112");
        assert!(match token1.token_type() {
            TokenType::Number => true,
            _ => false,
        });

        let token2 = lx.next_number().unwrap();
        assert_eq!(token2.val(), "28");

        lx.next_number();
        assert!(lx.is_error());
    }
}
