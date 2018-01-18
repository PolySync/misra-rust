fn main() {
    let u8a: u8 = 0;
    let _x = u8a - 10;
    //~^ERROR Non-compliant - attempt to subtract with overflow
}
