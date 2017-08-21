//! builder

use ast::Expr;

pub struct Builder {
    regex: String,
}


impl Builder {
    pub fn from_ast<'a>(expr: Expr) -> &'a str {
        unimplemented!("builder")
    }
}
