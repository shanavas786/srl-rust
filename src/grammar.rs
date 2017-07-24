use std::collections::HashMap;
use token::{Token, TokenType, TokenValue};

lazy_static! {
    static ref GRAMMAR_TABLE: HashMap<&'static str, (TokenType, TokenValue)> = {
        let mut table = HashMap::new();
        table.insert("literally", (TokenType::Character, TokenValue::Literally));
        table.insert("one of", (TokenType::Character, TokenValue::OneOf));
        table.insert("letter", (TokenType::Character, TokenValue::Letter));
        table.insert("uppercase letter", (TokenType::Character, TokenValue::UppercaseLetter));
        table.insert("any character", (TokenType::Character, TokenValue::AnyCharacter));
        table.insert("no character", (TokenType::Character, TokenValue::NoCharacter));
        table.insert("digit", (TokenType::Character, TokenValue::Digit));
        table.insert("anything", (TokenType::Character, TokenValue::Anything));
        table.insert("new line", (TokenType::Character, TokenValue::NewLine));
        table.insert("whitespace", (TokenType::Character, TokenValue::Whitespace));
        table.insert("no whitespace", (TokenType::Character,TokenValue::NoWhitespace));
        table.insert("tab", (TokenType::Character, TokenValue::Tab));
        table.insert("raw", (TokenType::Character, TokenValue::Raw));
        table.insert("from", (TokenType::Character, TokenValue::From));
        table.insert("to", (TokenType::Character, TokenValue::To));

        table.insert("exactly", (TokenType::Quantifier, TokenValue::ExcatlyXTimes));
        table.insert("exactly 1 time", (TokenType::Quantifier, TokenValue::ExactlyOneTime));
        table.insert("once", (TokenType::Quantifier, TokenValue::Once));
        table.insert("twice", (TokenType::Quantifier, TokenValue::Twice));
        table.insert("between", (TokenType::Quantifier, TokenValue::BetweenXAndYTimes));
        table.insert("optional", (TokenType::Quantifier, TokenValue::Optional));
        table.insert("once or more", (TokenType::Quantifier, TokenValue::OnceOrMore));
        table.insert("never or more", (TokenType::Quantifier, TokenValue::NeverOrMore));
        table.insert("at least", (TokenType::Quantifier, TokenValue::AtLeastXTimes));
        table.insert("time", (TokenType::Quantifier, TokenValue::Time));
        table.insert("times", (TokenType::Quantifier, TokenValue::Times));
        table.insert("and", (TokenType::Quantifier, TokenValue::And));
        table.insert("capture", (TokenType::Group, TokenValue::CaptureAs));
        table.insert("any of", (TokenType::Group, TokenValue::AnyOf));
        table.insert("until", (TokenType::Group, TokenValue::Until));
        table.insert("as", (TokenType::Group, TokenValue::As));

        table.insert("if followed by", (TokenType::Lookaround, TokenValue::IfFollowedBy));
        table.insert("if not followed by", (TokenType::Lookaround, TokenValue::IfNotFollowedBy));
        table.insert("if already had", (TokenType::Lookaround, TokenValue::IfAlreadyHad));
        table.insert("if not already had", (TokenType::Lookaround, TokenValue::IfNotAlreadyHad));

        table.insert("case insensitive", (TokenType::Flag, TokenValue::CaseInsensitive));
        table.insert("multi line", (TokenType::Flag, TokenValue::MultiLine));
        table.insert("all lazy", (TokenType::Flag, TokenValue::AllLazy));

        table.insert("begin with", (TokenType::Anchor, TokenValue::BeginWith));
        table.insert("starts with", (TokenType::Anchor, TokenValue::StartsWith));
        table.insert("must end", (TokenType::Anchor, TokenValue::MustEnd));

        table.insert(",", (TokenType::SrcWhitespace, TokenValue::Space));
        table.insert(" ", (TokenType::SrcWhitespace, TokenValue::Space));
        table.insert("\n", (TokenType::SrcWhitespace, TokenValue::Space));

        table.insert("\"", (TokenType::Delimiter, TokenValue::String));
        table.insert("\'", (TokenType::Delimiter, TokenValue::String));
        table.insert("(", (TokenType::Delimiter, TokenValue::GroupStart));
        table.insert(")", (TokenType::Delimiter, TokenValue::GroupEnd));
        table
    };
}


pub fn get<'a>(token: &'a str) -> Option<Token> {
    GRAMMAR_TABLE.get(token).and_then(|tk| {
        Some(Token::new(token, tk.0, tk.1))
    })
}
