extern crate srl;

fn main() {
    let mut a = srl::Lexer::new("exactly literally 'a'");
    println!("{:?}", a.next());
    println!("{:?}", a.next());
    println!("{:?}", a.next());
}
