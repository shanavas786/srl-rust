use ast::CharKind;

#[derive(Clone, Copy, Debug)]
pub enum Characters {
    Literally,
    OneOf,
    Letter,
    UppercaseLetter,
    Digit,
    AnyCharacter,
    NoCharacter,
    Anything,
    NewLine,
    Whitespace,
    NoWhitespace,
    Tab,
    Raw,
}

impl Characters {
    pub fn to_charkind(self, val: Option<String>) -> CharKind {
        match self {
            Characters::Literally => CharKind::Literally(val.unwrap()),
            Characters::OneOf => CharKind::OneOf(val.unwrap()),
            Characters::Raw => CharKind::Raw(val.unwrap()),
            Characters::AnyCharacter => CharKind::AnyCharacter,
            Characters::NoCharacter => CharKind::NoCharacter,
            Characters::Anything => CharKind::Anything,
            _ => unimplemented!(""),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Specifications {
    From,
    To,
}

#[derive(Clone, Copy, Debug)]
pub enum Quantifiers {
    Exactly,
    Once,
    Twice,
    Between,
    Optional,
    OnceOrMore,
    NeverOrMore,
    AtLeast,
    Times,
    And,
}

#[derive(Clone, Copy, Debug)]
pub enum Groups {
    Capture,
    As,
    Until,
    AnyOf,

    IfFollowedBy,
    IfNotFollowedBy,
    IfAlreadyHad,
    IfNotAlreadyHad,

    GroupStart,
    GroupEnd,
}

#[derive(Clone, Copy, Debug)]
pub enum Flags {
    CaseInsensitive,
    MultiLine,
    AllLazy,
}

#[derive(Clone, Copy, Debug)]
pub enum Anchors {
    BeginWith,
    MustEnd,
}

#[derive(Clone, Copy, Debug)]
pub enum TokenType {
    Character(Characters),
    Specification(Specifications),
    Quantifier(Quantifiers),
    Group(Groups),
    Flag(Flags),
    Anchor(Anchors),

    Number,
    String,
    Char,
    Digit,

    EndOfFile,
}

#[derive(Clone, Debug)]
pub struct Token {
    val: String,
    token_type: TokenType,
}

impl Token {
    pub fn new(t_val: &str, t_type: TokenType) -> Token {
        Token {
            val: String::from(t_val),
            token_type: t_type,
        }
    }

    pub fn val(&self) -> String {
        self.val.to_string()
    }

    pub fn token_type(&self) -> TokenType {
        self.token_type
    }

    pub fn is_string(&self) -> bool {
        match self.token_type {
            TokenType::String => true,
            _ => false,
        }
    }
}
