use std::str::SplitWhitespace;
use token::Token;
use grammar::Grammar;

enum State {
    Identifier,
    Number,
    String,

    None,
    EndOfFile,
    Error
}

pub struct Lexer {
    src: String,
    tokens: SplitWhitespace,
    state: State,
    curr_token: Token,
}

impl Lexer {
    pub fn new(src: &str) -> Lexer {
        Lexer {
            grammar: Grammar::new(),
            src: src,
            tokens: src.split_whitespace(),
            state: State::None,
            curr_token: Token::start(),
        }
    }

    pub fn token(&self) -> Token {
        self.curr_token
    }

    pub fn next(&self) -> Option<&Token> {
        self.tokens.next().map_or(None, |s| grammar.get(s))
    }
}
