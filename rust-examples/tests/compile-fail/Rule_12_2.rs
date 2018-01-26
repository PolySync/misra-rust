#![deny(clippy)]
#[deny(warnings)]

fn main() {
    let u8a: u8 = 1;
    let u8b: u8 = u8a << 8;
    println!("{}", u8b);
}
