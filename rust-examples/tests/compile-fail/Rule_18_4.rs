#![deny(clippy)]
#[deny(warnings)]

fn main() {
    let a: [u8; 10] = [0;10];
    let ptr = &a[5];

    (ptr + 5) = 0;
}
