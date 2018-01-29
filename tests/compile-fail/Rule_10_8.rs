#![deny(clippy)]
#[deny(warnings)]

fn main() {
    let u16a: u16 = 1;
    let u16b: u16 = 2;
    let _ = (u16a + u16b) as u32;
    //~^ ERROR casting u16 to u32 may become silently lossy if types change
}
