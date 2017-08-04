use std::collections::HashMap;
use token::{Token, TokenType};

lazy_static! {
    static ref GRAMMAR_TABLE: HashMap<&'static str, TokenType> = {
        let mut table = HashMap::new();
        table.insert("literally", TokenType::Literally);
        table.insert("one of", TokenType::OneOf);
        table.insert("letter", TokenType::Letter);
        table.insert("uppercase letter", TokenType::UppercaseLetter);
        table.insert("any character", TokenType::AnyCharacter);
        table.insert("no character", TokenType::NoCharacter);
        table.insert("digit", TokenType::Digit);
        table.insert("anything", TokenType::Anything);
        table.insert("new line", TokenType::NewLine);
        table.insert("whitespace", TokenType::Whitespace);
        table.insert("no whitespace", TokenType::NoWhitespace);
        table.insert("tab", TokenType::Tab);
        table.insert("raw", TokenType::Raw);
        table.insert("from", TokenType::From);
        table.insert("to", TokenType::To);

        table.insert("exactly", TokenType::ExcatlyXTimes);
        table.insert("once", TokenType::Once);
        table.insert("twice", TokenType::Twice);
        table.insert("between", TokenType::BetweenXAndYTimes);
        table.insert("optional", TokenType::Optional);
        table.insert("once or more", TokenType::OnceOrMore);
        table.insert("never or more", TokenType::NeverOrMore);
        table.insert("at least", TokenType::AtLeastXTimes);
        table.insert("and", TokenType::And);
        table.insert("capture", TokenType::CaptureAs);
        table.insert("any of", TokenType::AnyOf);
        table.insert("until", TokenType::Until);

        table.insert("if followed by", TokenType::IfFollowedBy);
        table.insert("if not followed by", TokenType::IfNotFollowedBy);
        table.insert("if already had", TokenType::IfAlreadyHad);
        table.insert("if not already had", TokenType::IfNotAlreadyHad);

        table.insert("case insensitive", TokenType::CaseInsensitive);
        table.insert("multi line", TokenType::MultiLine);
        table.insert("all lazy", TokenType::AllLazy);

        table.insert("begin with", TokenType::BeginWith);
        table.insert("starts with", TokenType::BeginWith);
        table.insert("must end", TokenType::MustEnd);

        table.insert("\n", TokenType::Space);
        table.insert("(", TokenType::GroupStart);
        table.insert(")", TokenType::GroupEnd);
        table.insert("eof", TokenType::EndOfFile);
        table
    };
}

/// greatest index space can occurr in a token
pub const MAX_SPC_INDEX: usize = 15;

pub fn get_token<'a>(token: &'a str) -> Option<Token> {
    GRAMMAR_TABLE.get(token).and_then(
        |tk| Some(Token::new(token, *tk)),
    )
}


pub fn get_string_token(val: &str) -> Token {
   Token::new(val, TokenType::String)
}
