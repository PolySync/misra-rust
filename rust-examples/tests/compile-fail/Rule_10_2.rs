#![deny(clippy)]
#[deny(warnings)]

fn main() {
    let x: i16 = -2;
    println!("{}", x - 'a');
}
