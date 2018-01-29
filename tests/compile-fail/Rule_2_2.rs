#![deny(clippy)]
#[deny(warnings)]

fn main() {
    let x: u16; //~ ERROR variable `x` is assigned to, but never used
    x = 3; //~ ERROR value assigned to `x` is never read
}
