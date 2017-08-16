use std::str::Chars;
use std::iter::Peekable;
use token::{Token, TokenType};
use grammar::{MAX_SPC_INDEX, get_token, get_string_token, get_number_token, get_eof_token,
              get_char_token, get_digit_token};
use std::ascii::AsciiExt;
mod srlchar;
use self::srlchar::SrlChar;

#[derive(PartialEq)]
enum State {
    Identifier,
    String,
    Number,
    CharOrDigit,

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
    fn set_last_char(&mut self, ch: char) {
        self.last_char = ch;
    }

    /// reset buffer to empty string
    fn reset_buffer(&mut self) {
        self.buffer = String::new();
    }

    /// set next state
    fn next_state(&mut self, token: &Token) {
        match token.token_type() {
            TokenType::Raw | TokenType::Literally | TokenType::OneOf | TokenType::As => {
                self.set_string()
            }
            TokenType::Exactly | TokenType::Between | TokenType::And => self.set_number(),
            TokenType::From | TokenType::To => self.set_char_or_digit(),
            TokenType::Capture | TokenType::Until => self.set_none(),
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

    /// check error state
    #[allow(dead_code)]
    fn is_error(&self) -> bool {
        self.state == State::Error
    }

    /// sets error state
    fn set_error(&mut self) {
        self.state = State::Error
    }

    fn skip_space(&self) -> bool {
        self.buffer.is_empty() || self.last_char.is_space()
    }

    /// Returns next identifier Token
    fn next_identifier(&mut self) -> Option<Token> {
        self.set_ident();

        loop {
            if let Some(ch) = self.src.next() {
                if self.skip_space() && ch.is_srl_whitespace() {
                    continue;
                }

                if ch.is_ascii() && ch.is_alphabetic() {
                    // identifiers are case insensitive
                    self.buffer.push(ch.to_ascii_lowercase());
                    self.set_last_char(ch);
                } else if ch.is_group_start() && self.buffer.is_empty() {
                    self.set_last_char(' ');
                    return get_token("(");
                } else if ch.is_srl_whitespace() || ch.is_group_char() {
                    if let Some(token) = get_token(&self.buffer) {
                        // valid token !!
                        self.reset_buffer();
                        self.next_state(&token);
                        if ch.is_group_char() {
                            // part of next token
                            self.buffer.push(ch);
                        }
                        self.set_last_char('\0');
                        return Some(token);
                    } else if self.buffer.len() < MAX_SPC_INDEX  {
                        // add space to token
                        self.buffer.push(' ');
                        self.set_last_char(' ');
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
                if self.buffer.is_empty() {
                    self.set_eof();
                    return Some(get_eof_token());
                } else if let Some(token) = get_token(&self.buffer) {
                    // valid token !!
                    self.reset_buffer();
                    return Some(token);
                } else {
                    // unexpected eof
                    self.set_error();
                    break;
                }
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

            } else {
                self.set_error();
                return None;
            }
        }

        loop {
            if let Some(ch) = self.src.next() {
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
                        self.reset_buffer();
                        self.next_state(&token);
                        return Some(token);
                    }
                } else {
                    self.buffer.push(ch);
                    self.set_last_char(ch);
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
                if self.skip_space() && ch.is_srl_whitespace() {
                    continue;
                } else if ch.is_digit(10) {
                    self.buffer.push(ch);
                    self.set_last_char(ch);
                } else if ch.is_srl_whitespace() {
                    // number ends
                    let token = get_number_token(self.buffer.as_ref());
                    self.next_state(&token);
                    self.reset_buffer();
                    self.set_last_char(' ');
                    return Some(token);
                } else {
                    // invalid char in number
                    self.set_error();
                    return None;
                }
            } else {
                if self.buffer.is_empty() {
                    self.set_error();
                    return None;
                } else {
                    // number ends
                    let token = get_number_token(self.buffer.as_ref());
                    self.next_state(&token);
                    self.reset_buffer();
                    self.set_last_char(' ');
                    return Some(token);
                }
            }
        }
    }


    /// Returns next char or digit
    fn next_char_or_digit(&mut self) -> Option<Token> {
        self.set_char_or_digit();
        loop {
            if let Some(ch) = self.src.next() {
                if ch.is_srl_whitespace() {
                    continue;
                } else if ch.is_ascii() && ch.is_digit(10) {
                    self.buffer.push(ch);
                    let token = get_digit_token(self.buffer.as_ref());
                    self.reset_buffer();
                    self.set_ident();
                    return Some(token);
                } else if ch.is_ascii() && ch.is_alphabetic() {
                    self.buffer.push(ch);
                    let token = get_char_token(self.buffer.as_ref());
                    self.reset_buffer();
                    self.set_ident();
                    return Some(token);
                } else {
                    // unexpected char
                    self.set_error();
                    return None;
                }
            } else {
                self.set_error();
                return None;
            }
        }
    }

    fn next_token(&mut self) -> Option<Token> {
        loop {
            let ch = *self.src.peek().unwrap_or(&'\0');
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

            if ch == '\0' {
                return Some(get_eof_token());
            }

            self.set_error();
            return None;
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    /// Returns next token
    ///
    /// # Examples
    /// ```
    /// use srl::lexer::Lexer;
    ///
    /// let mut lx = Lexer::new("bEgin with letter from a to k twice");
    /// assert!(lx.next().is_some());
    /// assert!(lx.next().is_some());
    /// assert!(lx.next().is_some());
    /// ```
    fn next(&mut self) -> Option<Token> {
        match self.state {
            State::None => self.next_token(),
            State::Identifier => self.next_identifier(),
            State::String => self.next_string(),
            State::Number => self.next_number(),
            State::CharOrDigit => self.next_char_or_digit(),
            _ => None,
        }
    }
}


#[cfg(test)]
mod test;
