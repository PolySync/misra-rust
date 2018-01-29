#![deny(clippy)]
#[deny(warnings)]

macro_rules! println {
   () => (3;);
}

fn main() {
    println!();
}
