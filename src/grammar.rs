use std::collections::HashMap;
use token::*;

lazy_static! {
    static ref GRAMMAR_TABLE: HashMap<&'static str, TokenType> = {
        let mut table = HashMap::new();
        table.insert("literally", TokenType::Character(Characters::Literally));
        table.insert("one of", TokenType::Character(Characters::OneOf));
        table.insert("letter", TokenType::Character(Characters::Letter));
        table.insert("uppercase letter", TokenType::Character(Characters::UppercaseLetter));
        table.insert("any character", TokenType::Character(Characters::AnyCharacter));
        table.insert("no character", TokenType::Character(Characters::NoCharacter));
        table.insert("digit", TokenType::Character(Characters::Digit));
        table.insert("anything", TokenType::Character(Characters::Anything));
        table.insert("new line", TokenType::Character(Characters::NewLine));
        table.insert("whitespace", TokenType::Character(Characters::Whitespace));
        table.insert("no whitespace", TokenType::Character(Characters::NoWhitespace));
        table.insert("tab", TokenType::Character(Characters::Tab));
        table.insert("raw", TokenType::Character(Characters::Raw));

        table.insert("from", TokenType::Specification(Specifications::From));
        table.insert("to", TokenType::Specification(Specifications::To));

        table.insert("exactly", TokenType::Quantifier(Quantifiers::Exactly));
        table.insert("once", TokenType::Quantifier(Quantifiers::Once));
        table.insert("twice", TokenType::Quantifier(Quantifiers::Twice));
        table.insert("between", TokenType::Quantifier(Quantifiers::Between));
        table.insert("and", TokenType::Quantifier(Quantifiers::And));
        table.insert("optional", TokenType::Quantifier(Quantifiers::Optional));
        table.insert("once or more", TokenType::Quantifier(Quantifiers::OnceOrMore));
        table.insert("never or more", TokenType::Quantifier(Quantifiers::NeverOrMore));
        table.insert("at least", TokenType::Quantifier(Quantifiers::AtLeast));

        table.insert("capture", TokenType::Group(Groups::Capture));
        table.insert("as", TokenType::Group(Groups::As));
        table.insert("any of", TokenType::Group(Groups::AnyOf));
        table.insert("either of", TokenType::Group(Groups::AnyOf));
        table.insert("until", TokenType::Group(Groups::Until));
        table.insert("if followed by", TokenType::Group(Groups::IfFollowedBy));
        table.insert("if not followed by", TokenType::Group(Groups::IfNotFollowedBy));
        table.insert("if already had", TokenType::Group(Groups::IfAlreadyHad));
        table.insert("if not already had", TokenType::Group(Groups::IfNotAlreadyHad));
        table.insert("(", TokenType::Group(Groups::GroupStart));
        table.insert(")", TokenType::Group(Groups::GroupEnd));

        table.insert("case insensitive", TokenType::Flag(Flags::CaseInsensitive));
        table.insert("multi line", TokenType::Flag(Flags::MultiLine));
        table.insert("all lazy", TokenType::Flag(Flags::AllLazy));

        table.insert("begin with", TokenType::Anchor(Anchors::BeginWith));
        table.insert("starts with", TokenType::Anchor(Anchors::BeginWith));
        table.insert("must end", TokenType::Anchor(Anchors::MustEnd));

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

// create a String token with given value
pub fn get_string_token(val: &str) -> Token {
    Token::new(val, TokenType::String)
}

// create a Number token with given value
pub fn get_number_token(val: &str) -> Token {
    Token::new(val, TokenType::Number)
}

// create a character token with given value
pub fn get_char_token(val: &str) -> Token {
    Token::new(val, TokenType::Char)
}

// create a digit token with given value
pub fn get_digit_token(val: &str) -> Token {
    Token::new(val, TokenType::Digit)
}

// create a EndOfFile token
pub fn get_eof_token() -> Token {
    Token::new("eof", TokenType::EndOfFile)
}
