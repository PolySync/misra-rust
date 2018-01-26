#![deny(clippy)]
#[deny(warnings)]

macro_rules! while {
   () => (3;);
}

fn main() {
    println!("{}", while!());
}
