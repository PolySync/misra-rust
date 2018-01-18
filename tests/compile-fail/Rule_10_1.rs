#[allow(unused_variables)]

fn main() {
    let x: i32 = 0xFF;
    let y = x << 2;
    //~^ ERROR Non-compliant - inappropriate essential type
}
