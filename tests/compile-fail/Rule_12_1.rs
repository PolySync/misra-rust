fn main() {
    let x: usize = 1;
    if x >= 2 << 2 + 1 as usize {
        //~^ ERROR Non-compliant - operator precedence can trip the unwary
    }
}
