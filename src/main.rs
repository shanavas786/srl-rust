extern crate srl;

use srl::SRL;

fn main() {
    let lx = SRL::new("bEgin with capture \"test\" capture(letter from a to k twice)");
    println!("{}", lx);
}
