#[forbid(clippy_pedantic)]
#[forbid(clippy)]
#[forbid(warnings)]

fn main() {
    let u16_a: u16 = 1;
    let u16_b: u16 = 2;
    let _: u32 = u16_a + u16_b;
    //~^ ERROR mismatched types
}
