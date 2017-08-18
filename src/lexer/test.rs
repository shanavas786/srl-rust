use super::*;

#[test]
fn test_next_identifier() {
    let mut lx = Lexer::new("bEgin with capture (letter) twice");
    let token1 = lx.next_identifier().unwrap();
    assert_eq!(token1.val(), "begin with");
    assert!(match token1.token_type() {
        TokenType::BeginWith => true,
        _ => false,
    });

    let token2 = lx.next_identifier().unwrap();
    assert_eq!(token2.val(), "capture");
    assert!(match token2.token_type() {
        TokenType::Capture => true,
        _ => false,
    });

    let token3 = lx.next_identifier().unwrap();
    assert_eq!(token3.val(), "(");
    assert!(match token3.token_type() {
        TokenType::GroupStart => true,
        _ => false,
    });

    let token4 = lx.next_identifier().unwrap();
    assert_eq!(token4.val(), "letter");
    assert!(match token4.token_type() {
        TokenType::Letter => true,
        _ => false,
    });

    let token5 = lx.next_identifier().unwrap();
    assert_eq!(token5.val(), ")");
    assert!(match token5.token_type() {
        TokenType::GroupEnd => true,
        _ => false,
    });

    let token6 = lx.next_identifier().unwrap();
    assert_eq!(token6.val(), "twice");
    assert!(match token6.token_type() {
        TokenType::Twice => true,
        _ => false,
    });

    let token7 = lx.next_identifier().unwrap();
    assert_eq!(token7.val(), "eof");
    assert!(match token7.token_type() {
        TokenType::EndOfFile => true,
        _ => false,
    });
}

#[test]
fn test_next_string() {
    let mut lx = Lexer::new("\"first string\" 'second' \"esca\\\"ped1\" 'escaped\\'2'");
    let token1 = lx.next_string().unwrap();
    assert_eq!(token1.val(), "first string");
    assert!(match token1.token_type() {
        TokenType::String => true,
        _ => false,
    });
    let token2 = lx.next_string().unwrap();
    assert_eq!(token2.val(), "second");

    let token3 = lx.next_string().unwrap();
    assert_eq!(token3.val(), "esca\"ped1");

    let token4 = lx.next_string().unwrap();
    assert_eq!(token4.val(), "escaped'2");

    lx.next_string();
    assert!(lx.is_error());

    let mut lx2 = Lexer::new("\"unterminated ");
    lx2.next_string();
    assert!(lx2.is_error());
}

#[test]
fn test_next_number() {
    let mut lx = Lexer::new("112 28, 282");
    let token1 = lx.next_number().unwrap();
    assert_eq!(token1.val(), "112");
    assert!(match token1.token_type() {
        TokenType::Number => true,
        _ => false,
    });

    let token2 = lx.next_number().unwrap();
    assert_eq!(token2.val(), "28");

    let token3 = lx.next_number().unwrap();
    assert_eq!(token3.val(), "282");

    let mut lx2 = Lexer::new("112 28b 282");
    lx2.next_number();
    lx2.next_number();
    assert!(lx2.is_error());
}

#[test]
fn test_next_char_or_digit() {
    let mut lx = Lexer::new("from a to c");
    lx.next_identifier();

    let token1 = lx.next_char_or_digit().unwrap();
    assert_eq!(token1.val(), "a");
    assert!(match token1.token_type() {
        TokenType::Character => true,
        _ => false,
    });

    lx.next_identifier();
    let token2 = lx.next_char_or_digit().unwrap();
    assert_eq!(token2.val(), "c");
    assert!(match token2.token_type() {
        TokenType::Character => true,
        _ => false,
    });

    let mut lx = Lexer::new("from 1 to 5");

    lx.next_identifier();
    let token1 = lx.next_char_or_digit().unwrap();
    assert_eq!(token1.val(), "1");
    assert!(match token1.token_type() {
        TokenType::Digit => true,
        _ => false,
    });

    lx.next_identifier();
    let token2 = lx.next_char_or_digit().unwrap();
    assert_eq!(token2.val(), "5");
    assert!(match token2.token_type() {
        TokenType::Digit => true,
        _ => false,
    });
}

#[test]
fn test_next_token() {
    let mut lx = Lexer::new("capture \"test\" capture(letter from a to k twice)");

    lx.next();

    let tk1 = lx.next_token().unwrap();
    assert_eq!(tk1.val(), "test");
    assert!(match tk1.token_type() {
        TokenType::String => true,
        _ => false,
    });

    lx.next();

    let tk2 = lx.next_token().unwrap();
    assert_eq!(tk2.val(), "(");
    assert!(match tk2.token_type() {
        TokenType::GroupStart => true,
        _ => false,
    });

}

#[test]
fn test_lexer_iter() {
    let mut lx = Lexer::new("bEgin with capture \"test\" capture(letter from a to k twice)");
    let tk1 = lx.next().unwrap();
    assert_eq!(tk1.val(), "begin with");
    assert!(match tk1.token_type() {
        TokenType::BeginWith => true,
        _ => false,
    });

    let tk2 = lx.next().unwrap();
    assert_eq!(tk2.val(), "capture");
    assert!(match tk2.token_type() {
        TokenType::Capture => true,
        _ => false,
    });

    let tk3 = lx.next().unwrap();
    assert_eq!(tk3.val(), "test");
    assert!(match tk3.token_type() {
        TokenType::String => true,
        _ => false,
    });

    let tk4 = lx.next().unwrap();
    assert_eq!(tk4.val(), "capture");
    assert!(match tk2.token_type() {
        TokenType::Capture => true,
        _ => false,
    });

    let tk5 = lx.next().unwrap();
    assert_eq!(tk5.val(), "(");
    assert!(match tk5.token_type() {
        TokenType::GroupStart => true,
        _ => false,
    });

    let tk6 = lx.next().unwrap();
    assert_eq!(tk6.val(), "letter");
    assert!(match tk6.token_type() {
        TokenType::Letter => true,
        _ => false,
    });

    let tk7 = lx.next().unwrap();
    assert_eq!(tk7.val(), "from");
    assert!(match tk7.token_type() {
        TokenType::From => true,
        _ => false,
    });

    let tk8 = lx.next().unwrap();
    assert_eq!(tk8.val(), "a");
    assert!(match tk8.token_type() {
        TokenType::Character => true,
        _ => false,
    });

    let tk9 = lx.next().unwrap();
    assert_eq!(tk9.val(), "to");
    assert!(match tk9.token_type() {
        TokenType::To => true,
        _ => false,
    });

    let tk10 = lx.next().unwrap();
    assert_eq!(tk10.val(), "k");
    assert!(match tk10.token_type() {
        TokenType::Character => true,
        _ => false,
    });

    let tk11 = lx.next().unwrap();
    assert_eq!(tk11.val(), "twice");
    assert!(match tk11.token_type() {
        TokenType::Twice => true,
        _ => false,
    });

    let tk12 = lx.next().unwrap();
    assert_eq!(tk12.val(), ")");
    assert!(match tk12.token_type() {
        TokenType::GroupEnd => true,
        _ => false,
    });
}
