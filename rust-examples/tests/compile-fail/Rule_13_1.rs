#![deny(clippy)]
#[deny(warnings)]

fn main() {
    let mut x: u8 = 0;
    let a: [&u8; 2] = [ &mut x, &mut x];
    println!("{:?}", a);
}
