use std::str::Chars;
use std::iter::Peekable;
use token::*;
use grammar::{MAX_SPC_INDEX, get_token, get_string_token, get_number_token, get_eof_token,
              get_char_token, get_digit_token};
use std::ascii::AsciiExt;
mod srlchar;
use self::srlchar::SrlChar;

#[derive(PartialEq)]
enum State {
    None,

    Identifier,
    String,
    Number,
    CharOrDigit,
    EndOfFile,

    Done,
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
    fn set_last_char(&mut self, ch: char) {
        self.last_char = ch;
    }

    /// set next state
    fn next_state(&mut self, token: &Token) {
        match token.token_type() {
            TokenType::Character(tk) => {
                match tk {
                    Characters::Raw | Characters::Literally | Characters::OneOf => {
                        self.set_string()
                    }
                    _ => self.set_ident(),
                }
            }
            TokenType::Quantifier(tk) => {
                match tk {
                    Quantifiers::Exactly | Quantifiers::Between | Quantifiers::And => {
                        self.set_number()
                    }
                    _ => self.set_ident(),
                }
            }
            TokenType::Specification(_) => self.set_char_or_digit(),
            TokenType::Group(tk) => {
                match tk {
                    Groups::As => self.set_string(),
                    Groups::GroupStart | Groups::GroupEnd => self.set_ident(),
                    _ => self.set_none(),
                }
            }
            _ => self.set_ident(),
        }
    }

    /// sets none state
    fn set_none(&mut self) {
        self.state = State::None
    }

    /// sets identifer state
    fn set_ident(&mut self) {
        self.state = State::Identifier
    }

    /// sets string state
    fn set_string(&mut self) {
        self.state = State::String
    }

    /// sets number state
    fn set_number(&mut self) {
        self.state = State::Number
    }

    /// set char_or_digit state
    fn set_char_or_digit(&mut self) {
        self.state = State::CharOrDigit
    }

    /// sets identifer state
    fn set_eof(&mut self) {
        self.state = State::EndOfFile
    }

    /// sets done state
    fn set_done(&mut self) {
        self.state = State::Done
    }

    /// sets error state
    fn set_error(&mut self) {
        self.state = State::Error
    }

    /// check error state
    pub fn is_error(&self) -> bool {
        self.state == State::Error
    }

    /// checks if lexer should skip srl whitespace characters
    fn skip_space(&self) -> bool {
        self.buffer.is_empty() || self.last_char.is_space()
    }

    /// Returns next identifier Token
    fn next_identifier(&mut self) -> Option<Token> {
        self.set_ident();

        while let Some(&ch) = self.src.peek() {
            if self.skip_space() && ch.is_srl_whitespace() {
                self.src.next();
                continue;
            }

            if ch.is_ascii() && ch.is_alphabetic() {
                // identifiers are case insensitive
                self.buffer.push(ch.to_ascii_lowercase());
                self.src.next();
                self.set_last_char(ch);
            } else if ch.is_srl_whitespace() || ch.is_group_char() {
                if self.buffer.is_empty() {
                    self.src.next();
                    // group char
                    self.set_last_char(ch);
                    let token = get_token(&format!("{}", ch));
                    return token;
                } else if let Some(token) = get_token(self.buffer.as_ref()) {
                    // valid token !!
                    self.buffer.clear();
                    self.set_last_char(ch);
                    self.next_state(&token);
                    return Some(token);
                } else if self.buffer.len() < MAX_SPC_INDEX {
                    self.src.next();
                    // add space to token
                    self.buffer.push(' ');
                    self.set_last_char(' ');
                } else {
                    // invalid identifier
                    self.set_error();
                    return None;
                }
            } else {
                // invalid char in identifier
                self.set_error();
                return None;
            }
        }

        if self.buffer.is_empty() {
            self.set_done();
            Some(get_eof_token())
        } else if let Some(token) = get_token(self.buffer.as_ref()) {
            // valid token !!
            self.buffer.clear();
            self.set_eof();
            Some(token)
        } else {
            // unexpected eof
            self.set_error();
            None
        }
    }

    /// Returns next string Token
    fn next_string(&mut self) -> Option<Token> {
        self.set_string();
        // ' or "
        let mut start_char = '\0';

        while let Some(ch) = self.src.next() {
            if ch.is_srl_whitespace() {
                continue;
            } else if ch.is_quote() {
                start_char = ch;
                break;
            } else {
                // expeced ' or "
                self.set_error();
                return None;
            }

        }

        if start_char == '\0' {
            self.set_error();
            return None;
        }

        while let Some(ch) = self.src.next() {
            if ch.is_backslash() {
                if self.last_char.is_backslash() {
                    self.buffer.push(ch);
                    self.set_last_char('\0');
                } else {
                    // escape for next character
                    self.set_last_char(ch);
                }
            } else if ch == start_char {
                if self.last_char.is_backslash() {
                    self.buffer.push(ch);
                    self.set_last_char('\0');
                } else {
                    // string terminated
                    let token = get_string_token(self.buffer.as_ref());
                    self.buffer.clear();
                    self.next_state(&token);
                    return Some(token);
                }
            } else {
                self.buffer.push(ch);
                self.set_last_char(ch);
            }
        }

        // unterminated string
        self.set_error();
        None
    }

    // Returns next number token
    fn next_number(&mut self) -> Option<Token> {
        self.set_number();

        while let Some(&ch) = self.src.peek() {
            if self.skip_space() && ch.is_srl_whitespace() {
                self.src.next();
                continue;
            } else if ch.is_digit(10) {
                self.src.next();
                self.buffer.push(ch);
                self.set_last_char(ch);
            } else if ch.is_srl_whitespace() || ch.is_group_char() {
                // number ends
                let token = get_number_token(self.buffer.as_ref());
                self.next_state(&token);
                self.buffer.clear();
                self.set_last_char(' ');
                return Some(token);
            } else {
                // invalid char in number
                self.set_error();
                return None;
            }
        }

        if self.buffer.is_empty() {
            self.set_error();
            return None;
        } else {
            // number ends
            let token = get_number_token(self.buffer.as_ref());
            self.buffer.clear();
            self.set_eof();
            return Some(token);
        }
    }

    /// Returns next char or digit
    fn next_char_or_digit(&mut self) -> Option<Token> {
        self.set_char_or_digit();
        while let Some(ch) = self.src.next() {
            if ch.is_srl_whitespace() {
                continue;
            } else if ch.is_ascii() && ch.is_digit(10) {
                self.buffer.push(ch);
                let token = get_digit_token(self.buffer.as_ref());
                self.buffer.clear();
                self.set_ident();
                return Some(token);
            } else if ch.is_ascii() && ch.is_alphabetic() {
                self.buffer.push(ch);
                let token = get_char_token(self.buffer.as_ref());
                self.buffer.clear();
                self.set_ident();
                return Some(token);
            } else {
                // unexpected char
                break;
            }
        }

        self.set_error();
        return None;
    }

    /// Returns next Token
    fn next_token(&mut self) -> Option<Token> {
        while let Some(&ch) = self.src.peek() {
            if ch.is_srl_whitespace() {
                self.src.next();
                continue;
            }

            if ch.is_quote() {
                return self.next_string();
            }

            if (ch.is_ascii() && ch.is_alphabetic()) || ch.is_group_char() {
                return self.next_identifier();
            }

            if ch.is_digit(10) {
                return self.next_number();
            }

            self.set_error();
            return None;

        }

        self.set_done();
        Some(get_eof_token())
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    /// Returns next token
    fn next(&mut self) -> Option<Token> {
        match self.state {
            State::None => self.next_token(),
            State::Identifier => self.next_identifier(),
            State::String => self.next_string(),
            State::Number => self.next_number(),
            State::CharOrDigit => self.next_char_or_digit(),
            State::EndOfFile => {
                self.set_done();
                Some(get_eof_token())
            }
            State::Error | State::Done => None,
        }
    }
}

#[cfg(test)]
mod test;
