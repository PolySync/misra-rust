#[allow(dead_code, unused_variables)]

const ABC: i32 = 0;

fn main() {
    let abc: i32 = 1;
    //~^ ERROR Non-compliant - variable name shadows ABC
    let _ = abc + ABC;
}
