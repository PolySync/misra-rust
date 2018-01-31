#[forbid(clippy_pedantic)]
#[forbid(clippy)]
#[forbid(warnings)]

/// This function adds one to the input
fn add_one(x: i32) -> i32 {
    x + 1
}

fn main() {
    let _: u32 = add_one as u32;
}
