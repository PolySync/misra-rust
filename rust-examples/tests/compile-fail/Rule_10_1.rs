#![deny(clippy)]
#[deny(warnings)]

fn main() {
    let x: i32 = 0xFF;
    let _ = x << 2;
}