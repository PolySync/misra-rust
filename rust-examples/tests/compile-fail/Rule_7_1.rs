#![deny(clippy)]
#[deny(warnings)]

fn main() {
    let count: i32 = 0o52;
    println!("{}", count);
}
