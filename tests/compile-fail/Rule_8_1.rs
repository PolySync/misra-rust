#[allow(unused_assignments, unused_variables)]

fn main() {
    let x;
    //~^ ERROR Non-compliant - type not explicitly specified
    x = 1;
}
