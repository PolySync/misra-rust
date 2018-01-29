#![deny(clippy)]
#[deny(warnings)]

fn main() {
    let i: i16 = 1;
    if true {
        let i: i16 = 0;
        println!("{}", i);
    }
    println!("{}", i);
}
