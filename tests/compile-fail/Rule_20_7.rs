#![deny(clippy)]
#[deny(warnings)]

macro_rules! three {
   () => 3; //~ ERROR macro rhs must be delimited
}

fn main() {
    println!("{}", three!());
}
