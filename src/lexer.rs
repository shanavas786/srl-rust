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

pub struct Lexer<'a> {
    grammar: Grammar,
    tokens: SplitWhitespace<'a>,
    state: State,
    curr_token: &'a str,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &str) -> Lexer {
        Lexer {
            grammar: Grammar::new(),
            tokens: src.split_whitespace(),
            state: State::None,
            curr_token: "",
        }
    }

    pub fn next(&mut self) -> Option<&Token> {
        if let Some(mut s) = self.tokens.next() {
            if let Some(token) = self.grammar.get(s) {
                if token.is_partial() {
                    return self.next();
                } else {
                    Some(&token)
                }
            } else {
                panic!("invalid input");
            }
        } else {
            self.state = State::EndOfFile;
            None
        }
    }
}
