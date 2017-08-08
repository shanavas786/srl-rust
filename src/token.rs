#[derive(Clone, Copy, Debug)]
pub enum TokenType {
    Literally,
    OneOf,
    Letter,
    UppercaseLetter,
    AnyCharacter,
    NoCharacter,
    Digit,
    Anything,
    NewLine,
    Whitespace,
    NoWhitespace,
    Tab,
    Raw,
    From,
    To,

    Exactly,
    Once,
    Twice,
    Between,
    Optional,
    OnceOrMore,
    NeverOrMore,
    AtLeast,
    Time,
    Times,
    And,

    Capture,
    As,
    AnyOf,
    Until,

    IfFollowedBy,
    IfNotFollowedBy,
    IfAlreadyHad,
    IfNotAlreadyHad,

    CaseInsensitive,
    MultiLine,
    AllLazy,

    BeginWith,
    MustEnd,

    Space,
    Number,
    String,
    GroupStart,
    GroupEnd,

    EndOfFile,
    Undefined,
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
}
