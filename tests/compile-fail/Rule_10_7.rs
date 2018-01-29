#![deny(clippy)]
#[deny(warnings)]

fn main() {
    let u16a: u16 = 1;
    let u16b: u16 = 2;
    let u32a: u32 = 3;
    let _ = u32a * (u16a + u16b); //~ ERROR mismatched types
    //~^ ERROR the trait bound `u32: std::ops::Mul<u16>` is not satisfied
}
