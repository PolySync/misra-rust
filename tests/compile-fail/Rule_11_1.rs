/// This function adds one to the input
fn add_one(x: i32) -> i32 {
    x + 1
}

fn main() {
    let _: u32 = 1 as add_one;
    // ERROR~^ expected type, found function `add_one`
}
