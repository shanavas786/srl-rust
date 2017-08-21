extern crate srl;

use srl::srl;

fn main() {
    let lx = srl("bEgin with capture \"test\" capture(letter from a to k twice)");
    println!("{}", lx);
}
