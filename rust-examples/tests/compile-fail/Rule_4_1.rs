#![deny(clippy)]
#[deny(warnings)]

fn main() {
    let s1 = "\x41g";
    println!("s1: {}", s1); // "Ag"
}
