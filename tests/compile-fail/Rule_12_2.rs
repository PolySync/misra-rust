fn main() {
    let u8a: u8 = 1;
    let _: u8 = u8a << 8;
    //~^ ERROR attempt to shift left with overflow
}
