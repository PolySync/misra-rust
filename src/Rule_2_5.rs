// Enforce MISRA-C Rule 2.5 at compile time
#[deny(unused_macros)]

macro_rules! data {
   () => (3;);
}

fn main() {
    println!("This program contains an unused macro declaration.");
}
