//! parse tokens to AST

use ast::{self, Expr};
use lexer::Lexer;
use token::*;

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    peeked: Option<Option<Token>>,
}

impl<'a> Iterator for Parser<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        match self.peeked.take() {
            Some(v) => v,
            None => self.lexer.next(),
        }
    }

}

impl <'a> Parser<'a> {
    pub fn peek(&mut self) -> Option<&Token> {
        if self.peeked.is_none() {
            self.peeked = Some(self.lexer.next());
        }
        match self.peeked {
            Some(Some(ref value)) => Some(value),
            Some(None) => None,
            _ => unreachable!(),
        }
    }
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer) -> Parser {
        Parser {
            lexer: lexer,
            peeked: None,
        }
    }

    pub fn parse(mut self) -> Result<Expr, String> {
        while let Some(tk) = self.next() {
            let expr = match tk.token_type() {
                TokenType::Character(_) => self.parse_character(tk),
                _ => Err(format!("error occured")),
            };

            if let Ok(v) = expr {
            }
        }

        if self.lexer.is_error() {
            Err(format!("error occured"))
        } else {
            unimplemented!("")
        }
    }

    fn parse_character(&mut self, tk: Token) -> Result<Expr, String> {
        unimplemented!("")
    }
}
