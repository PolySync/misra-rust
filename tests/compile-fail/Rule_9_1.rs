#[allow(unused_variables)]

fn main() {
    let x: u16;
    let y = x + 1;
    //~^ ERROR use of possibly uninitialized variable: `x`
}
