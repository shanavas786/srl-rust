//! data types for SRL AST

use token::{Token, Anchors, Flags};

pub enum Specification {
    Char { from: char, to: char },
    Digit { from: i32, to: i32 },
    String(String),
}

pub enum Quantifier {
    NeverOrMore,
    Once,
    OnceOrMore,
    Twice,
    Optional,
    Exactly(i32),
    Between { from: i32, to: i32 },
    Alteast(i32),
}

pub struct Character {
    pub ty: Token,
    pub spec: Option<Specification>,
}

pub enum Group {
    AnyOf(Vec<Expr>),
    Capture {
        cond: Vec<Expr>,
        name: Option<String>,
    },
    Until(Vec<Expr>),
    IfFollowedBy(Vec<Expr>),
    IfNotFollowedBy(Vec<Expr>),
    IfAlreadyHad(Vec<Expr>),
    IfNotAlreadyHad(Vec<Expr>),
}

pub enum Expr {
    Anchor(Anchors),
    Character(Character),
    Group(Group),
    Quantifier,
    Flag(Flags),
}

impl From<Character> for Expr {
    fn from(ch: Character) -> Expr {
        Expr::Character(ch)
    }
}

impl From<Token> for Expr {
    fn from(tk: Token) -> Expr {
        Expr::Character(Character { ty: tk, spec: None })
    }
}
