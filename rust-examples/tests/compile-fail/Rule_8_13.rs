#![deny(clippy)]
#[deny(warnings)]

fn main() {
    let mut x: Box<u8> = Box::new(8);

    *x += 1;
}
