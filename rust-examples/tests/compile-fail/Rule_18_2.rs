#![deny(clippy)]
#[deny(warnings)]

fn main() {
    let a1: [i32; 10] = [0;10];
    let a2: [i32; 10] = [0;10];

    let diff = &a1 - &a2;

    println!("subtract references: {}", diff);
}
