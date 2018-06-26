fn main() {
    let x: u32 = 2;
    let y: u16 = 4;
    let z = x + y; //~ ERROR mismatched types
    //~^ ERROR cannot add `u16` to `u32`
}
