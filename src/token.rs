#[derive(Clone, Copy)]
pub enum TokenType {
    Character,
    Quantifier,
    Group,
    Lookaround,
    Flag,
    Anchor,
    SrcWhitespace,
    SrcNumber,
    SrcString,
    Delimiter,
    EndOfFile,
    Undefined,
}

#[derive(Clone, Copy)]
pub enum TokenValue {
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

    ExcatlyXTimes,
    ExactlyOneTime,
    Once,
    Twice,
    BetweenXAndYTimes,
    Optional,
    OnceOrMore,
    NeverOrMore,
    AtLeastXTimes,
    Time,
    Times,
    And,

    CaptureAs,
    AnyOf,
    Until,
    As,

    IfFollowedBy,
    IfNotFollowedBy,
    IfAlreadyHad,
    IfNotAlreadyHad,

    CaseInsensitive,
    MultiLine,
    AllLazy,

    BeginWith,
    StartsWith,
    MustEnd,

    Space,
    Number,
    String,
    GroupStart,
    GroupEnd,

    EndOfFile,
    Undefined,
}

#[derive(Clone)]
pub struct Token<'a> {
    val: &'a str,
    token_type: TokenType,
    token_value: TokenValue,
}

impl<'a> Token<'a> {
    pub fn new(t_val: &str, t_type: TokenType, t_value: TokenValue) -> Token {
        Token {
            val: t_val,
            token_type: t_type,
            token_value: t_value,
        }
    }

    pub fn val(&self) -> String {
        self.val.to_string()
    }

    pub fn token_type(&self) -> TokenType {
        self.token_type
    }

    pub fn token_value(&self) -> TokenValue {
        self.token_value
    }
}
