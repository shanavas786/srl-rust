//! builder

use ast::Expr;

pub struct Builder {
    regex: String,
}


impl Builder {
    pub fn from_ast<'a>(expr: Vec<Expr>) -> &'a str {
        "not implemented"
    }
}
