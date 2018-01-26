#![deny(clippy)]
#[deny(warnings)]

fn main() {
    let a = 1;
    *(&mut a) = 2;
    println!("{}", a);
}
