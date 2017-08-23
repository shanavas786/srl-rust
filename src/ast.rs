//! data types for SRL AST

use token::{Anchors, Flags};

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

pub enum CharKind {
    Literally(String),
    OneOf(String),
    Raw(String),
    Letter {from: char, to: char},
    UppercaseLetter {from: char, to: char},
    Digit {from: i32, to: i32},
    AnyCharacter,
    NoCharacter,
    Anything,
    NewLine,
    Whitespace,
    NoWhitespace,
    Tab,
}

pub enum Character {
    CharKind(CharKind),
    Group(Box<Group>),
    LookAroundChar(LookAround, Box<Character>),
    CharLookAround(Box<Character>, LookAround),
    CharAnchor(Box<Character>, Anchors),
    AnchorChar(Anchors, Box<Character>),
    CharFlag(Box<Character>, Flags),
}

pub enum LookAround {
    Until(Group),
    IfFollowedBy(Group),
    IfNotFollowedBy(Group),
    IfAlreadyHad(Group),
    IfNotAlreadyHad(Group),
}

pub enum Group {
    String(String),
    AnyOf(Box<Expr>),
    Capture {
        cond: Box<Expr>,
        name: Option<String>,
    },
}

pub enum Expr {
    Character(Character, Option<Quantifier>),
    CharExpr(Character, Box<Expr>),
    Group(Box<Group>),
}
