#![deny(clippy)]
#[deny(warnings)]

fn main() {
    let u16a: u16 = 1;
    let u16b: u16 = 2;
    let _: u32 = u16a + u16b;
    //~^ ERROR mismatched types
}
