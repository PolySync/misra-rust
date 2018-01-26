#![deny(clippy)]
#[deny(warnings)]

static arr: [i32] = [];

fn main() {
    println!("{}", arr);
}
