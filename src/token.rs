use ast::CharKind;

#[derive(Clone, Copy, Debug)]
pub enum Characters {
    Literally,
    OneOf,
    Raw,
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
            Characters::NewLine => CharKind::NewLine,
            Characters::Whitespace => CharKind::Whitespace,
            Characters::NoWhitespace => CharKind::NoWhitespace,
            Characters::Tab => CharKind::Tab,
            Characters::Letter |
            Characters::UppercaseLetter |
            Characters::Digit => unimplemented!(""),
        }
    }

    pub fn to_char_class_with_spec(self, from: Option<char>, to: Option<char>) -> CharKind {
        match self {
            Characters::Letter => CharKind::Letter {
                from: from.unwrap_or('a'),
                to: to.unwrap_or('z'),
            },
            Characters::UppercaseLetter => CharKind::UppercaseLetter {
                from: from.unwrap_or('A'),
                to: to.unwrap_or('Z'),
            },
            Characters::Digit => CharKind::Digit {
                from: from.unwrap_or('0') as i32,
                to: to.unwrap_or('9') as i32,
            },
            _ => unreachable!(""),
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

impl TokenType {
    pub fn is_spec_start(self) -> bool {
        match self {
            TokenType::Specification(Specifications::From) => true,
            _ => false,
        }
    }
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
