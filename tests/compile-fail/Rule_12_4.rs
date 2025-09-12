fn main() {
    let u8a: u8 = 0;
    let _x = u8a - 10; //~ERROR this arithmetic operation will overflow [arithmetic_overflow]
}
