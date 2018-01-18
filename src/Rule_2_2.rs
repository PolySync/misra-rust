// Enforce MISRA-C Rule 2.2 at compile time
#[deny(unused_variables)]
#[warn(unused_assignments)]

fn main() {
    println!("This program contains dead code.");
    let x: u16;
    x = 3;
}
