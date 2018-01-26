#![deny(clippy)]
#[deny(warnings)]

macro_rules! three {
   () => 3;
}

fn main() {
    println!("{}", three!());
}