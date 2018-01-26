#![deny(clippy)]
#[deny(warnings)]

#[derive(Debug)]
struct Once {
    a: i32,
}

fn main() {
    let once = Once { a: 1, a: 2 };
    println!("{:?}", once);
}
