fn main() {
    let u8a: u8 = 1;
    let _: u8 = u8a << 8; //~ ERROR this arithmetic operation will overflow [arithmetic_overflow]
}