//! parse tokens to AST

use ast::{self, Expr};
use lexer::Lexer;
use token::*;

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    curr_tk: Option<Token>,
}

impl<'a> Iterator for Parser<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        let tk = self.lexer.next();
        self.curr_tk = tk.clone();
        tk
    }
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer) -> Parser {
        let mut p = Parser {
            lexer: lexer,
            curr_tk: None,
        };
        p.next();
        p
    }

    pub fn parse_expr(&mut self) -> Result<Expr, String> {
        if self.curr_tk.is_none() {
            return Err(format!("keyword expected"))
        }

        let tk = self.curr_tk.clone().unwrap();
        let expr = match tk.token_type () {
            TokenType::Character(ty) => {
                self.parse_character(ty)
            },
            _ => unimplemented!("2"),
        };

        if expr.is_ok() {
            return expr;
        }

        if self.lexer.is_error() {
            Err(format!("error occured"))
        } else {
            unreachable!("")
        }
    }

    pub fn parse_character(&mut self, ty: Characters) -> Result<Expr, String> {
        let ch = match ty {
            Characters::Literally | Characters::OneOf | Characters::Raw => {
                if let Some(spec) = self.next() {
                    if spec.is_string() {
                        ty.to_charkind(Some(spec.val()))
                    } else {
                        return Err(format!("expected string"));
                    }
                } else {
                    return Err(format!("expected string"));
                }
            },
            Characters::Letter |
            Characters::UppercaseLetter |
            Characters::Digit => {
                self.parse_spec(ty)
            },
            _ => ty.to_charkind(None),
        };

        unimplemented!("")
    }

    fn parse_spec(&mut self, ty: Characters) -> ast::CharKind {
        if let Some(from) = self.next() {
            if from.token_type().is_spec_start() {
                unimplemented!("")
            } else {
                ty.to_char_class_with_spec(None, None)
            }
        } else {
            ty.to_char_class_with_spec(None, None)
        }
    }
}
