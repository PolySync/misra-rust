#![deny(clippy)]
#[deny(warnings)]

fn main() {
    let x: u32 = 2;
    let y: u16 = 4;
    let _ = x + y; //~ ERROR mismatched types
    //~^ ERROR the trait bound `u32: std::ops::Add<u16>` is not satisfied
}
