#[macro_use]
extern crate lazy_static;

mod token;
mod grammar;
mod lexer;
mod ast;
mod parser;
mod builder;


pub fn srl(src: &str) -> &str {
    let lx = lexer::Lexer::new(src);
    let pr = parser::Parser::new(lx);
    let res = pr.parse();
    if let Ok(ast) = res {
        return builder::Builder::from_ast(ast)
    } else {
        panic!(res.err());
    }
}
