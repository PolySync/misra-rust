fn main() {
    let x = 1;

    if x > 1 {
        return;
    }

    if x < 1 {
        return;
        //~^ ERROR Non-compliant - more than one exit point from function
    }
}
