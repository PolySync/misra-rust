fn main() {
    let u16_a: u16 = 1;
    let u16_b: u16 = 2;
    let u32_c: u32 = 3;
    let _ = u32_c *  //~ ERROR cannot multiply `u32` by `u16` [E0277]
				(u16_a + u16_b);  //~ ERROR mismatched types [E0308]
}
