#[allow(unused_variables)]

fn main() {
    let x: u16;
    let y = x + 1; //~ ERROR  used binding `x` isn't initialized [E0381]
}
