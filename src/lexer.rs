use std::str::SplitWhitespace;
use token::Token;
use grammar::get_token;

enum State {
    Identifier,
    Number,
    String,

    None,
    EndOfFile,
    Error
}

pub struct Lexer<'a> {
    src: &'a str,
    tokens: SplitWhitespace<'a>,
    state: State,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &str) -> Lexer {
        Lexer {
            src: src,
            tokens: src.split_whitespace(),
            state: State::None
        }
    }

    pub fn next(&mut self) -> Option<Token> {
        self.tokens.next().map_or(None, |s| get_token(s))
    }
}
