fn main() {
    let x = 1;
    if x == 2 {
        let _ = 1;
    } else if x == 3 {
        let _ = 2;
    }
    //~^ ERROR Non-compliant - no terminating `else`
}
