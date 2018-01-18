// Enforce MISRA-C Rule 2.3 at compile time
#[deny(dead_code)]

fn main() {
    println!("This program contains an unused type declaration.");
    type LocalType = i16;
}
