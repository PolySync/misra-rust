#![deny(clippy)]
#[deny(warnings)]

use std::fs::File;

fn main() {
    let f2 = File::open("open.txt").unwrap();
    println!("{}", *f2);
}