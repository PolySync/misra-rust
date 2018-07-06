const GLOBAL: u32 = 0;
fn main() {
    let _x = GLOBAL + 1;
    //~^ ERROR Non-compliant - global only used at block scope
}
