use token::{Token, TokenType, TokenValue};

#[allow(dead_code)]
pub struct Grammar {
    table: [Token; 48],
    prefices: [String; 4],
    max_key_len: i32,
}

#[allow(dead_code)]
impl Grammar {
    pub fn new() -> Grammar {
        Grammar {
            table: [
                Token::new("literally", TokenType::Character, TokenValue::Literally),
                Token::new("one of", TokenType::Character, TokenValue::OneOf),
                Token::new("letter", TokenType::Character, TokenValue::Letter),
                Token::new(
                    "uppercase letter",
                    TokenType::Character,
                    TokenValue::UppercaseLetter,
                ),
                Token::new(
                    "any character",
                    TokenType::Character,
                    TokenValue::AnyCharacter,
                ),
                Token::new(
                    "no character",
                    TokenType::Character,
                    TokenValue::NoCharacter,
                ),
                Token::new("digit", TokenType::Character, TokenValue::Digit),
                Token::new("anything", TokenType::Character, TokenValue::Anything),
                Token::new("new line", TokenType::Character, TokenValue::NewLine),
                Token::new("whitespace", TokenType::Character, TokenValue::Whitespace),
                Token::new(
                    "no whitespace",
                    TokenType::Character,
                    TokenValue::NoWhitespace,
                ),
                Token::new("tab", TokenType::Character, TokenValue::Tab),
                Token::new("raw", TokenType::Character, TokenValue::Raw),
                Token::new("from", TokenType::Character, TokenValue::From),
                Token::new("to", TokenType::Character, TokenValue::To),

                Token::new("exactly", TokenType::Quantifier, TokenValue::ExcatlyXTimes),
                Token::new(
                    "exactly 1 time",
                    TokenType::Quantifier,
                    TokenValue::ExactlyOneTime,
                ),
                Token::new("once", TokenType::Quantifier, TokenValue::Once),
                Token::new("twice", TokenType::Quantifier, TokenValue::Twice),
                Token::new(
                    "between",
                    TokenType::Quantifier,
                    TokenValue::BetweenXAndYTimes,
                ),
                Token::new("optional", TokenType::Quantifier, TokenValue::Optional),
                Token::new(
                    "once or more",
                    TokenType::Quantifier,
                    TokenValue::OnceOrMore,
                ),
                Token::new(
                    "never or more",
                    TokenType::Quantifier,
                    TokenValue::NeverOrMore,
                ),
                Token::new("at least", TokenType::Quantifier, TokenValue::AtLeastXTimes),
                Token::new("time", TokenType::Quantifier, TokenValue::Time),
                Token::new("times", TokenType::Quantifier, TokenValue::Times),
                Token::new("and", TokenType::Quantifier, TokenValue::And),
                Token::new("capture", TokenType::Group, TokenValue::CaptureAs),
                Token::new("any of", TokenType::Group, TokenValue::AnyOf),
                Token::new("until", TokenType::Group, TokenValue::Until),
                Token::new("as", TokenType::Group, TokenValue::As),

                Token::new(
                    "if followed by",
                    TokenType::Lookaround,
                    TokenValue::IfFollowedBy,
                ),
                Token::new(
                    "if not followed by",
                    TokenType::Lookaround,
                    TokenValue::IfNotFollowedBy,
                ),
                Token::new(
                    "if already had",
                    TokenType::Lookaround,
                    TokenValue::IfAlreadyHad,
                ),
                Token::new(
                    "if not already had",
                    TokenType::Lookaround,
                    TokenValue::IfNotAlreadyHad,
                ),

                Token::new(
                    "case insensitive",
                    TokenType::Flag,
                    TokenValue::CaseInsensitive,
                ),
                Token::new("multi line", TokenType::Flag, TokenValue::MultiLine),
                Token::new("all lazy", TokenType::Flag, TokenValue::AllLazy),

                Token::new("begin with", TokenType::Anchor, TokenValue::BeginWith),
                Token::new("starts with", TokenType::Anchor, TokenValue::StartsWith),
                Token::new("must end", TokenType::Anchor, TokenValue::MustEnd),

                Token::new(",", TokenType::SrcWhitespace, TokenValue::Space),
                Token::new(" ", TokenType::SrcWhitespace, TokenValue::Space),
                Token::new("\n", TokenType::SrcWhitespace, TokenValue::Space),

                Token::new("\"", TokenType::Delimiter, TokenValue::String),
                Token::new("\'", TokenType::Delimiter, TokenValue::String),
                Token::new("(", TokenType::Delimiter, TokenValue::GroupStart),
                Token::new(")", TokenType::Delimiter, TokenValue::GroupEnd),
            ],
            prefices: [
                String::from("exactly"),
                String::from("once"),
                String::from("time"),
                String::from("times"),
            ],
            max_key_len: 18,
        }
    }

    pub fn max_key_len(&self) -> i32 {
        self.max_key_len
    }

    pub fn is_prefix(&self, token: &str) -> bool {
        self.prefices.iter().any(|t| *t == String::from(token))
    }

    pub fn get(&self, token: &str) -> Option<&Token> {
        self.table.iter().find(|t| t.val() == String::from(token))
    }
}
