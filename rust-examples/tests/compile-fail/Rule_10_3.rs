#![deny(clippy)]
#[deny(warnings)]

fn main() {
    let x: u32 = 2;
    let y: u16 = x;
    println!("{}, {}", x, y);
}
