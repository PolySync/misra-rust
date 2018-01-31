#[forbid(clippy_pedantic)]
#[forbid(clippy)]
#[forbid(warnings)]

fn main() {
    let u16_a: u16 = 1;
    let u16_b: u16 = 2;
    let u32_c: u32 = 3;
    let _ = u32_c * (u16_a + u16_b); //~ ERROR mismatched types
    //~^ ERROR the trait bound `u32: std::ops::Mul<u16>` is not satisfied
}
