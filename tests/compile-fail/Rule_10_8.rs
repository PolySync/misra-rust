fn main() {
    let u16_a: u16 = 1;
    let u16_b: u16 = 2;
    let _ = (u16_a + u16_b) as u32;
    //~^ ERROR casting u16 to u32 may become silently lossy if types change
}
