use pest::prelude::*;
use super::*;

fn test_parse<F>(ip: &'static str, func: F, expected: Vec<Token<Rule>>)
    where F: FnOnce(&mut Rdp<StringInput>) -> bool {
    let mut parser = Rdp::new(StringInput::new(ip));
    assert!(func(&mut parser));
    assert!(parser.end());
    assert_eq!(parser.queue(), &expected);
}

#[test]
fn test_number() {
    test_parse("123456", |p| p.number(), vec![Token::new(Rule::number, 0, 6)]);
}

#[test]
fn test_alpha() {
    test_parse("a", |p| p.lower_alpha(), vec![Token::new(Rule::lower_alpha, 0, 1)]);
    test_parse("U", |p| p.upper_alpha(), vec![Token::new(Rule::upper_alpha, 0, 1)]);
}

#[test]
fn test_quantifier() {
    test_parse("exactly 78 Times",
               |p| p.exactly(),
               vec![Token::new(Rule::exactly, 0, 16),
                    Token::new(Rule::number, 8, 10)]);

    test_parse("once",
               |p| p.once(),
               vec![Token::new(Rule::once, 0, 4)]);

    test_parse("twice",
               |p| p.twice(),
               vec![Token::new(Rule::twice, 0, 5)]);

    test_parse("between 78 and 100 Times",
               |p| p.between_x_y(),
               vec![Token::new(Rule::between_x_y, 0, 24),
                    Token::new(Rule::number, 8, 10),
                    Token::new(Rule::number, 15, 18)]);

    test_parse("between 78 and 100",
               |p| p.between_x_y(),
               vec![Token::new(Rule::between_x_y, 0, 18),
                    Token::new(Rule::number, 8, 10),
                    Token::new(Rule::number, 15, 18)]);

    test_parse("optional",
               |p| p.optional(),
               vec![Token::new(Rule::optional, 0, 8)]);

    test_parse("once or more",
               |p| p.once_or_more(),
               vec![Token::new(Rule::once_or_more, 0, 12)]);

    test_parse("never or more",
               |p| p.never_or_more(),
               vec![Token::new(Rule::never_or_more, 0, 13)]);

    test_parse("atleast 8 times",
               |p| p.atleast_x(),
               vec![Token::new(Rule::atleast_x, 0, 15),
                    Token::new(Rule::number, 8, 9)]);

}

#[test]
fn test_anchor() {
    test_parse("begin with",
               |p| p.begin_with(),
               vec![Token::new(Rule::begin_with, 0, 10)]);

    test_parse("start with",
               |p| p.begin_with(),
               vec![Token::new(Rule::begin_with, 0, 10)]);

    test_parse("must end",
               |p| p.must_end(),
               vec![Token::new(Rule::must_end, 0, 8)]);
}

#[test]
fn test_flags() {
    test_parse("case insensitive",
               |p| p.case_insensitive(),
               vec![Token::new(Rule::case_insensitive, 0, 16)]);

    test_parse("multiline",
               |p| p.multiline(),
               vec![Token::new(Rule::multiline, 0, 9)]);

    test_parse("all lazy",
               |p| p.all_lazy(),
               vec![Token::new(Rule::all_lazy, 0, 8)]);
}

#[test]
fn test_character() {
    test_parse("literally \"str\"",
               |p| p.literally(),
               vec![Token::new(Rule::literally, 0, 15),
                    Token::new(Rule::string_literal, 10, 15)]);

    test_parse("one of \"abc\\\"def\"",
               |p| p.oneof(),
               vec![Token::new(Rule::oneof, 0, 17),
                    Token::new(Rule::string_literal, 7, 17)]);

    test_parse("letter from a to k",
               |p| p.letter(),
               vec![Token::new(Rule::letter, 0, 18),
                    Token::new(Rule::lower_alpha, 12, 13),
                    Token::new(Rule::lower_alpha, 17, 18)]);

    test_parse("letter",
               |p| p.letter(),
               vec![Token::new(Rule::letter, 0, 6)]);

    test_parse("uppercase letter from A to L",
               |p| p.upperletter(),
               vec![Token::new(Rule::upperletter, 0, 28),
                    Token::new(Rule::upper_alpha, 22, 23),
                    Token::new(Rule::upper_alpha, 27, 28)]);

    test_parse("uppercase letter",
               |p| p.upperletter(),
               vec![Token::new(Rule::upperletter, 0, 16)]);

    test_parse("any character",
               |p| p.anycharacter(),
               vec![Token::new(Rule::anycharacter, 0, 13)]);

    test_parse("no character",
               |p| p.nocharacter(),
               vec![Token::new(Rule::nocharacter, 0, 12)]);

    test_parse("digit from 5 to 9",
               |p| p.chardigit(),
               vec![Token::new(Rule::chardigit, 0, 17),
                    Token::new(Rule::single_digit, 11, 12),
                    Token::new(Rule::single_digit, 16, 17)]);

    test_parse("digit",
               |p| p.chardigit(),
               vec![Token::new(Rule::chardigit, 0, 5)]);

    test_parse("anything",
               |p| p.anything(),
               vec![Token::new(Rule::anything, 0, 8)]);

    test_parse("new line",
               |p| p.newline(),
               vec![Token::new(Rule::newline, 0, 8)]);

    test_parse("whitespace",
               |p| p.space(),
               vec![Token::new(Rule::space, 0, 10)]);

    test_parse("tab",
               |p| p.tab(),
               vec![Token::new(Rule::tab, 0, 3)]);

    test_parse("no whitespace",
               |p| p.nospace(),
               vec![Token::new(Rule::nospace, 0, 13)]);

    test_parse("raw \"abc\\\"def\"",
               |p| p.raw(),
               vec![Token::new(Rule::raw, 0, 14),
                    Token::new(Rule::string_literal, 4, 14)]);

    test_parse("(literally \"abcdef\")",
               |p| p.character(),
               vec![Token::new(Rule::character, 0, 20),
                    Token::new(Rule::group, 0, 20),
                    Token::new(Rule::character, 1, 19),
                    Token::new(Rule::literally, 1, 19),
                    Token::new(Rule::string_literal, 11, 19)]);

    test_parse("capture (literally \"abcdef\") as \"name\"",
               |p| p.character(),
               vec![Token::new(Rule::character, 0, 38),
                    Token::new(Rule::capture, 0, 38),
                    Token::new(Rule::group, 8, 28),
                    Token::new(Rule::character, 9, 27),
                    Token::new(Rule::literally, 9, 27),
                    Token::new(Rule::string_literal, 19, 27),
                    Token::new(Rule::string_literal, 32, 38)]);
}

#[test]
fn test_group() {
    test_parse("\"abcdef\"",
               |p| p.group(),
               vec![Token::new(Rule::group, 0, 8),
                    Token::new(Rule::string_literal, 0, 8)]);

    test_parse("(literally \"abcdef\")",
               |p| p.group(),
               vec![Token::new(Rule::group, 0, 20),
                    Token::new(Rule::character, 1, 19),
                    Token::new(Rule::literally, 1, 19),
                    Token::new(Rule::string_literal, 11, 19)]);

}
