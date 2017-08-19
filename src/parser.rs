//! parse tokens to AST

use ast::{self, Expr};
use lexer::Lexer;
use token::*;

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    ast: Vec<Expr>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer) -> Parser {
        Parser {
            lexer: lexer,
            ast: vec![],
        }
    }

    pub fn parse(mut self) -> Result<Vec<Expr>, String> {
        while let Some(tk) = self.lexer.next() {
            let expr = match tk.token_type() {
                TokenType::Character(_) => self.parse_character(tk),
                _ => Err(format!("error occured")),
            };

            if let Ok(v) = expr {
                self.ast.push(v);
            }
        }

        if self.lexer.is_error() {
            Err(format!("error occured"))
        } else {
            Ok(self.ast)
        }
    }

    fn parse_character(&mut self, tk: Token) -> Result<Expr, String> {
        match tk.token_type() {
            TokenType::Character(Characters::Literally) |
            TokenType::Character(Characters::OneOf) |
            TokenType::Character(Characters::Raw) => {
                if let Some(val) = self.lexer.next() {
                    match val.token_type() {
                        TokenType::String => Ok(Expr::from(ast::Character {
                            ty: tk,
                            spec: Some(ast::Specification::String(val.val())),
                        })),
                        _ => Err(format!("expected string, found {:?}", val)),
                    }
                } else {
                    Err(format!("expected string"))
                }
            },
            TokenType::Character(_) => {
                Ok(Expr::from(tk))
            },
            _ => Err(format!("expected character, found {:?}", tk)),
        }
    }
}
