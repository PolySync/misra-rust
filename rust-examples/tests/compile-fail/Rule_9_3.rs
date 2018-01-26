#![deny(clippy)]
#[deny(warnings)]

fn main() {
    let x: [i32; 3] = [ 0, 1 ];
    println!("{:?}", x);
}
