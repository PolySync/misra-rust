fn main() {
    let mut f: f64 = 0_f64;

    while f < 1.0 {
        f += 0.001_f64;
        //^ ERROR Non-compliant - loop counter with floating point type
    }
}
