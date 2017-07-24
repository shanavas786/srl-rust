extern crate srl;

use srl::lexer::Lexer;

#[test]
fn test_tokens() {
    let mut lx = Lexer::new("exactly twice");
    println!("{:?}", lx.next().unwrap());
    println!("{:?}", lx.next().unwrap());
}
