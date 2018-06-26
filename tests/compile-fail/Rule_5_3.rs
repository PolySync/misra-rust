fn main() {
    let i: i16 = 1;
    if true {
        let i: i16 = 0; //~ ERROR Non-compliant - `i` shadows outer scope.
        let _ = i;
    }
    let _ = i;
}
